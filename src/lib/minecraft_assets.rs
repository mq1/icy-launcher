// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use url::Url;

use crate::{AppState, View};

use super::{BASE_DIR, HTTP_CLIENT};

const ASSETS_DOWNLOAD_ENDPOINT: Lazy<Url> =
    Lazy::new(|| Url::parse("https://resources.download.minecraft.net").unwrap());

pub static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("assets"));
static INDEXES_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("indexes"));
static OBJECTS_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("objects"));

#[derive(Deserialize)]
pub struct AssetIndexInfo {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    #[serde(rename = "totalSize")]
    pub total_size: Option<usize>,
    pub url: Url,
}

impl AssetIndexInfo {
    fn get_path(&self) -> PathBuf {
        INDEXES_DIR.join(format!("{}.json", &self.id))
    }

    fn download(&self) -> Result<()> {
        let path = self.get_path();
        let url = &self.url.to_string();

        fs::create_dir_all(path.parent().ok_or(eyre!("Invalid path"))?)?;
        let resp = HTTP_CLIENT.get(url).call()?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        io::copy(&mut resp.into_reader(), &mut writer)?;

        Ok(())
    }

    fn check_hash(&self) -> Result<bool> {
        let path = self.get_path();
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == self.sha1)
    }

    pub fn get(&self, event_sink: &druid::ExtEventSink) -> Result<AssetIndex> {
        let path = self.get_path();

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Loading;
            data.current_message = "Downloading asset index...".to_string();
        });

        if path.exists() && !self.check_hash()? {
            fs::remove_file(&path)?;
        }

        if !path.exists() {
            self.download()?;
        }

        if !self.check_hash()? {
            bail!("Asset index hash mismatch");
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let index = serde_json::from_reader(reader)?;

        Ok(index)
    }
}

#[derive(Deserialize)]
struct Object {
    pub hash: String,
}

impl Object {
    pub fn get_path(&self) -> PathBuf {
        OBJECTS_DIR.join(&self.hash[..2]).join(&self.hash)
    }

    fn get_url(&self) -> Result<Url, url::ParseError> {
        ASSETS_DOWNLOAD_ENDPOINT.join(&format!("{}/{}", &self.hash[..2], &self.hash))
    }

    pub fn download(&self) -> Result<()> {
        let path = self.get_path();
        let url = &self.get_url()?.to_string();

        fs::create_dir_all(path.parent().ok_or(eyre!("Invalid path"))?)?;
        let resp = HTTP_CLIENT.get(url).call()?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        io::copy(&mut resp.into_reader(), &mut writer)?;

        Ok(())
    }

    pub fn check_hash(&self) -> Result<bool> {
        let path = self.get_path();

        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == self.hash)
    }
}

#[derive(Deserialize)]
pub struct AssetIndex {
    #[serde(rename = "objects")]
    objects: HashMap<String, Object>,
}

impl AssetIndex {
    pub fn download_objects(&self, event_sink: &druid::ExtEventSink) -> Result<()> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Progress;
            data.current_message = "Downloading assets...".to_string();
            data.current_progress = 0.;
        });

        let mut downloaded_objects = 0.;
        let object_count = self.objects.len() as f64;

        for object in self.objects.values() {
            let path = object.get_path();

            if path.exists() && !object.check_hash()? {
                fs::remove_file(&path)?;
            }

            if !path.exists() {
                object.download()?;
            }

            if !object.check_hash()? {
                bail!("Failed to download object");
            }

            downloaded_objects += 1.;
            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_objects / object_count;
            });
        }

        Ok(())
    }
}

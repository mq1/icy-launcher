// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use attohttpc::Session;
use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod launcher_updater;
mod minecraft_assets;
mod minecraft_libraries;
pub mod minecraft_news;
mod minecraft_rules;
pub mod minecraft_version_manifest;
pub mod minecraft_version_meta;
pub mod msa;
pub mod runtime_manager;
pub mod modrinth;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "ice-launcher")
        .expect("Could not get project directories")
        .data_dir()
        .to_path_buf()
});

pub static HTTP_CLIENT: Lazy<Session> = Lazy::new(|| {
    let mut sess = Session::new();
    sess.header(
        "User-Agent",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    );

    sess
});

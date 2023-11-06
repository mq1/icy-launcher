// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use eframe::egui;

const LOGO: egui::ImageSource = egui::include_image!("../../assets/logo-128x128.png");

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let logo = egui::Image::new(LOGO).fit_to_exact_size(egui::vec2(128., 128.));
        ui.add(logo);

        ui.heading("CrabLauncher");

        ui.separator();

        ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));

        ui.add_space(8.);

        ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

        ui.add_space(8.);

        ui.label(format!("Authors: {}", env!("CARGO_PKG_AUTHORS")));

        ui.add_space(8.);

        ui.horizontal(|ui| {
            ui.label("Repository:".to_string());
            ui.hyperlink(env!("CARGO_PKG_REPOSITORY"));
        });
    });
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{
        button, column, container, horizontal_space, row, text, text_input, toggler, vertical_space,
    },
    Element, Length,
};

use crate::{lib, style, Message};

pub fn view(config: &Result<lib::launcher_config::LauncherConfig>) -> Element<Message> {
    let heading = text("Settings").size(50);

    let settings: Element<_> = match config {
        Ok(config) => column![
            container(toggler(
                "Automatically check for updates".to_string(),
                config.automatically_check_for_updates,
                Message::UpdatesTogglerChanged,
            ))
            .padding(10)
            .style(style::card()),
            container(toggler(
                "Automatically update JVM".to_string(),
                config.automatically_update_jvm,
                Message::UpdateJvmTogglerChanged,
            ))
            .padding(10)
            .style(style::card()),
            container(toggler(
                "Automatically optimize JVM".to_string(),
                config.automatically_optimize_jvm_arguments,
                Message::OptimizeJvmTogglerChanged,
            ))
            .padding(10)
            .style(style::card()),
            container(row![
                text("JVM memory"),
                horizontal_space(Length::Fill),
                text_input("JVM memory", &config.jvm_memory, Message::UpdateJvmMemory),
            ])
            .padding(10)
            .style(style::card()),
        ]
        .spacing(10)
        .into(),
        Err(_) => text("Failed to load settings").into(),
    };

    let footer = row![
        horizontal_space(Length::Fill),
        button("Reset to default settings").on_press(Message::ResetConfig),
        button("Save settings").on_press(Message::SaveConfig),
    ]
    .spacing(10);

    column![heading, settings, vertical_space(Length::Fill), footer]
        .spacing(20)
        .padding(20)
        .into()
}

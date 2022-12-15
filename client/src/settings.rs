// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, text, text_input, toggler,
        vertical_space, Column,
    },
    Element, Length,
};
use mclib::launcher_config::LauncherConfig;

use crate::{icons, style, Message};

pub fn view(config: &Result<LauncherConfig>) -> Element<Message> {
    let heading = row![icons::settings().size(50), text("Settings").size(50)].spacing(5);

    let settings: Element<_> = match config {
        Ok(config) => {
            let mut settings = Column::new().spacing(10);

            if cfg!(feature = "check-for-updates") {
                settings = settings.push(
                    container(toggler(
                        "Automatically check for updates".to_string(),
                        config.automatically_check_for_updates,
                        Message::UpdatesTogglerChanged,
                    ))
                    .padding(10)
                    .style(style::card()),
                );
            }

            settings = settings.push(
                container(toggler(
                    "Automatically update JVM".to_string(),
                    config.automatically_update_jvm,
                    Message::UpdateJvmTogglerChanged,
                ))
                .padding(10)
                .style(style::card()),
            );

            settings = settings.push(
                container(toggler(
                    "Automatically optimize JVM".to_string(),
                    config.automatically_optimize_jvm_arguments,
                    Message::OptimizeJvmTogglerChanged,
                ))
                .padding(10)
                .style(style::card()),
            );

            settings = settings.push(
                container(row![
                    text("JVM memory"),
                    horizontal_space(Length::Fill),
                    text_input("JVM memory", &config.jvm_memory, Message::UpdateJvmMemory),
                ])
                .padding(10)
                .style(style::card()),
            );

            settings.into()
        }
        Err(_) => text("Failed to load settings").into(),
    };

    let footer = row![
        horizontal_space(Length::Fill),
        button("Reset to default settings")
            .on_press(Message::ResetConfig)
            .style(theme::Button::Secondary),
        button("Save settings")
            .on_press(Message::SaveConfig)
            .style(theme::Button::Positive),
    ]
    .spacing(10);

    column![heading, settings, vertical_space(Length::Fill), footer]
        .spacing(20)
        .padding(20)
        .into()
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element,
    Length, theme, widget::{button, Column, horizontal_space, row, Row, text, vertical_space},
};

use crate::{
    APP_NAME,
    components::{assets, icons},
    Message, pages::Page, style,
};

const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub struct About;

impl Page for About {
    type Message = Message;

    fn update(&mut self, _message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<'static, Message> {
        let logo = icons::view_png(assets::LOGO_PNG, 128);

        let repo_button = button(
            row![" Repository ", icons::view(icons::GITHUB)]
                .align_items(Alignment::Center)
                .padding(5),
        )
            .style(style::circle_button(theme::Button::Primary))
            .on_press(Message::OpenURL(REPOSITORY.to_string()));

        let footer = Row::new()
            .push(horizontal_space(Length::Fill))
            .push(repo_button);

        Column::new()
            .push(vertical_space(Length::Fill))
            .push(logo)
            .push(text(APP_NAME).size(50))
            .push(text(APP_VERSION))
            .push(text(LICENSE))
            .push(text(COPYRIGHT))
            .push(vertical_space(Length::Fill))
            .push(footer)
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .into()
    }
}

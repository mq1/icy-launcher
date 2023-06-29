// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, scrollable, text, Image},
    Alignment, Command, Element, Length,
};
use iced_aw::Wrap;

use crate::{components::assets, pages::Page, style, util::instances::Instances, Message};

impl Page for Instances {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut wrap = Wrap::new();
        for instance in &self.list {
            let logo_handle = image::Handle::from_memory(assets::LOGO_PNG);
            let logo = Image::new(logo_handle).height(100);

            let c = container(
                column![
                    logo,
                    text(instance.to_owned()).size(20),
                    button("Edit").style(style::circle_button()),
                    button("Launch").style(style::circle_button()),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10),
            )
            .style(style::card());
            wrap = wrap.push(container(c).padding(5));
        }

        let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

        column![text("Instances").size(30), content]
            .spacing(10)
            .padding(10)
            .into()
    }
}

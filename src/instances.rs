// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{widget::text, Element};

use crate::Message;

pub struct InstancesView;

impl InstancesView {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<Message> {
        text("Instances").into()
    }
}

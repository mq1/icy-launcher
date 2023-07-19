// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element,
    Length, theme, widget::{button, Column, container, image, Image, tooltip, vertical_space},
};
use iced_aw::Spinner;

use crate::{APP_NAME, components::icons, Message, style, util, View};

pub fn view<'a>(
    current_view: &'a View,
    active_account: &'a Option<util::accounts::Account>,
    latest_instance: Option<util::instances::Instance>,
) -> Element<'a, Message> {
    let change_view_button =
        |view: View, icon: Element<'static, Message>, tooltip_text| -> Element<Message> {
            tooltip(
                button(icon)
                    .padding(10)
                    .style(if &view == current_view {
                        style::selected_button()
                    } else {
                        theme::Button::Text
                    })
                    .on_press(Message::ChangeView(view)),
                tooltip_text,
                tooltip::Position::Right,
            )
                .gap(10)
                .style(theme::Container::Box)
                .into()
        };

    let account_icon = if let Some(account) = active_account {
        if let Some(cached_head) = &account.cached_head {
            let head_handle = image::Handle::from_memory(cached_head.clone());
            let head = Image::new(head_handle).width(32).height(32);

            head.into()
        } else {
            Spinner::new().into()
        }
    } else {
        icons::view_custom(icons::ACCOUNT_ALERT_OUTLINE, 32)
    };

    let col = Column::new()
        .push(change_view_button(
            View::Instance(latest_instance),
            icons::view_custom(icons::PACKAGE_VARIANT, 32),
            "Latest Instance",
        ))
        .push(change_view_button(
            View::NewInstance,
            icons::view_custom(icons::VIEW_GRID_PLUS_OUTLINE, 32),
            "New Instance",
        ))
        .push(change_view_button(
            View::Instances,
            icons::view_custom(icons::VIEW_GRID_OUTLINE, 32),
            "Instances",
        ))
        .push(vertical_space(Length::Fill))
        .push(change_view_button(
            View::Accounts,
            account_icon,
            "Accounts",
        ))
        .push(change_view_button(
            View::Settings,
            icons::view_custom(icons::COG_OUTLINE, 32),
            "Settings",
        ))
        .push(change_view_button(
            View::About,
            icons::view_custom(icons::INFORMATION_OUTLINE, 32),
            format!("About {}", APP_NAME).as_str(),
        ))
        .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

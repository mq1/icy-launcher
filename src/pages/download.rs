// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{progress_bar, text, Column},
    Alignment, Command, Element, Subscription,
};

use crate::{pages::Page, subscriptions::download, util::DownloadItem};

#[derive(Debug, Clone)]
pub enum Message {
    DownloadProgressed(download::Progress),
}

#[derive(Debug)]
pub struct Download {
    state: State,
}

#[derive(Debug)]
enum State {
    Idle,
    Downloading {
        progress: f32,
        items: Vec<DownloadItem>,
    },
    Finished,
    Errored,
}

impl Download {
    pub fn new() -> Self {
        Download { state: State::Idle }
    }

    pub fn start(&mut self, items: Vec<DownloadItem>) {
        match self.state {
            State::Idle { .. } | State::Finished { .. } | State::Errored { .. } => {
                self.state = State::Downloading {
                    progress: 0.0,
                    items,
                };
            }
            _ => {}
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            State::Downloading { progress: _, items } => {
                download::files(items.to_owned()).map(Message::DownloadProgressed)
            }
            _ => Subscription::none(),
        }
    }
}

impl Page for Download {
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DownloadProgressed(new_progress) => {
                if let State::Downloading { progress, items: _ } = &mut self.state {
                    match new_progress {
                        download::Progress::Started => {
                            *progress = 0.0;
                        }
                        download::Progress::Advanced(percentage) => {
                            *progress = percentage;
                        }
                        download::Progress::Finished => {
                            self.state = State::Finished;
                        }
                        download::Progress::Errored => {
                            self.state = State::Errored;
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let current_progress = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress, items: _ } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
        };

        let progress_bar = progress_bar(0.0..=100.0, current_progress);

        let current_progress = format!("Downloading... {current_progress:.2}%");
        let text = text(match &self.state {
            State::Idle => "Starting download",
            State::Finished => "Download finished!",
            State::Downloading { .. } => &current_progress,
            State::Errored => "Something went wrong :(",
        });

        Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(text)
            .push(progress_bar)
            .into()
    }
}

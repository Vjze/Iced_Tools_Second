use iced::widget::{button, container, row, text, Space};
use iced::{window, Command, Font, Length, Renderer};

use super::styles;

#[derive(Clone, Debug)]
pub enum Message {
    Minwindow,
    Maxwindow,
    Closewindow,
    // Theme_Type(TroxideTheme),
}

#[derive(Clone, Debug)]
pub struct Bar {}

impl Bar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let cpmmand = match message {
            Message::Minwindow => window::minimize(true),
            Message::Closewindow => window::close(),
            Message::Maxwindow => window::maximize(true),
        };
        cpmmand
    }

    pub fn view(&self) -> iced::Element<'_, Message, Renderer> {
        let min = button("-")
            .on_press(Message::Minwindow)
            .style(styles::button_styles::transparent_button_theme());
        let max = button("口")
            .on_press(Message::Maxwindow)
            .style(styles::button_styles::transparent_button_theme());
        let close = button("X")
            .on_press(Message::Closewindow)
            .style(styles::button_styles::transparent_button_theme());
        let title = text("Iced_Tools").size(40.0).font(Font::MONOSPACE);

        let bar = container(
            row![
                title,
                Space::new(Length::Fill, Length::Shrink),
                Space::new(Length::Fill, Length::Shrink),
                Space::new(Length::Fill, Length::Shrink),
                min,
                max,
                close,
            ]
            .spacing(5)
            .align_items(iced::Alignment::Start),
        );

        bar.into()
    }
}

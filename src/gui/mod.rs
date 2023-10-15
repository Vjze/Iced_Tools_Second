use self::{
    bar::{Bar, Message as BarMessage},
    left_view::{Leftview, Message as LeftMessage},
};

use crate::gui::styles::container_styles::release_time_container_theme;
use assets::fonts::FONT;
use iced::{widget::container, Application, Command, Length, Renderer, Subscription};
pub mod assets;
pub mod bar;
pub mod foot;
pub mod left_view;
pub mod styles;
pub mod views;
pub struct MyTools {
    bar: Bar,
    leftview: Leftview,
    theme: bool,
    pub now: time::OffsetDateTime,
    pub runtime: f32,
}

#[derive(Clone, Debug)]
pub enum Message {
    BarMessage(BarMessage),
    SwitchChanged(bool),
    Datetime(time::OffsetDateTime),
    FontLoaded(Result<(), iced::font::Error>),
    LeftMessage(LeftMessage),
}

impl Application for MyTools {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = tokio::runtime::Runtime;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let font_command = iced::font::load(FONT);
        (
            Self {
                leftview: Leftview::new(),
                bar: Bar::new(),
                theme: false,
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                runtime: 0.0,
            },
            font_command.map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        format!(
            "{} - V {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::BarMessage(msg) => self.bar.update(msg).map(Message::BarMessage),
            Message::SwitchChanged(b) => {
                self.theme = b;
                Command::none()
            }
            Message::Datetime(local_time) => {
                let now = local_time;
                if now != self.now {
                    self.now = now;
                }
                Command::none()
            }
            Message::FontLoaded(_res) => Command::none(),
            Message::LeftMessage(msg) => {
                self.theme = self.leftview.get_theme();
                self.leftview.update(msg).map(Message::LeftMessage)
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| {
            Message::Datetime(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }

    fn view(&self) -> iced::Element<'_, Message, Renderer> {
        let leftview = self.leftview.view(self).map(Message::LeftMessage);
        container(leftview)
            .padding(5)
            .style(release_time_container_theme())
            .height(Length::Fill)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        let custom_theme = Box::new(
            match self.theme {
                true => styles::theme::TroxideTheme::Dark,
                false => styles::theme::TroxideTheme::Light,
            }
            .get_custom_theme(),
        );
        iced::Theme::Custom(custom_theme)
    }
}

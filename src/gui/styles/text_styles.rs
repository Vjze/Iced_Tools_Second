use super::colors::*;
use iced::theme::Text;

pub fn red_text_theme() -> Text {
    Text::Color(red())
}

pub fn green_text_theme() -> Text {
    Text::Color(green())
}

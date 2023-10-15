#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gui::{assets::pngs::IMG_LOGO, MyTools};
use iced::{
    window::{self, Position},
    Application, Font, Settings,
};
mod gui;
pub mod util;
fn main() -> iced::Result {
    let icon = window::icon::from_file_data(IMG_LOGO, Some(image::ImageFormat::Png)).ok();
    MyTools::run(Settings {
        window: window::Settings {
            icon,
            resizable: false,
            size: (1650, 800),
            decorations: false,
            position: Position::Centered,
            ..window::Settings::default()
        },
        default_font: Font::with_name("Source Han Sans HW SC"),
        ..Default::default()
    })?;

    Ok(())
}

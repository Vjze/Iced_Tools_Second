use iced::{
    widget::{container, row, text, toggler, Space},
    Length, Renderer,
};

use super::{Message, MyTools};

pub fn footview(state: &MyTools) -> iced::Element<'_, Message, Renderer> {
    let datetime = state.now;
    let time = datetime.to_hms();
    let date = datetime.date();
    // let tt = &t[..19];
    let theme = if state.theme {
        String::from("深色模式")
    } else {
        String::from("浅色模式")
    };
    let timetext = text(format!("{:?} {}:{}:{}", date, time.0, time.1, time.2));
    let version = text(format!("当前版本：{}", env!("CARGO_PKG_VERSION")));
    let toggler = toggler(theme, state.theme, Message::SwitchChanged)
        .text_alignment(iced::alignment::Horizontal::Right);
    let row = row![
        timetext,
        Space::new(Length::Fill, Length::Shrink),
        version,
        toggler
    ]
    .width(Length::Fill)
    .height(Length::Fill);
    container(row).into()
}

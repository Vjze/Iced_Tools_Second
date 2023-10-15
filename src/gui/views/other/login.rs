use iced::{
    alignment::{self, Horizontal},
    widget::{button, column, container, row, text, text_input, Container},
    Length,
};
use iced_aw::{modal, Card};

use crate::gui::{MyTools, Message};


pub fn login(state: &MyTools) -> Container<'static, Message> {
    let overlay = if !state.logined {
        Some(
            Card::new(
                text("登录"),
                column![
                    row![
                        text("账号："),
                        text_input("输入账号.....", &state.name).on_input(Message::Namechange)
                    ],
                    row![
                        text("密码："),
                        text_input("输入密码.....", &state.password)
                            .on_input(Message::Passchange)
                            .password()
                    ]
                ],
            )
            .foot(
                row![
                    button(text("取消").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::LoginCence),
                    button(text("登录").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(Message::Logingpre),
                ]
                .spacing(10)
                .padding(5)
                .width(Length::Fill),
            )
            .max_width(300.0)
            //.width(Length::Shrink)
            .on_close(Message::LoginCence),
        )
    } else {
        None
    };
    let c = row![];
    let login = modal(c, overlay)
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center);
    let login_tip = container(login);
    login_tip.into()
}

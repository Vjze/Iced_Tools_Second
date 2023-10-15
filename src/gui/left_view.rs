use iced::widget::image::Handle;
use iced::widget::{button, column, container, row, text, Space};
use iced::widget::{image, toggler};
use iced::{Command, Length, Renderer};

use super::assets::pngs::{IMG_LOGO_DARK, IMG_LOGO_LIGHT};
use super::bar::{Bar, Message as BarMessage};
use super::views::bandin_view::{BandinView, Message as BandinMessage};
use super::views::order_view::{Message as OrderMessage, OrderView};
use super::views::query_view::{Message as QueryMessage, QueryView};
use super::{styles, MyTools};

#[derive(Clone, Debug)]
pub enum Message {
    BarMessage(BarMessage),
    ViewChanged(TabId),
    QueryButton(QueryMessage),
    OrderButoon(OrderMessage),
    BandinButton(BandinMessage),
    ThemeChange(bool),
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Query,
    Bandin,
    Order,
}
#[derive(Clone, Debug)]
pub struct Leftview {
    bar: Bar,
    view: TabId,
    query: QueryView,
    order: OrderView,
    bandin: BandinView,
    theme: bool,
}

impl Leftview {
    pub fn new() -> Self {
        Self {
            bar: Bar::new(),
            view: Default::default(),
            query: QueryView::new(),
            order: Default::default(),
            bandin: Default::default(),
            theme: false,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let cpmmand = match message {
            Message::BarMessage(msg) => self.bar.update(msg).map(Message::BarMessage),
            Message::QueryButton(msg) => {
                self.view = TabId::Query;
                self.query.update(msg).map(Message::QueryButton)
            }
            Message::OrderButoon(msg) => {
                self.view = TabId::Query;
                self.order.update(msg).map(Message::OrderButoon)
            }
            Message::BandinButton(msg) => {
                self.view = TabId::Query;
                self.bandin.update(msg).map(Message::BandinButton)
            }
            Message::ViewChanged(view) => {
                self.view = view;
                Command::none()
            }
            Message::ThemeChange(bool) => {
                self.theme = bool;
                Command::none()
            }
        };
        cpmmand
    }

    pub fn view(&self, state: &MyTools) -> iced::Element<'_, Message, Renderer> {
        let theme_logo = if self.theme {
            IMG_LOGO_DARK
        } else {
            IMG_LOGO_LIGHT
        };
        let theme_text = if self.theme {
            String::from("深色模式")
        } else {
            String::from("浅色模式")
        };
        let logo = image(Handle::from_memory(theme_logo)).width(Length::Fixed(200.0));
        let query_button = button("查询功能合集")
            .on_press(Message::ViewChanged(TabId::Query))
            .style(styles::button_styles::transparent_button_with_rounded_border_theme());
        let bandin_button = button("条码绑定")
            .on_press(Message::ViewChanged(TabId::Bandin))
            .style(styles::button_styles::transparent_button_with_rounded_border_theme());
        let order_button = button("工单打印")
            .on_press(Message::ViewChanged(TabId::Order))
            .style(styles::button_styles::transparent_button_with_rounded_border_theme());
        let datetime = state.now.clone();
        let time = datetime.to_hms();
        let date = datetime.date();
        let timetext = text(format!("{:?} {}:{}:{}", date, time.0, time.1, time.2))
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let version = text(format!("当前版本：{}", env!("CARGO_PKG_VERSION")));
        let toggler = toggler(theme_text, self.theme, Message::ThemeChange)
            .text_alignment(iced::alignment::Horizontal::Center);
        let bar = self.bar.view().map(Message::BarMessage);
        let l = container(logo).align_y(iced::alignment::Vertical::Top);
        let left = container(
            column![
                l,
                Space::new(Length::Shrink, Length::Fill),
                Space::new(Length::Shrink, Length::Fill),
                Space::new(Length::Shrink, Length::Fill),
                query_button,
                bandin_button,
                order_button,
                Space::new(Length::Shrink, Length::Fill),
                Space::new(Length::Shrink, Length::Fill),
                Space::new(Length::Shrink, Length::Fill),
                timetext,
                version,
                toggler,
            ]
            .spacing(5)
            .align_items(iced::Alignment::Center)
            .height(Length::Fill),
        )
        .align_y(iced::alignment::Vertical::Top)
        .width(Length::Fixed(200.0))
        .height(Length::Fill);

        let all_bar = container(left).padding(5);
        let view = match self.view {
            TabId::Query => self.query.view().map(Message::QueryButton),
            TabId::Bandin => self.bandin.view().map(Message::BandinButton),
            TabId::Order => self.order.view().map(Message::OrderButoon),
        };
        let col = row![all_bar, column![bar, view]]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill);
        container(col)
            .padding(5)
            .height(Length::Fill)
            // .style(second_class_container_rounded_theme())
            .into()
    }
    pub fn get_theme(&self) -> bool {
        self.theme
    }
}

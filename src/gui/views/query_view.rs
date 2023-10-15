use super::{
    listview::{box_list_view, sn_list_view},
    BoxDataInfo, Selected, SnDataInfo,
};
use crate::gui::styles::container_styles::second_class_container_rounded_theme;
use crate::{
    gui::styles::{self, button_styles::transparent_button_with_rounded_border_theme},
    util::sql::{box_work_test, sn_work},
};
use iced::{
    alignment::Horizontal,
    widget::{button, checkbox, column, container, pick_list, row, text, text_input},
    Command, Length, Renderer,
};
use iced_aw::date_picker;
use iced_aw::date_picker::Date;
#[derive(Clone, Debug)]
pub enum Message {
    Button,
    Addboxresult(Vec<BoxDataInfo>),
    Addsnresult(Vec<SnDataInfo>),
    Textchange(String),
    Selected(Selected),
    Open1,
    Cancel1,
    Submit1(Date),
    Open2,
    Cancel2,
    Submit2(Date),
    Datequery(bool),
}
#[derive(Clone, Debug)]
pub struct QueryView {
    pub input_value: String,
    pub boxlists: Vec<BoxDataInfo>,
    pub snlists: Vec<SnDataInfo>,
    pub text: String,
    pub selected: Option<Selected>,
    pub quantity: usize,
    pub show_picker_1: bool,
    pub show_picker_2: bool,
    pub datequery: bool,
    pub date1: Date,
    pub date2: Date,
}
impl QueryView {
    pub fn new() -> Self {
        Self {
            input_value: Default::default(),
            text: Default::default(),
            selected: Default::default(),
            quantity: Default::default(),
            show_picker_1: false,
            show_picker_2: false,
            datequery: false,
            date1: Date::today(),
            date2: Date::today(),
            boxlists: vec![],
            snlists: vec![],
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Button => {
                // self.starttime = Instant::now();
                if self.input_value.is_empty() {
                    // self.show_modal = true;
                    // self.tip_type = TipType::EmptyTip;
                    Command::none()
                } else {
                    let s = self.input_value.clone();
                    self.text = s.clone();
                    match self.selected {
                        Some(value) => match value {
                            Selected::Sn => Command::perform(sn_work(s), Message::Addsnresult),
                            Selected::Box => {
                                Command::perform(box_work_test(s), Message::Addboxresult)
                            }
                            Selected::Carton => Command::none(),
                            Selected::Workid => Command::none(),
                        },
                        None => Command::none(),
                    }
                }
            }
            Message::Textchange(value) => {
                self.input_value = value;

                Command::none()
            }
            Message::Addsnresult(value) => {
                self.snlists = value;
                self.quantity = self.snlists.len();
                Command::none()
            }
            Message::Selected(check) => {
                self.selected = Some(check);
                Command::none()
            }
            Message::Addboxresult(value) => {
                self.boxlists = value;
                self.quantity = self.boxlists.len();
                Command::none()
            }
            Message::Open1 => {
                self.show_picker_1 = true;
                Command::none()
            }
            Message::Cancel1 => {
                self.show_picker_1 = false;
                Command::none()
            }
            Message::Submit1(value) => {
                self.date1 = value;
                self.show_picker_1 = false;
                Command::none()
            }
            Message::Datequery(bool) => {
                self.datequery = bool;
                Command::none()
            }
            Message::Open2 => {
                self.show_picker_2 = true;
                Command::none()
            }
            Message::Cancel2 => {
                self.show_picker_2 = false;
                Command::none()
            }
            Message::Submit2(value) => {
                self.date2 = value;
                self.show_picker_2 = false;
                Command::none()
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message, Renderer> {
        let input = text_input("输入要查询的数据...", &self.input_value)
            .on_input(Message::Textchange)
            .on_submit(Message::Button)
            .size(16);

        let select = pick_list(&Selected::ALL[..], self.selected, Message::Selected)
            .placeholder("选择查询项....");
        let check = checkbox("以日期查询", self.datequery, Message::Datequery);
        let pick_date_1 = button(text(format!("{}", self.date1))).on_press(Message::Open1);
        let pick_date_2 = button(text(format!("{}", self.date2))).on_press(Message::Open2);
        let datepicker_1 = date_picker(
            self.show_picker_1,
            self.date1,
            pick_date_1,
            Message::Cancel1,
            Message::Submit1,
        );
        let datepicker_2 = date_picker(
            self.show_picker_2,
            self.date2,
            pick_date_2,
            Message::Cancel2,
            Message::Submit2,
        );
        let text_much = if self.quantity == 0 {
            text(format!("数量:{}", self.quantity))
                .size(18)
                .style(styles::text_styles::red_text_theme())
        } else {
            text(format!("数量:{}", self.quantity))
                .size(18)
                .style(styles::text_styles::green_text_theme())
        };
        let but = button("查 询")
            .on_press(Message::Button)
            .style(transparent_button_with_rounded_border_theme());
        let main_row = row![input, select, check, text_much]
            .spacing(10)
            .padding(5)
            .width(Length::Fill)
            .align_items(Horizontal::Center.into());

        let sele = self.selected;

        let con = match sele {
            Some(value) => match value {
                Selected::Sn => {
                    let list = self.snlists.clone();
                    let l = sn_list_view(list);
                    let col = column![main_row.push(but), l];
                    container(col).width(Length::Fill)
                    // .into()
                }
                Selected::Box => {
                    let dataquery_is_check = match self.datequery {
                        true => container(row![datepicker_1, datepicker_2].spacing(10)),
                        false => container(""),
                    };
                    let list = self.boxlists.clone();
                    let l = box_list_view(list);
                    let col = column![main_row.push(dataquery_is_check).push(but), l];
                    container(col).width(Length::Fill)
                    // .into()
                }
                Selected::Carton => todo!(),
                Selected::Workid => todo!(),
            },
            None => {
                let text = text("本工具由纯Rust语言编写，包括UI（Iced）也是纯Rust！").size(40);
                let col = column![main_row.push(but), text].align_items(Horizontal::Center.into());
                container(col)
                    .center_x()
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
        };
        container(con)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(second_class_container_rounded_theme())
            .into()
    }
}

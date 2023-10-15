use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, scrollable, text, Column, Space},
    Element, Length, Renderer,
};

use crate::gui::styles::{
    container_styles::{loading_container_theme, second_class_container_rounded_theme},
    scrollable_styles::vertical_direction,
};

use super::{query_view::Message, BoxDataInfo, SnDataInfo};

macro_rules! fill {
    () => {
        Space::new(Length::Fill, 0.0)
    };
}
macro_rules! filled { ($($rs:expr),+) => { row![$($rs, fill![]),+] }; }

pub fn t<'a>(s: impl ToString) -> iced::widget::Text<'a> {
    text(s)
}

pub fn box_list_view(os: Vec<BoxDataInfo>) -> iced::Element<'static, Message, Renderer> {
    let header = container(
        filled![
            t("Pack_No").width(Length::Fixed(150.0)),
            t("Sn").width(Length::Fixed(150.0)),
            t("Pn").width(Length::Fixed(100.0)),
            t("WorkOrder").width(Length::Fixed(100.0)),
            t("Creator").width(Length::Fixed(100.0)),
            t("CreateTime").width(Length::Fixed(200.0))
        ]
        .align_items(Horizontal::Center.into())
        .align_items(Vertical::Center.into())
        .width(Length::Fill)
        .padding(10),
    )
    .style(loading_container_theme());

    let rows: Vec<Element<_>> = os
        .iter()
        .map(|b| {
            let pno = t(&b.pno()).width(Length::Fixed(150.0));
            let sn = t(&b.sn()).width(Length::Fixed(150.0));
            let pn = t(&b.pn()).width(Length::Fixed(100.0));
            let workorder = t(b.workorder()).width(Length::Fixed(100.0));
            let creator = t(b.creator()).width(Length::Fixed(100.0));
            let createtime = t(&b.createtime()).width(Length::Fixed(200.0));

            container(
                row!(filled![pno, sn, pn, workorder, creator, createtime].width(Length::Fill),)
                    .padding(5),
            )
            .style(loading_container_theme())
            .into()
        })
        .collect();

    container(column![
        header,
        scrollable(Column::with_children(rows).padding(10)).direction(vertical_direction()),
    ])
    .style(second_class_container_rounded_theme())
    .into()
}

pub fn sn_list_view(os: Vec<SnDataInfo>) -> iced::Element<'static, Message, Renderer> {
    let header = container(
        filled![
            t("Sn").width(Length::Fixed(150.0)),
            t("工号").width(Length::Fixed(50.0)),
            t("温度").width(Length::Fixed(50.0)),
            t("结果").width(Length::Fixed(50.0)),
            t("ith").width(Length::Fixed(50.0)),
            t("pf").width(Length::Fixed(50.0)),
            t("vop").width(Length::Fixed(50.0)),
            t("im").width(Length::Fixed(50.0)),
            t("rs").width(Length::Fixed(50.0)),
            t("sen").width(Length::Fixed(50.0)),
            t("res").width(Length::Fixed(50.0)),
            t("icc").width(Length::Fixed(50.0)),
            t("idark").width(Length::Fixed(50.0)),
            t("vbr").width(Length::Fixed(50.0)),
            t("ixtalk").width(Length::Fixed(50.0)),
            t("kink").width(Length::Fixed(50.0)),
            t("测试时间").width(Length::Fixed(200.0))
        ]
        .align_items(Horizontal::Center.into())
        .align_items(Vertical::Center.into())
        .width(Length::Fill)
        .padding(10),
    )
    .style(loading_container_theme());

    let rows: Vec<Element<_>> = os
        .iter()
        .map(|b| {
            let sn = t(&b.sn()).width(Length::Fixed(150.0));
            let productbill = t(&b.productbill()).width(Length::Fixed(50.0));
            let pn = t(&b.testtype()).width(Length::Fixed(50.0));
            let result = t(b.result()).width(Length::Fixed(50.0));
            let ith = t(b.ith()).width(Length::Fixed(50.0));
            let pf = t(&b.pf()).width(Length::Fixed(50.0));
            let vop = t(&b.vop()).width(Length::Fixed(50.0));
            let im = t(&b.im()).width(Length::Fixed(50.0));
            let rs = t(&b.rs()).width(Length::Fixed(50.0));
            let sen = t(b.sen()).width(Length::Fixed(50.0));
            let res = t(b.res()).width(Length::Fixed(50.0));
            let icc = t(&b.icc()).width(Length::Fixed(50.0));
            let idark = t(&b.idark()).width(Length::Fixed(50.0));
            let vbr = t(&b.vbr()).width(Length::Fixed(50.0));
            let ixtalk = t(&b.ixtalk()).width(Length::Fixed(50.0));
            let kink = t(b.kink()).width(Length::Fixed(50.0));
            let createtime = t(&b.testdate()).width(Length::Fixed(200.0));

            container(
                filled![
                    sn,
                    productbill,
                    pn,
                    result,
                    ith,
                    pf,
                    vop,
                    im,
                    rs,
                    sen,
                    res,
                    icc,
                    idark,
                    vbr,
                    ixtalk,
                    kink,
                    createtime
                ]
                .width(Length::Fill),
            )
            .padding(5)
            .style(loading_container_theme())
            .into()
        })
        .collect();

    container(column![
        header,
        scrollable(Column::with_children(rows).padding(10)).direction(vertical_direction()),
    ])
    .style(second_class_container_rounded_theme())
    .into()
}

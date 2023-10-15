use iced::{widget::text_input, Theme, theme::palette::Background};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInput {
    #[default]
    Default,

    /// A style that decorate a TextInput as a Text.
    ///
    /// This is a workaround until iced support selection of Text directly.
    /// See https://github.com/hecrj/iced/issues/36
    InputAsText,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: Background::Color(TEXTINPUT_SURFACE),
                border_radius: (2.).into(),
                border_width: 0.,
                border_color: Color::TRANSPARENT,
                icon_color: Color::TRANSPARENT,
            },
            TextInput::InputAsText => text_input::Appearance {
                background: Background::Color(MAIN_SURFACE_BACKGROUND),
                border_radius: (0.).into(),
                border_width: 0.,
                border_color: Color::TRANSPARENT,
                icon_color: Color::TRANSPARENT,
            },
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(MAIN_SURFACE_BACKGROUND),
            border_radius: (0.).into(),
            border_width: 0.,
            border_color: Color::TRANSPARENT,
            icon_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                border_width: 1.,
                border_color: ACCENT,
                ..self.active(style)
            },
            _ => self.active(style),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        match style {
            TextInput::Default => ACTIVE,
            _ => self.value_color(style),
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                border_width: 1.,
                border_color: Color { a: 0.3, ..ACCENT },
                ..self.focused(style)
            },
            _ => self.focused(style),
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }
}
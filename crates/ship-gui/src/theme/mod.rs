use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{theme, Background, Color};

pub enum Theme {
    First,
}

pub enum Button {
    Selected,
    Launch,
    Update,
}

impl button::StyleSheet for Button {
    type Style = theme::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        match self {
            Button::Selected => Appearance {
                background: Some(Background::Color(Color::new(0.0, 0.6, 0.8, 1.0))),
                text_color: Color::WHITE,
                ..Default::default()
            },
            Button::Launch => Appearance {
                background: Some(Background::Color(Color::new(0.0, 0.6, 0.0, 1.0))),
                text_color: Color::WHITE,
                ..Default::default()
            },
            Button::Update => Appearance {
                background: Some(Background::Color(Color::new(0.2, 0.4, 1.0, 1.0))),
                text_color: Color::WHITE,
                ..Default::default()
            },
        }
    }
    fn hovered(&self, _style: &Self::Style) -> Appearance {
        match self {
            Button::Selected => Appearance {
                background: Some(Background::Color(Color::new(0.0, 0.6, 0.8, 0.9))),
                text_color: Color::WHITE,
                ..Default::default()
            },
            Button::Launch => Appearance {
                background: Some(Background::Color(Color::new(0.0, 0.6, 0.0, 0.9))),
                text_color: Color::WHITE,
                ..Default::default()
            },
            Button::Update => Appearance {
                background: Some(Background::Color(Color::new(0.2, 0.4, 1.0, 0.9))),
                text_color: Color::WHITE,
                ..Default::default()
            },
        }
    }
}

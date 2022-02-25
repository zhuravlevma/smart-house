use iced::{button, Background, Color, Vector, Font, Text, Length};
use iced::HorizontalAlignment;

pub enum Button {
    House,
    Apartment,
    Device,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::House => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.6, 0.6, 0.7))),
                border_radius: 10.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Apartment => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.5, 0.6, 0.7))),
                border_radius: 10.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Device => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                border_radius: 10.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Destructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(
                    0.9, 0.3, 0.3,
                ))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }
}

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/fa-solid.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

pub fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

pub fn details_icons() -> Text {
    icon('\u{f107}')
}

pub fn exclamation_icon() -> Text {
    icon('\u{f06a}')
}

use iced::{button, Background, Color};

pub enum Button {
    House,
    Apartment,
    Device,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::House => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 10.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Apartment => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
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
        }
    }
}

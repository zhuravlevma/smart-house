use crate::style;
use iced::{button, Align, Button, Element, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApartmentView {
    pub house_id: String,
    pub name: String,
    #[serde(skip)]
    state: ApartmentViewState,
}

#[derive(Debug, Clone)]
pub enum ApartmentViewMessage {
    ViewDetails,
}

#[derive(Debug, Clone)]
pub enum ApartmentViewState {
    Idle { show_devices: button::State },
}

impl Default for ApartmentViewState {
    fn default() -> Self {
        ApartmentViewState::Idle {
            show_devices: button::State::new(),
        }
    }
}

impl ApartmentView {
    pub fn new(house_id: String, name: String) -> Self {
        Self {
            house_id,
            name,
            state: ApartmentViewState::Idle {
                show_devices: button::State::new(),
            },
        }
    }

    pub fn view(&mut self) -> Element<ApartmentViewMessage> {
        match &mut self.state {
            ApartmentViewState::Idle { show_devices } => Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(
                    Button::new(show_devices, Text::new(&self.name))
                        .on_press(ApartmentViewMessage::ViewDetails)
                        .padding(10)
                        .style(style::Button::Apartment),
                )
                .into(),
        }
    }
}

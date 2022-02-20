use crate::style;
use iced::{button, Align, Button, Element, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RosetteView {
    pub house_id: String,
    pub apartment_name: String,
    pub name: String,
    description: String,
    power: u32,
    ip: String,
    #[serde(skip)]
    state: RosetteViewState,
}

#[derive(Debug, Clone)]
pub enum RosetteViewMessage {
    ViewDetails,
}

#[derive(Debug, Clone)]
pub enum RosetteViewState {
    Idle { show_rosette: button::State },
}

impl Default for RosetteViewState {
    fn default() -> Self {
        RosetteViewState::Idle {
            show_rosette: button::State::new(),
        }
    }
}

impl RosetteView {
    pub fn new(
        house_id: String,
        apartment_name: String,
        name: String,
        description: String,
        ip: String,
        power: u32,
    ) -> Self {
        Self {
            house_id,
            apartment_name,
            name,
            description,
            ip,
            state: RosetteViewState::Idle {
                show_rosette: Default::default(),
            },
            power,
        }
    }
    pub fn view(&mut self) -> Element<RosetteViewMessage> {
        match &mut self.state {
            RosetteViewState::Idle { show_rosette } => {
                let label = Text::new(&self.name);
                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(show_rosette, label)
                            .on_press(RosetteViewMessage::ViewDetails)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .into()
            }
        }
    }
}

use crate::style;
use iced::{button, Align, Button, Column, Element, Row, Text};
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
    Idle {
        rosette_on: button::State,
        rosette_off: button::State,
    },
}

impl Default for RosetteViewState {
    fn default() -> Self {
        RosetteViewState::Idle {
            rosette_on: button::State::new(),
            rosette_off: button::State::new(),
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
                rosette_on: Default::default(),
                rosette_off: Default::default(),
            },
            power,
        }
    }
    pub fn view(&mut self) -> Element<RosetteViewMessage> {
        match &mut self.state {
            RosetteViewState::Idle {
                rosette_on,
                rosette_off,
            } => {
                let label = Text::new(&self.name);
                let description_label = Text::new(&self.description);
                let label_on = Text::new("On");
                let label_off = Text::new("Off");
                let title = Row::new().push(Text::new("Name: ")).push(label);
                let description = Row::new()
                    .push(Text::new("Description: "))
                    .push(description_label);
                let ip_label = Text::new(&self.ip);
                let ip = Row::new().push(Text::new("Ip: ")).push(ip_label);
                let row = Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(rosette_on, label_on)
                            .on_press(RosetteViewMessage::ViewDetails)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .push(
                        Button::new(rosette_off, label_off)
                            .on_press(RosetteViewMessage::ViewDetails)
                            .padding(10)
                            .style(style::Button::Device),
                    );
                Column::new()
                    .spacing(10)
                    .push(title)
                    .push(description)
                    .push(ip)
                    .push(row)
                    .into()
            }
        }
    }
}

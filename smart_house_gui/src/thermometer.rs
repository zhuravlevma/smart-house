use crate::style;
use iced::{button, Align, Button, Element, Row, Text, Column};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermometerView {
    pub house_id: String,
    pub apartment_name: String,
    pub name: String,
    description: String,
    temperature: f32,
    ip: String,
    updating: bool,
    #[serde(skip)]
    state: ThermometerViewState,
}

#[derive(Debug, Clone)]
pub enum ThermometerViewMessage {
    ViewDetails,
}

#[derive(Debug, Clone)]
pub enum ThermometerViewState {
    Idle { show_thermometer: button::State },
}

impl Default for ThermometerViewState {
    fn default() -> Self {
        ThermometerViewState::Idle {
            show_thermometer: button::State::new(),
        }
    }
}

impl ThermometerView {
    pub fn new(
        house_id: String,
        apartment_name: String,
        name: String,
        description: String,
        temperature: f32,
        ip: String,
        updating: bool,
    ) -> Self {
        Self {
            house_id,
            apartment_name,
            name,
            description,
            temperature,
            ip,
            updating,
            state: ThermometerViewState::Idle {
                show_thermometer: Default::default(),
            },
        }
    }
    pub fn view(&mut self) -> Element<ThermometerViewMessage> {
        match &mut self.state {
            ThermometerViewState::Idle { show_thermometer } => {
                let label = Text::new(&self.name);
                let description_label = Text::new(&self.description);
                let temperature_label = Text::new("Update temperature");
                let title = Row::new().push(Text::new("Name: ")).push(label);
                let description = Row::new().push(Text::new("Description: ")).push(description_label);
                let ip_label = Text::new(&self.ip);
                let ip = Row::new().push(Text::new("Ip: ")).push(ip_label);
                let row = Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(show_thermometer, temperature_label)
                            .on_press(ThermometerViewMessage::ViewDetails)
                            .padding(10)
                            .style(style::Button::Device),
                    );
                Column::new().spacing(10).push(title).push(description).push(ip).push(row).into()
            }
        }
    }
}

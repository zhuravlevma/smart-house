use crate::style::{delete_icon, sync_icon};
use crate::{style, Message};
use iced::{button, Align, Button, Column, Element, Length, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermometerView {
    pub house_id: String,
    pub apartment_name: String,
    pub name: String,
    description: String,
    pub(crate) temperature: f32,
    ip: String,
    updating: bool,
    #[serde(skip)]
    state: ThermometerViewState,
}

#[derive(Debug, Clone)]
pub enum ThermometerViewMessage {
    Sync,
    Delete,
}

#[derive(Debug, Clone)]
pub enum ThermometerViewState {
    Idle {
        show_thermometer: button::State,
        delete_button: button::State,
    },
}

impl Default for ThermometerViewState {
    fn default() -> Self {
        ThermometerViewState::Idle {
            show_thermometer: button::State::new(),
            delete_button: button::State::new(),
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
                delete_button: Default::default(),
            },
        }
    }
    pub fn view(&mut self) -> Element<ThermometerViewMessage> {
        match &mut self.state {
            ThermometerViewState::Idle {
                show_thermometer,
                delete_button,
            } => {
                let label = Text::new(&self.name);
                let description_label = Text::new(&self.description);
                let temperature_label = Row::new()
                    .push(Text::new("Temperature: "))
                    .push(Text::new(self.temperature.to_string()));
                let sync_label = Row::new().push(Text::new("Sync ")).push(sync_icon());
                let title = Row::new().push(Text::new("Name: ")).push(label);
                let description = Row::new()
                    .push(Text::new("Description: "))
                    .push(description_label);
                let ip_label = Text::new(&self.ip);
                let ip = Row::new().push(Text::new("Ip: ")).push(ip_label);
                let row = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(
                        Button::new(show_thermometer, sync_label)
                            .on_press(ThermometerViewMessage::Sync)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .push(
                        Button::new(delete_button, Row::new().spacing(10).push(delete_icon()))
                            .on_press(ThermometerViewMessage::Delete)
                            .padding(10)
                            .style(style::Button::Destructive),
                    );
                Column::new()
                    .spacing(10)
                    .push(title)
                    .push(description)
                    .push(ip)
                    .push(temperature_label)
                    .push(row)
                    .into()
            }
        }
    }
}

pub fn create_thermometer_elements(thermometers: &mut Vec<ThermometerView>) -> Element<Message> {
    thermometers
        .iter_mut()
        .fold(
            Column::new()
                .push(
                    Text::new("Thermometers")
                        .width(Length::Fill)
                        .size(40)
                        .color([0.5, 0.5, 0.5]),
                )
                .spacing(20),
            |column, thermometer| {
                let house_id = thermometer.house_id.clone();
                let apartment_name = thermometer.apartment_name.clone();
                let thermometer_name = thermometer.name.clone();
                column.push(thermometer.view().map(move |message| {
                    Message::ThermometerMessages(
                        house_id.clone(),
                        apartment_name.clone(),
                        thermometer_name.clone(),
                        message,
                    )
                }))
            },
        )
        .into()
}

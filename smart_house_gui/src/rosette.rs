use crate::style::{delete_icon, power_icon, sync_icon};
use crate::{style, Message};
use iced::{button, Align, Button, Column, Element, Length, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RosetteView {
    pub house_id: String,
    pub apartment_name: String,
    pub name: String,
    description: String,
    pub(crate) power: u32,
    ip: String,
    #[serde(skip)]
    state: RosetteViewState,
}

#[derive(Debug, Clone)]
pub enum RosetteViewMessage {
    On,
    Off,
    Sync,
    Delete,
}

#[derive(Debug, Clone)]
pub enum RosetteViewState {
    Idle {
        rosette_on: button::State,
        rosette_off: button::State,
        rosette_sync: button::State,
        delete_button: button::State,
    },
}

impl Default for RosetteViewState {
    fn default() -> Self {
        RosetteViewState::Idle {
            rosette_on: button::State::new(),
            rosette_off: button::State::new(),
            rosette_sync: button::State::new(),
            delete_button: button::State::new(),
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
                rosette_sync: Default::default(),
                delete_button: Default::default(),
            },
            power,
        }
    }
    pub fn view(&mut self) -> Element<RosetteViewMessage> {
        match &mut self.state {
            RosetteViewState::Idle {
                rosette_on,
                rosette_off,
                rosette_sync,
                delete_button,
            } => {
                let label = Text::new(&self.name);
                let description_label = Text::new(&self.description);
                let label_on = Row::new().push(Text::new("On ")).push(power_icon());
                let label_off = Row::new().push(Text::new("Off ")).push(power_icon());
                let label_sync = Row::new().push(Text::new("Sync ")).push(sync_icon());
                let power_label = Text::new(&self.power.to_string());
                let title = Row::new().push(Text::new("Name: ")).push(label);
                let description = Row::new()
                    .push(Text::new("Description: "))
                    .push(description_label);
                let ip_label = Text::new(&self.ip);
                let ip = Row::new().push(Text::new("Ip: ")).push(ip_label);
                let power = Row::new().push(Text::new("Power: ")).push(power_label);
                let row = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(
                        Button::new(rosette_on, label_on)
                            .on_press(RosetteViewMessage::On)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .push(
                        Button::new(rosette_off, label_off)
                            .on_press(RosetteViewMessage::Off)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .push(
                        Button::new(rosette_sync, label_sync)
                            .on_press(RosetteViewMessage::Sync)
                            .padding(10)
                            .style(style::Button::Device),
                    )
                    .push(
                        Button::new(delete_button, Row::new().spacing(10).push(delete_icon()))
                            .on_press(RosetteViewMessage::Delete)
                            .padding(10)
                            .style(style::Button::Destructive),
                    );
                Column::new()
                    .spacing(10)
                    .push(title)
                    .push(description)
                    .push(ip)
                    .push(power)
                    .push(row)
                    .into()
            }
        }
    }
}

pub fn create_rosette_elements(rosettes: &mut Vec<RosetteView>) -> Element<Message> {
    rosettes
        .iter_mut()
        .fold(
            Column::new()
                .push(
                    Text::new("Rosettes")
                        .width(Length::Fill)
                        .size(40)
                        .color([0.5, 0.5, 0.5]),
                )
                .spacing(20),
            |column, rosette| {
                let id = rosette.house_id.clone();
                let rosette_name = rosette.name.clone();
                let apartment_name = rosette.apartment_name.clone();
                column.push(rosette.view().map(move |message| {
                    Message::RosetteMessages(
                        id.clone(),
                        apartment_name.clone(),
                        rosette_name.clone(),
                        message,
                    )
                }))
            },
        )
        .into()
}

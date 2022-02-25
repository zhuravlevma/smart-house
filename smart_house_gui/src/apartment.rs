use crate::{Message, style};
use iced::{button, Align, Button, Element, Row, Text, Column};
use serde::{Deserialize, Serialize};
use crate::style::{delete_icon, details_icons, exclamation_icon};

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
    Delete
}

#[derive(Debug, Clone)]
pub enum ApartmentViewState {
    Idle { show_devices: button::State, delete_button: button::State },
}

impl Default for ApartmentViewState {
    fn default() -> Self {
        ApartmentViewState::Idle {
            show_devices: button::State::new(),
            delete_button: button::State::new()
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
                delete_button: button::State::new(),
            },
        }
    }

    pub fn view(&mut self) -> Element<ApartmentViewMessage> {
        match &mut self.state {
            ApartmentViewState::Idle { show_devices, delete_button } => {
                let label = Text::new(&self.name);
                let label_device = Text::new("devices");
                let title = Row::new().push(Text::new("Name: ")).align_items(Align::Center).spacing(10).push(label).push(
                    Button::new(delete_button, Row::new()
                        .spacing(5)
                        .push(delete_icon()))
                        .on_press(ApartmentViewMessage::Delete)
                        .padding(10).style(style::Button::Destructive),
                );
                Column::new().spacing(5).push(title).push(
                    Row::new()
                        .spacing(5)
                        .align_items(Align::Center)
                        .push(
                            Button::new(show_devices, Row::new()
                                .spacing(5)
                                .push(label_device)
                                .push(details_icons()))
                                .on_press(ApartmentViewMessage::ViewDetails)
                                .padding(10)
                                .style(style::Button::Apartment),
                        )
                ).into()
            },
        }
    }
}

pub fn empty_apartments() -> Element<'static, Message> {
    Column::new().push(Row::new().spacing(5).push(exclamation_icon()).push(Text::new("choose house who include apartments"))).into()
}
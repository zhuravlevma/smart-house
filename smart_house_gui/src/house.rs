use crate::style::{delete_icon, details_icons};
use crate::{style, Message};
use iced::{button, Align, Button, Column, Element, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseView {
    pub(crate) id: String,
    name: String,
    #[serde(skip)]
    state: HouseViewState,
}

#[derive(Debug, Clone)]
pub enum HouseViewMessage {
    ViewDetails,
    Delete,
}

#[derive(Debug, Clone)]
pub enum HouseViewState {
    Idle {
        show_apartments: button::State,
        delete_button: button::State,
    },
}

impl Default for HouseViewState {
    fn default() -> Self {
        HouseViewState::Idle {
            show_apartments: button::State::new(),
            delete_button: button::State::new(),
        }
    }
}

impl HouseView {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: HouseViewState::Idle {
                show_apartments: button::State::new(),
                delete_button: button::State::new(),
            },
        }
    }
    pub fn view(&mut self) -> Element<HouseViewMessage> {
        match &mut self.state {
            HouseViewState::Idle {
                show_apartments,
                delete_button,
            } => {
                let label = Text::new(&self.name);
                let label_detail = Text::new("apartments");
                let column = Column::new();
                let title = Row::new()
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(Text::new("Name: "))
                    .push(label);
                column
                    .push(title)
                    .spacing(10)
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_items(Align::Center)
                            .push(
                                Button::new(
                                    show_apartments,
                                    Row::new()
                                        .spacing(5)
                                        .push(label_detail)
                                        .push(details_icons()),
                                )
                                .on_press(HouseViewMessage::ViewDetails)
                                .padding(10)
                                .style(style::Button::House),
                            )
                            .push(
                                Button::new(
                                    delete_button,
                                    Row::new().spacing(10).push(delete_icon()),
                                )
                                .on_press(HouseViewMessage::Delete)
                                .padding(10)
                                .style(style::Button::Destructive),
                            ),
                    )
                    .into()
            }
        }
    }
}

pub fn create_house_elements(houses: &mut Vec<HouseView>) -> Element<Message> {
    houses
        .iter_mut()
        .fold(Column::new().spacing(20), |column, house| {
            let id = house.id.clone();
            column.push(
                house
                    .view()
                    .map(move |message| Message::HomeMessages(id.clone(), message)),
            )
        })
        .into()
}

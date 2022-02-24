use crate::style;
use iced::{button, Align, Button, Element, Row, Text, Column};
use serde::{Deserialize, Serialize};
use crate::style::{delete_icon};

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
    Idle { show_apartments: button::State, delete_button: button::State },
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
            HouseViewState::Idle { show_apartments, delete_button } => {
                let label = Text::new(&self.name);
                let label_add = Text::new("show details");
                let column = Column::new();
                let title = Row::new().push(Text::new("Name: ")).push(label);
                column.push(title).spacing(10).push(
                    Row::new()
                        .spacing(20)
                        .align_items(Align::Center)
                        .push(
                            Button::new(show_apartments, label_add)
                                .on_press(HouseViewMessage::ViewDetails)
                                .padding(10)
                                .style(style::Button::House),
                        ).push(
                        Button::new(delete_button, Row::new()
                            .spacing(10)
                            .push(delete_icon()))
                            .on_press(HouseViewMessage::Delete)
                            .padding(10).style(style::Button::Destructive)
                    )
                ).into()
            }
        }
    }
}

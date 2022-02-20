use crate::style;
use iced::{button, Align, Button, Element, Row, Text};
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
}

#[derive(Debug, Clone)]
pub enum HouseViewState {
    Idle { show_apartments: button::State },
}

impl Default for HouseViewState {
    fn default() -> Self {
        HouseViewState::Idle {
            show_apartments: button::State::new(),
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
            },
        }
    }
    pub fn view(&mut self) -> Element<HouseViewMessage> {
        match &mut self.state {
            HouseViewState::Idle { show_apartments } => {
                let label = Text::new(&self.name);
                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(show_apartments, label)
                            .on_press(HouseViewMessage::ViewDetails)
                            .padding(10)
                            .style(style::Button::House),
                    )
                    .into()
            }
        }
    }
}

use iced::{button, Align, Button, Element, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseView {
    pub(crate) id: String,
    name: String,
    completed: bool,

    #[serde(skip)]
    state: HouseViewState,
}

#[derive(Debug, Clone)]
pub enum HouseViewMessage {
    ViewDetails,
}

#[derive(Debug, Clone)]
pub enum HouseViewState {
    Idle { edit_button: button::State },
}

impl Default for HouseViewState {
    fn default() -> Self {
        HouseViewState::Idle {
            edit_button: button::State::new(),
        }
    }
}

impl HouseView {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: HouseViewState::Idle {
                edit_button: button::State::new(),
            },
            completed: false,
        }
    }
    pub fn view(&mut self) -> Element<HouseViewMessage> {
        match &mut self.state {
            HouseViewState::Idle { edit_button } => Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(
                    Button::new(edit_button, Text::new(&self.name))
                        .on_press(HouseViewMessage::ViewDetails)
                        .padding(10),
                )
                .into(),
        }
    }
}

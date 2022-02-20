use crate::HomeMessage;
use iced::{button, Align, Button, Element, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeElem {
    pub(crate) id: String,
    description: String,
    name: String,
    completed: bool,

    #[serde(skip)]
    state: HomeElemState,
}

#[derive(Debug, Clone)]
pub enum HomeElemState {
    Idle { edit_button: button::State },
}

impl Default for HomeElemState {
    fn default() -> Self {
        HomeElemState::Idle {
            edit_button: button::State::new(),
        }
    }
}

impl HomeElem {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            state: HomeElemState::Idle {
                edit_button: button::State::new(),
            },
            completed: false,
        }
    }
    pub fn view(&mut self) -> Element<HomeMessage> {
        match &mut self.state {
            HomeElemState::Idle { edit_button } => Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(
                    Button::new(edit_button, Text::new(&self.name))
                        .on_press(HomeMessage::ViewDetails)
                        .padding(10),
                )
                .into(),
        }
    }
}

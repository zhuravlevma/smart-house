mod house;

use crate::house::HomeElem;
use iced::{Application, Clipboard, Column, Command, Container, Element, Length, Settings, Text};
use smart_house::{Apartment, House};

fn main() -> iced::Result {
    Home::run(Settings::default())
}

async fn get_houses() -> Vec<House> {
    let url = String::from("http://127.0.0.1:8080/");
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<House> = res.json().await.unwrap();
    tmp
}

async fn get_apartments(id: String) -> Vec<Apartment> {
    let url = format!("http://127.0.0.1:8080/{}/apartment", { id });
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<Apartment> = res.json().await.unwrap();
    tmp
}

enum Home {
    Loaded(State),
    Loading,
}

#[derive(Debug, Default)]
struct State {
    houses: Vec<HomeElem>,
}

#[derive(Debug, Clone)]
enum Message {
    ViewResultApartments(Vec<Apartment>),
    Loaded(Vec<House>),
    HomeMessages(String, usize, HomeMessage),
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    ViewDetails,
}

impl Application for Home {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Home::Loading,
            Command::perform(get_houses(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        match &self {
            Home::Loaded(_state) => "House".to_string(),
            Home::Loading => "House loading".to_string(),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match self {
            Home::Loaded(_state) => match message {
                Message::ViewResultApartments(data) => {
                    println!("{:?}", data);
                    Command::none()
                }
                Message::HomeMessages(house_id, _i, HomeMessage::ViewDetails) => {
                    Command::perform(get_apartments(house_id), Message::ViewResultApartments)
                }
                _ => Command::none(),
            },
            Home::Loading => match message {
                Message::Loaded(houses) => {
                    let house_domain = houses
                        .iter()
                        .map(|elem| {
                            HomeElem::new(
                                elem.get_id().to_string(),
                                elem.get_name().to_string(),
                                "Haha".to_string(),
                            )
                        })
                        .collect();
                    println!("{:?}", house_domain);
                    *self = Home::Loaded(State {
                        houses: house_domain,
                    });
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Home::Loaded(State { houses }) => {
                let title = Text::new("Houses")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5]);
                let houses: Element<Message> = houses
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (i, house)| {
                        let id = house.id.clone();
                        column.push(
                            house
                                .view()
                                .map(move |message| Message::HomeMessages(id.clone(), i, message)),
                        )
                    })
                    .into();

                let content = Column::new()
                    .push(title)
                    .push(Container::new(houses))
                    .into();
                content
            }
            Home::Loading => loading_message(),
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Loading...").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

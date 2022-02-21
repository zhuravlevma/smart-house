use crate::apartment::{ApartmentView, ApartmentViewMessage};
use crate::api::{get_apartments, get_devices, get_houses};
use crate::house::{HouseView, HouseViewMessage};
use crate::rosette::{RosetteView, RosetteViewMessage};
use crate::thermometer::{ThermometerView, ThermometerViewMessage};
use iced::{Application, Clipboard, Column, Command, Container, Element, Length, Settings, Text};
use smart_house::{Apartment, Device, House};

fn main() -> iced::Result {
    Home::run(Settings::default())
}

enum Home {
    Loaded(State),
    Loading,
}

#[derive(Debug, Default)]
struct State {
    houses: Vec<HouseView>,
    apartments: Vec<ApartmentView>,
    thermometers: Vec<ThermometerView>,
    rosettes: Vec<RosetteView>,
}

#[derive(Debug, Clone)]
enum Message {
    ViewResultApartments((String, Vec<Apartment>)),
    Loaded(Vec<House>),
    HomeMessages(String, HouseViewMessage),
    ApartmentMessages(String, String, ApartmentViewMessage),
    ViewDetailsApartments((String, String, Vec<Device>)),
    ThermometerMessages(String, ThermometerViewMessage),
    RosetteMessages(String, RosetteViewMessage),
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
            Home::Loaded(state) => match message {
                Message::ViewResultApartments((id, apartments)) => {
                    let apartments_domain: Vec<ApartmentView> = apartments
                        .iter()
                        .map(|elem| ApartmentView::new(id.clone(), elem.get_name().to_string()))
                        .collect();
                    state.thermometers = vec![];
                    state.rosettes = vec![];
                    state.apartments = apartments_domain;
                    Command::none()
                }
                Message::HomeMessages(house_id, HouseViewMessage::ViewDetails) => {
                    Command::perform(get_apartments(house_id), Message::ViewResultApartments)
                }
                Message::ApartmentMessages(
                    house_id,
                    apartment_name,
                    ApartmentViewMessage::ViewDetails,
                ) => Command::perform(
                    get_devices(house_id, apartment_name),
                    Message::ViewDetailsApartments,
                ),
                Message::ViewDetailsApartments((house_id, apartment_name, devices)) => {
                    let mut thermometers = vec![];
                    let mut rosettes = vec![];
                    devices.iter().for_each(|device| match device {
                        Device::Rosette(rosette) => rosettes.push(RosetteView::new(
                            house_id.clone(),
                            apartment_name.clone(),
                            rosette.get_name().to_string(),
                            rosette.get_description().to_string(),
                            rosette.get_ip().to_string(),
                            0,
                        )),
                        Device::Thermometer(thermometer) => {
                            thermometers.push(ThermometerView::new(
                                house_id.clone(),
                                apartment_name.clone(),
                                thermometer.get_name().to_string(),
                                thermometer.get_description().to_string(),
                                0.0,
                                thermometer.get_ip().to_string(),
                                false,
                            ))
                        }
                    });
                    state.thermometers = thermometers;
                    state.rosettes = rosettes;
                    Command::none()
                }
                Message::ThermometerMessages(id, _message) => {
                    println!("{}", id);
                    Command::none()
                }
                Message::RosetteMessages(id, _message) => {
                    println!("{}", id);
                    Command::none()
                }
                _ => Command::none(),
            },
            Home::Loading => match message {
                Message::Loaded(houses) => {
                    let house_domain = houses
                        .iter()
                        .map(|elem| {
                            HouseView::new(elem.get_id().to_string(), elem.get_name().to_string())
                        })
                        .collect();
                    *self = Home::Loaded(State {
                        houses: house_domain,
                        apartments: vec![],
                        thermometers: vec![],
                        rosettes: vec![],
                    });
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Home::Loaded(State {
                houses,
                apartments,
                thermometers,
                rosettes,
            }) => {
                let title = Text::new("Houses")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5]);
                let houses: Element<Message> = houses
                    .iter_mut()
                    .fold(Column::new().spacing(20), |column, house| {
                        let id = house.id.clone();
                        column.push(
                            house
                                .view()
                                .map(move |message| Message::HomeMessages(id.clone(), message)),
                        )
                    })
                    .into();

                let apartments: Element<Message> = if !apartments.is_empty() {
                    apartments
                        .iter_mut()
                        .fold(
                            Column::new()
                                .push(
                                    Text::new("Apartments")
                                        .width(Length::Fill)
                                        .size(50)
                                        .color([0.5, 0.5, 0.5]),
                                )
                                .spacing(20),
                            |column, apartment| {
                                let name = apartment.name.clone();
                                let id = apartment.house_id.clone();
                                column.push(apartment.view().map(move |message| {
                                    Message::ApartmentMessages(id.clone(), name.clone(), message)
                                }))
                            },
                        )
                        .into()
                } else {
                    Column::new().into()
                };

                let thermometers: Element<Message> = if !thermometers.is_empty() {
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
                                let id = thermometer.house_id.clone();
                                column.push(thermometer.view().map(move |message| {
                                    Message::ThermometerMessages(id.clone(), message)
                                }))
                            },
                        )
                        .into()
                } else {
                    Column::new().into()
                };

                let rosettes: Element<Message> = if !rosettes.is_empty() {
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
                                column.push(rosette.view().map(move |message| {
                                    Message::RosetteMessages(id.clone(), message)
                                }))
                            },
                        )
                        .into()
                } else {
                    Column::new().into()
                };

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(Container::new(houses))
                    .push(Container::new(apartments))
                    .push(Container::new(rosettes))
                    .push(Container::new(thermometers));
                Container::new(content)
                    .width(Length::Fill)
                    .center_x()
                    .into()
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

mod apartment;
mod api;
mod house;
mod rosette;
mod style;
mod thermometer;

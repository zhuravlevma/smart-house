use crate::apartment::{
    create_apartment_elements, empty_apartments, ApartmentView, ApartmentViewMessage,
};
use crate::api::{
    get_apartments, get_devices, get_houses, rosette_off, rosette_on, rosette_sync,
    thermometer_sync,
};
use crate::house::{create_house_elements, HouseView, HouseViewMessage};
use crate::rosette::{create_rosette_elements, RosetteView, RosetteViewMessage};
use crate::thermometer::{create_thermometer_elements, ThermometerView, ThermometerViewMessage};
use iced::scrollable::{self, Scrollable};
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
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewResultApartments((String, Vec<Apartment>)),
    Loaded(Vec<House>),
    HomeMessages(String, HouseViewMessage),
    ApartmentMessages(String, String, ApartmentViewMessage),
    ViewDetailsApartments((String, String, Vec<Device>)),
    ThermometerMessages(String, String, String, ThermometerViewMessage),
    RosetteMessages(String, String, String, RosetteViewMessage),
    RosetteOff((String, String, String, bool)),
    RosetteOn((String, String, String, bool)),
    RosetteSync((String, String, String, u32)),
    ThermometerSync((String, String, String, f32)),
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
                Message::ThermometerMessages(id, apartment_name, rosette_name, message) => {
                    match message {
                        ThermometerViewMessage::Sync => Command::perform(
                            thermometer_sync(id, apartment_name, rosette_name),
                            Message::ThermometerSync,
                        ),
                        ThermometerViewMessage::Delete => Command::none(),
                    }
                }
                Message::RosetteMessages(id, apartment_name, rosette_name, message) => {
                    match message {
                        RosetteViewMessage::On => Command::perform(
                            rosette_on(id, apartment_name, rosette_name),
                            Message::RosetteOn,
                        ),
                        RosetteViewMessage::Off => Command::perform(
                            rosette_off(id, apartment_name, rosette_name),
                            Message::RosetteOff,
                        ),
                        RosetteViewMessage::Sync => Command::perform(
                            rosette_sync(id, apartment_name, rosette_name),
                            Message::RosetteSync,
                        ),
                        RosetteViewMessage::Delete => Command::none(),
                    }
                }
                Message::RosetteOff((id, apartment_name, rosette_name, _res)) => {
                    state.rosettes.iter_mut().for_each(|el| {
                        if el.house_id == id
                            && el.apartment_name == apartment_name
                            && el.name == rosette_name
                        {
                            el.power = 0
                        }
                    });
                    Command::none()
                }
                Message::RosetteOn((id, apartment_name, rosette_name, _res)) => {
                    state.rosettes.iter_mut().for_each(|el| {
                        if el.house_id == id
                            && el.apartment_name == apartment_name
                            && el.name == rosette_name
                        {
                            el.power = 220
                        }
                    });
                    Command::none()
                }
                Message::RosetteSync((id, apartment_name, rosette_name, res)) => {
                    state.rosettes.iter_mut().for_each(|el| {
                        if el.house_id == id
                            && el.apartment_name == apartment_name
                            && el.name == rosette_name
                        {
                            el.power = res
                        }
                    });
                    Command::none()
                }
                Message::ThermometerSync((id, apartment_name, thermometer_name, res)) => {
                    state.thermometers.iter_mut().for_each(|el| {
                        if el.house_id == id
                            && el.apartment_name == apartment_name
                            && el.name == thermometer_name
                        {
                            el.temperature = res
                        }
                    });
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
                        ..State::default()
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
                scroll,
            }) => {
                let title = Text::new("Houses")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5]);
                let houses: Element<Message> = create_house_elements(houses);

                let apartments: Element<Message> = if !apartments.is_empty() {
                    create_apartment_elements(apartments)
                } else {
                    empty_apartments()
                };

                let thermometers: Element<Message> = if !thermometers.is_empty() {
                    create_thermometer_elements(thermometers)
                } else {
                    Column::new().into()
                };

                let rosettes: Element<Message> = if !rosettes.is_empty() {
                    create_rosette_elements(rosettes)
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
                Scrollable::new(scroll)
                    .padding(40)
                    .push(Container::new(content).width(Length::Fill).center_x())
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

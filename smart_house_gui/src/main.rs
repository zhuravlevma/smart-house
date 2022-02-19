use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Settings,
    Text,
};
use smart_house::House;

fn main() -> iced::Result {
    State::run(Settings::default())
}

async fn test() -> Vec<House> {
    let url = String::from("http://127.0.0.1:8080/");
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<House> = res.json().await.unwrap();
    println!("{:?}", tmp[0].get_apartments());
    let house = House::new("id".to_string(), "name".to_string());
    vec![house]
}

#[derive(Debug, Default)]
struct State {
    houses: Vec<House>,
    view: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    View,
    Response(Vec<House>),
}

impl iced::Application for State {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Smart home")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::View => Command::perform(test(), Message::Response),
            Message::Response(houses) => {
                self.houses = houses;
                println!("{:?}", self.houses);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let button_view = Button::new(&mut self.view, Text::new("View")).on_press(Message::View);
        Column::new()
            .padding(40)
            .align_items(Align::Center)
            .push(button_view)
            .into()
    }
}

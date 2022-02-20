// use iced::{button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Settings, Text, Container, Row};
// use smart_house::{Apartment, Device, House};
//
// fn main() -> iced::Result {
//     State::run(Settings::default())
// }
//
// async fn test() -> Vec<House> {
//     let url = String::from("http://127.0.0.1:8080/");
//     let res = reqwest::get(&url).await.unwrap();
//     let tmp: Vec<House> = res.json().await.unwrap();
//     println!("{:?}", tmp[0].get_apartments());
//     tmp
// }
//
// #[derive(Debug, Default)]
// struct State {
//     houses: Vec<House>,
//     view_all_houses_button: button::State,
//     house_button: Vec<button::State>,
// }
//
// #[derive(Debug, Clone)]
// pub enum Message {
//     ViewAllHouses,
//     ResponseAllHouses(Vec<House>),
// }
//
// impl iced::Application for State {
//     type Executor = executor::Default;
//     type Message = Message;
//     type Flags = ();
//
//     fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
//         (Self::default(), Command::none())
//     }
//
//     fn title(&self) -> String {
//         String::from("Smart home")
//     }
//
//     fn update(
//         &mut self,
//         message: Self::Message,
//         _clipboard: &mut Clipboard,
//     ) -> Command<Self::Message> {
//         match message {
//             Message::ViewAllHouses => Command::perform(test(), Message::ResponseAllHouses),
//             Message::ResponseAllHouses(houses) => {
//                 self.houses = houses;
//                 println!("{:?}", self.houses);
//                 Command::none()
//             }
//         }
//     }
//
//     fn view(&mut self) -> Element<Self::Message> {
//         let button_view = Button::new(&mut self.view_all_houses_button, Text::new("View")).on_press(Message::ViewAllHouses);
//         let houses: Vec<&str> = self.houses.iter().map(|house| house.get_name()).collect();
//         let mut rows = Vec::new();
//         self.house_button = vec![];
//         for _house in &houses {
//             self.house_button.push(button::State::new())
//         }
//         for house in houses {
//             let label = Text::new(house);
//             let mut new_state = button::State::new();
//             let button = Button::new(&mut new_state, label);
//             let row: Element<Self::Message>= Row::new().push(button).into();
//             self.house_button.push(new_state);
//             rows.push(row);
//         }
//         // let column = Column::with_children(rows);
//         Column::new()
//             .padding(40)
//             .align_items(Align::Center)
//             .push(button_view)
//             // .push(column)
//             .into()
//     }
// }
fn main() {}

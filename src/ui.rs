use crate::tcp;
use crate::listener;
use crate::style;

use iced::{
    executor, Application, Row, Column, Command,Element, Settings,
    Text, Subscription, Container, Length,
};


pub fn main() {
    AppState::run(Settings::default())
}


struct AppState {
    connections: Vec<tcp::ConnectionDetails>,
    listening: bool
}


#[derive(Debug, Clone, Copy)]
enum Message {
    ConnectionAttempt(tcp::Connection)
}


impl Application for AppState {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (AppState, Command<Message>) {(
        AppState {
            connections: vec![],
            listening: true,
        },
        Command::none(),
    )}

    fn title(&self) -> String {
        String::from("Panopticon")
    }

    

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConnectionAttempt(x) => {
                println!("Connection Attempt {}", x);
                self.connections.push(tcp::addr_lookup(x.to_string()))
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.listening {
            listener::listen("0.0.0.0:7878").map(Message::ConnectionAttempt)
        } else {
            Subscription::none()
        }
    }


    fn view(&mut self) -> Element<Message> {
        let placeholder = Text::new("--- Map Here :D ---");
        let connections: Element<_> = if self.connections.iter().count() > 0 {
            self.connections
                .iter()
                .fold(Column::new().spacing(20), |column, conn|
                      column.push(Row::new()
                                  .push(Text::new(conn.addr.clone()))))
                .into()
        } else {
            Column::new().spacing(20)
                .push(Text::new("No connections yet")).into()
        };
        Container::new(
            Row::new()
                .push(
                    Container::new(
                        Column::new()
                            .push(placeholder))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(style::Map))
                .push(
                    Container::new(connections)))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Connections)
            .into()
    }
}

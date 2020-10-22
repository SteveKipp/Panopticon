use crate::tcp;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use iced::{
    executor, Application, Column, Command,Element, Settings,
    Subscription, Text,
};

pub fn main() -> iced::Result {
    AppState::run(Settings::default())
}

struct AppState {
    state: ListenState,
    connections: Vec<i32>,
}

enum ListenState {
    Idle,
    Active,
}


#[derive(Debug, Clone, Copy)]
enum Message {
    ConnectionAttempt(i32),
}

impl Application for AppState {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (AppState, Command<Message>) {(
        AppState {
            state: ListenState::Idle,
            connections: vec![],
        },
        Command::none(),
    )}

    fn title(&self) -> String {
        String::from("Panopticon")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConnectionAttempt(x) => self.connections.push(x),
        }

        Command::none()
    }


    fn view(&mut self) -> Element<Message> {
        let connection_vec = Text::new(format!("{:?}", self.connections));

        Column::new()
            .padding(20)
            .push(connection_vec)
            .into()
    }
}

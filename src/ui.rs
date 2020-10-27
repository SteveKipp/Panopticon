use crate::tcp;
use crate::listener;
use iced::{
    executor, Application, Column, Command,Element, Settings, Text, Subscription,
};


pub fn main() {
    AppState::run(Settings::default())
}

struct AppState {
    connections: Vec<String>,
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
                self.connections.push(x.to_string())
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
        let connection_vec = Text::new(format!("{:?}", self.connections));

        Column::new()
            .padding(20)
            .push(connection_vec)
            .into()
    }
}

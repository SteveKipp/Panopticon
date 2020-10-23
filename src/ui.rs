use iced::{
    executor, Application, Column, Command,Element, Settings, Text, Subscription,
};


pub fn main() -> iced::Result {
    AppState::run(Settings::default())
}

struct AppState {
    connections: Vec<i32>,
    listening: bool
}

pub enum Connection{
    Incoming(String),
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
                println!("Connection Attempt");
                self.connections.push(x)
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.listening {
            Subscription::none()
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

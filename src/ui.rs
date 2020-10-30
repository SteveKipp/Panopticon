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

fn small_time(timestamp: String) -> String{
    let start = timestamp.find('T').unwrap();
    timestamp[start+1..start+8].to_string()
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
                .fold(Column::new(), |column, conn|
                      column.push(
                          Container::new(
                              Column::new()
                                  .push(
                                      Row::new()
                                          .push(Container::new(Text::new("-> ")).style(style::Green))
                                          .push(Text::new(&conn.addr))
                                  )
                                  .push(
                                      Row::new()
                                          .push(Container::new(Text::new("Origin: ")).style(style::Yellow))
                                          .push(Text::new(&conn.country))
                                          .push(Text::new("       "))
                                          .push(Container::new(Text::new("Time: ")).style(style::Yellow))
                                          .push(Text::new(small_time(conn.timestamp.clone())))
                                  )
                          ).padding(10)
                           .width(Length::Units(250))
                           .style(style::Notification)
                      )
                )
                .into()
        } else {
            Column::new()
                .push(Text::new("No connections yet")).into()
        };

        //STOPNOTE - need to make this a scrollable, and add hostname to ConnectionDetails struct
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

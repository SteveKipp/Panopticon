use iced::{Button, button, executor, Application, Command, Element,
           Settings, Column, Text};

struct Capture {
    enabled: bool,

    start_stop_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message{
    StartStopPressed,
}


impl Capture {

    //fn value_to_string(&mut self){
    //    if self.enabled == true {
    //        return "Active".to_string();
    //    } else {
    //        return "Stopped".to_string();
    //    }
    //}
    //this is the layout / view logic
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(
                //push the label to the column
                Text::new("Undynamic Text")
                    .size(50),
            )
            .push(
                //push the start/stop button to the column
                Button::new(&mut self.start_stop_button, Text::new("Start"))
                    .on_press(Message::StartStopPressed)
            )
    }

    pub fn update(&mut self, message: Message){
        match message {
            Message::StartStopPressed => {
                self.enabled = !self.enabled;
            }
        }
    }
}

struct Panopticon;

impl Application for Panopticon {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();


    fn new(_flags: ()) -> (Panopticon, Command<Self::Message>) {
        (Panopticon, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}

pub fn main(){
    Panopticon::run(Settings::default())
}

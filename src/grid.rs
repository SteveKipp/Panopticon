use iced::{         
    canvas::{
        self, Cache, Canvas, Cursor, Event, Frame, Geometry, Path, Text,
    },
    Vector, Element, Length, Rectangle,
};

pub struct Grid {
    scaling: f32,
    translation: Vector,
    show_lines: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Pinged
}

impl Grid {
    const MAX_LAT: f32 = 90.0;
    const MIN_LAT: f32 = -90.0;

    const MAX_LONG: f32 = 180.0;
    const MIN_LONG: f32 = -180.0;

    pub fn build() -> Self{
        //here Self = Grid
        Self {
            scaling: 0.01,
            translation: Vector::default(),
            show_lines: false,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message{
            Message::Pinged => {
                println!("Incoming connection passed to lat long grid")
            }
        }
    }

    pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        Canvas::new(self)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

}

//STOPNOTE - 
//this is going to be a beast of a function 
// this impl is required in order for the view to function to work
impl<'a> canvas::Program<Message> for Grid {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor,) -> Option<Message> {
        Some(Message::Pinged)
    }

    fn draw(&self, bounds: Rectangle, cursor: Cursor)  -> Vec<Geometry> {
        vec![]
    }
}
use crate::tcp;
use std::thread;
use druid::im::{vector, Vector};
use druid::widget::{Flex, Label, List, Scroll};
use druid::{
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc,
};


//I think this is the state of a component
#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub incoming: Vector<u32>,
}

fn launch(data: AppData){
    let window = WindowDesc::new(ui_builder).window_size((800.0, 600.0));
     AppLauncher::with_window(window)
            .use_simple_logger()
            .launch(data)
            .expect("#! Launch failed")
}

pub fn main() {
    let inc = vector![0, 1, 2, 3];
    let mut data = AppData { incoming: inc };

    // STOPNOTE - moving data between the threads is hard
    // need to figure out to make the data able to be moved into
    // a clojure
    thread::spawn(|| {
        tcp::listen(7878, &mut data);
    });
    launch(data);
}

fn ui_builder() -> impl Widget<AppData> {
    let log = Scroll::new(List::new(|| {
                Label::new(|item: &u32, _env: &_| format!("I -> {}", item))
                    .background(Color::rgb(0.5, 0.5, 0.5))
                })).vertical().lens(AppData::incoming);

    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("Map Widget Here")
                            .expand()
                            .border(Color::WHITE, 2.0),
            1.0),
        1.0)
        .with_child(log
                    .expand()
                    .width(100.0)
                    //Stop note: how do I make it fill the space only vertically
                    .height(600.0)
                    .border(Color::WHITE, 2.0))

        //.add_child(
        //    Flex::column()
        //        .with_flex_child(log, 1.0))
}

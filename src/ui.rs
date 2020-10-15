use crate::tcp;
use druid::im::{vector, Vector};
use druid::widget::{Flex, Label, List, Scroll};
use druid::{
    AppLauncher, Color, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};


//I think this is the state of a component
#[derive(Clone, Data, Lens)]
struct AppData {
    incoming: Vector<u32>,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder).window_size((800.0, 600.0));
    let inc = vector![0, 1, 2, 3];
    let data = AppData { incoming: inc };
    // this listen has to be run asynchronously from the ui
    // or else it halts the ui execution
    //tcp::listen(7878);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("#! Launch failed")
}

fn ui_builder() -> impl Widget<AppData> {
    let log = Scroll::new(List::new(|| {
                Label::new(|item: &u32, _env: &_| format!("Incoming -> #{}", item))
                    .align_vertical(UnitPoint::RIGHT)
                    .expand()
                    .height(50.0)
                    .width(50.0)
                    .background(Color::rgb(0.5, 0.5, 0.5))
                })).vertical().lens(AppData::incoming);

    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(
                Label::new("Map Widget Here")
                    .center(),
                1.0).fix_width(500.0),
            1.0)
        .with_flex_child(
            Flex::column()
                .with_flex_child(log, 1.0), 1.0)
}

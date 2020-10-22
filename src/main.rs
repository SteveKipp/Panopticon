mod tcp;
mod ui;
use::iced;

fn main() -> iced::Result {
    println!("Starting the Panopticon...");
    ui::main()
}

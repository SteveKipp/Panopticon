mod tcp;
mod ui;

fn main() {
    println!("Starting the Panopticon...");
    ui::main();
    //tcp::listen(7878);
}

mod tcp;
mod ui;

fn main() {
    println!("Starting the Panopticon...");
    //tcp::listen(7878);
    ui::main();
}

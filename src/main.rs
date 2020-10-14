mod tcp;

fn main() {
    println!("Starting the Panopticon...");
    tcp::listen(7878);
}

mod tcp;

fn main() {
    println!("Starting server on port 22...");
    tcp::listen(22);
}

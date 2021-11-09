struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Server is running on: {}!", self.addr);
    }
}

fn main() {
    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run();
}

use http::{
    Request,
    Method
};
use server::Server;

mod http;
mod server;
fn main() {
    let string = String::from("localhost:8080".to_string());
    let server = Server::new(string);
    server.run();
}
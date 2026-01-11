use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    println!("{}", listener.local_addr().unwrap().port());
    run(listener)?.await
}
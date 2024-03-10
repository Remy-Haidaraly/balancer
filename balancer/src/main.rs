use std::{io, process};
use tokio::time::interval;
use actix_web::{web, App, HttpServer};
use std::time::Duration;

mod colors;
mod logging;
mod server;
mod upstream;

const MIN_PORT: u16 = 1024;
const MAX_PORT: u16 = 65534;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port: u16;
    loop {
        println!("Choose a port to bind ({}-{}) :", MIN_PORT, MAX_PORT);
        let mut port_input = String::new();
        io::stdin().read_line(&mut port_input).expect("Error reading input line");
        port = match port_input.trim().parse() {
            Ok(num) if num >= MIN_PORT && num <= MAX_PORT => num,
            _ => {
                println!("Invalid port. Please enter a number between {} and {}.", MIN_PORT, MAX_PORT);
                continue;
            }
        };
        break;
    }

 // Set up a background task to periodically check upstream servers.
    let mut interval = interval(Duration::from_secs(3));
    actix_rt::spawn(async move {
        loop {
            interval.tick().await;
            server::check_upstream_servers().await;
        }
    });

  // Register a Ctrl+C signal handler for graceful shutdown.
    ctrlc::set_handler(move || {
        println!("\nServer shutting down...");
        logging::write_log();
        process::exit(0);
    })
    .expect("Error configuring signal handler");

    println!("Server is running. Press Ctrl+C to stop");

    // Start the Actix Web server to handle incoming requests.
    HttpServer::new(|| {
        App::new().route("/", web::get().to(server::redirect))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

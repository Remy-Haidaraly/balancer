use actix_web::{HttpRequest, HttpResponse, http::StatusCode};
use awc::Client;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use crate::logging::REQUEST_COUNT;
use crate::logging::SERVER_REQUESTS;
use std::sync::atomic::Ordering;
use crate::colors::{colorize, TextColor};
use std::io;
use std::io::Write;
use crate::upstream::UPSTREAM_SERVERS;

lazy_static::lazy_static! {
    /// Atomic counter for round-robin load balancing.
    static ref ROUND_ROBIN_COUNTER: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
}

pub async fn try_upstream(server: &str, client: &Client, _req: &HttpRequest) -> Result<HttpResponse, ()> {
    let response = client.get(server).send().await;
    match response {
        Ok(resp) if resp.status().is_success() => {
            REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
            let mut server_requests = SERVER_REQUESTS.lock().unwrap();
            *server_requests.entry(server.to_string()).or_insert(0) += 1;
            Ok(HttpResponse::Ok().body(format!("Proxied to {}", server)))
        }
        _ => Err(()),
    }
}

/// Redirects the incoming request to an available upstream server.
///
/// This function implements a round-robin load balancing strategy to redirect
/// incoming requests to available upstream servers. If all servers are down,
/// it returns a `Service Unavailable` response.
///
pub async fn redirect(req: HttpRequest) -> HttpResponse {
    let client = Client::default();
    let mut servers = UPSTREAM_SERVERS.lock().unwrap();
    let mut server_index = ROUND_ROBIN_COUNTER.fetch_add(1, Ordering::Relaxed) % servers.len();
    
    for _ in 0..servers.len() {
        let (server, alive) = &mut servers[server_index];
        if *alive {
            match try_upstream(server, &client, &req).await {
                Ok(response) => return response,
                Err(_) => *alive = false,
            }
        }
        server_index = (server_index + 1) % servers.len();
    }

    HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE)
}

/// Checks the availability of upstream servers.
///
/// This function sends a health check request to each upstream server to determine
/// its availability. It updates the status of each server and prints a summary of
/// the available and unavailable servers to the console.
pub async fn check_upstream_servers() {
    let client = Client::default();
    let mut servers = UPSTREAM_SERVERS.lock().unwrap();
    let alive_servers_count = servers.iter().filter(|(_, alive)| *alive).count();

    if alive_servers_count == 1 {
        let alive_server_index = servers.iter().position(|(_, alive)| *alive).unwrap();
        ROUND_ROBIN_COUNTER.store(alive_server_index as usize, Ordering::Relaxed);
    }

    for (server, alive) in servers.iter_mut() {
        let prev_alive = *alive;
        *alive = client.get(server.as_str()).send().await.is_ok();

        if *alive != prev_alive {
            print_server_status(server, *alive);
        }
    }

    print_servers_summary(&servers);
}
fn print_server_status(server: &String, is_up: bool) {
    let status = if is_up { colorize("UP", TextColor::Green) } else { colorize("DOWN", TextColor::Red) };
    println!("Server {} is now {}", server, status);
}

/// * `servers` - A reference to a vector containing tuples of server URLs and their availability status.
fn print_servers_summary(servers: &Vec<(String, bool)>) {
    let up_servers: Vec<String> = servers.iter().filter(|(_, alive)| *alive).map(|(server, _)| server.clone()).collect();
    let down_servers: Vec<String> = servers.iter().filter(|(_, alive)| !*alive).map(|(server, _)| server.clone()).collect();

    if !up_servers.is_empty() {
        print!("\rListe des serveurs up {}", colorize(&format!("{:?}", up_servers), TextColor::Green));
    } else {
        print!("\r");
    }
    if !down_servers.is_empty() {
        print!("\rListe des serveurs down {}", colorize(&format!("{:?}", down_servers), TextColor::Red));
    } else {
        print!("\r");
    }
    io::stdout().flush().unwrap(); // Ensures immediate output
}

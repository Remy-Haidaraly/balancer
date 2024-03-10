use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    collections::HashMap,
};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;



use lazy_static::lazy_static;


lazy_static! {
/// Atomic counter to track the total number of requests received by the server.
    pub static ref REQUEST_COUNT: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
/// Mutex-protected map to track the number of requests received by each server.
    pub static ref SERVER_REQUESTS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

/// Function to write server request statistics to a log file
pub fn write_log() {
    let mut file = File::create("fxloadbalancer.log")
        .unwrap_or_else(|err| panic!("Error creating log file : {}", err));

    let requests = REQUEST_COUNT.load(Ordering::Relaxed);

    writeln!(&mut file, "Total number of requests : {}", requests)
        .unwrap_or_else(|err| eprintln!("Error writing to log file : {}", err));

    let server_requests = SERVER_REQUESTS.lock().unwrap();
    for (server, count) in server_requests.iter() {
        writeln!(&mut file, "Number of requests for {} : {}", server, count)
            .unwrap_or_else(|err| eprintln!("Error writing to log file : {}", err));
    }

    println!("Log information has been written to fxloadbalancer.log");
}

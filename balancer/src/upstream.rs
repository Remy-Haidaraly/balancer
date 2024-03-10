use std::sync::Mutex;

lazy_static::lazy_static! {
    /// Mutex-protected vector representing upstream servers along with their availability status.
    pub static ref UPSTREAM_SERVERS: Mutex<Vec<(String, bool)>> = Mutex::new(vec![
        ("http://127.0.0.1:8000".to_string(), true),
        ("http://127.0.0.1:8001".to_string(), true),
    ]);
}

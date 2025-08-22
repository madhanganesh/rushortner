use axum::extract::OriginalUri;

pub fn base62_encode(mut n: u64) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    if n == 0 {
        return "0".to_string();
    }
    let mut buf = Vec::with_capacity(11); // enough for u64 base62
    while n > 0 {
        let idx = (n % 62) as usize;
        buf.push(ALPHABET[idx]);
        n /= 62;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}

pub fn base62_decode(s: &str) -> Option<u64> {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut n: u64 = 0;
    for &b in s.as_bytes() {
        let v = match ALPHABET.iter().position(|&x| x == b) {
            Some(p) => p as u64,
            None => return None,
        };
        n = n * 62 + v;
    }
    Some(n)
}

pub fn get_base_url(original_uri: &OriginalUri) -> String {
    let host = original_uri.0.host().unwrap_or("localhost");
    let scheme = original_uri.0.scheme_str().unwrap_or("http");
    let port_str = if let Some(port) = original_uri.0.port_u16() {
        // Only include port if it's not the default for the scheme
        if (scheme == "http" && port == 80) || (scheme == "https" && port == 443) {
            "".to_string()
        } else {
            format!(":{port}")
        }
    } else {
        "".to_string() // No port specified, assume default for scheme
    };
    format!("{scheme}://{host}{port_str}")
}

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
pub async fn initialize_database() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    eprintln!("Database URL: {db_url}");
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    db
}

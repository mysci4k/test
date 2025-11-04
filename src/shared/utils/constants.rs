use regex::Regex;
use std::{env, sync::LazyLock};

// Server configuration constants
pub static SERVER_ADDRESS: LazyLock<String> =
    LazyLock::new(|| env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()));

pub static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("SERVER_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16 number")
});

// Database configuration constants
pub static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable"));

pub static REDIS_URL: LazyLock<String> =
    LazyLock::new(|| env::var("REDIS_URL").expect("Missing REDIS_URL environment variable"));

// Authentication configuration constants
pub static SESSION_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("SESSION_KEY").expect("Missing SESSION_KEY environment variable"));

// Regular expressions for validation
pub static RE_ONLY_LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\p{L}+$").unwrap());

pub static RE_SPECIAL_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^.*?[@$!%*?&].*$").unwrap());

use std::{env, sync::LazyLock};

pub static SERVER_ADDRESS: LazyLock<String> =
    LazyLock::new(|| env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()));

pub static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("SERVER_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16 number")
});

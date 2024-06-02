use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    load_env_file();

    let ip_address = env::var("IP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", ip_address, port);

    let listener: TcpListener = TcpListener::bind(bind_address).unwrap();
    println!("Server started at http://{}:{}", ip_address, port);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    }
}

fn load_env_file() {
    let env_file_path = ".env";
    match fs::read_to_string(env_file_path) {
        Ok(contents) => {
            for line in contents.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    env::set_var(key.trim(), value.trim());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load .env file: {}", e);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, Rust API!\"}";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
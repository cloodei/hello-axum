#![allow(non_snake_case, unused, unused_imports, dead_code)]

use hello_axum::app;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();

    let mode = &args[1];
    if mode == "redis" {
        println!("Starting server...");
        
        if let Err(e) = app::server::redis().await {
            eprintln!("Error: {}", e);
        }
        println!("Server stopped.");
    }
    else if mode == "postgres" {
        println!("Starting client...");
        
        if let Err(e) = app::server::postgres().await {
            eprintln!("Error: {}", e);
        }
        println!("Server stopped.");
    }
    else if mode == "client" {
        println!("Starting client...");
        
        if let Err(e) = app::client::main().await {
            eprintln!("Error: {}", e);
        }
        println!("Client stopped.");
    }
    else {
        eprintln!("Invalid mode: {}. Use 'client' or 'server'.", mode);
        std::process::exit(1);
    }
}

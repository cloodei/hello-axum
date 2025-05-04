#![allow(non_snake_case, unused, unused_imports, dead_code)]

use hello_axum::app;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();

    let mode = &args[1];
    
    match mode.as_str() {
        "redis" => {
            if let Err(err) = app::redis().await {
                eprintln!("You failed my nga: {}", err);
            }

            println!("Server closed.");
        }
        "sqlx" => {
            if let Err(err) = app::sqlx().await {
                eprintln!("You failed my nga: {}", err);
            }

            println!("Server closed.");
        }
        "postgres" => {
            if let Err(err) = app::tok_postgres().await {
                eprintln!("You failed my nga: {}", err);
            }

            println!("Server closed.");
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'client' or 'redis'/'postgres'.", mode);
            std::process::exit(0xA0);
        }
    }
}

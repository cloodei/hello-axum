use anyhow::{anyhow, Result};
use colored::*;
use hello_axum::prelude::sqlx::{Datas, DatasPayload};
use reqwest::{Client, Method, StatusCode};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, BufReader};

const BASE_URL: &str = "http://127.0.0.1:3000/api";

async fn read_line_prompt(prompt: &str) -> Result<String> {
    print!("{} {}", prompt.cyan(), "> ".cyan());
    io::stdout().flush()?;

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    let line = stdin
        .next_line()
        .await
        .map_err(|e| anyhow!("Failed to read line: {}", e))?;

    match line {
        Some(l) => Ok(l.trim().to_string()),
        None => Err(anyhow!("Input stream closed unexpectedly."))
    }
}

async fn read_usize_prompt(prompt: &str) -> Result<usize> {
    loop {
        let line = read_line_prompt(prompt).await?;

        match line.parse::<usize>() {
            Ok(num) => return Ok(num),
            Err(_) => eprintln!("{}", "Invalid input. Please enter a number.".red())
        }
    }
}

async fn read_int_prompt<T: std::str::FromStr>(prompt: &str) -> Result<T> {
    loop {
        let line = read_line_prompt(prompt).await?;

        match line.parse::<T>() {
            Ok(num) => return Ok(num),
            Err(_) => eprintln!("{}", "Invalid input. Please enter a number.".red())
        }
    }
}

async fn read_confirmation(prompt: &str) -> Result<bool> {
    loop {
        let line = read_line_prompt(&format!("{} (yes/no)", prompt)).await?;

        match line.to_lowercase().as_str() {
            "yes" | "y" => return Ok(true),
            "no"  | "n" => return Ok(false),
            _           => eprintln!("{}", "Invalid input. Please enter 'yes' or 'no'.".red())
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1_000 {
        format!("{} ns", nanos)
    }
    else {
        let time = nanos as f64;

        let micros = time / 1_000.0;
        if micros < 1_000.0 {
            format!("{:3} µs", micros)
        }
        else {
            let millis = micros / 1_000.0;
            
            if millis < 1_000.0 {
                format!("{:3} ms", millis)
            }
            else {
                format!("{:3} s", millis / 1_000.0)
            }
        }
    }
}

async fn handle_get(client: &Client) -> Result<()> {
    println!("{}", "\n--- GET Request ---".blue().bold());
    let mut path = read_line_prompt("Enter path (e.g., /datas or /datas/1)").await?;
    if path == "/" {
        path = "/datas".to_string();
    }
    let url = format!("{}{}", BASE_URL, path);

    println!("{} {} {}...", "Sending".dimmed(), "GET".blue(), url);
    let start_time = Instant::now();
    let res = client.get(url).send().await;
    let elapsed = start_time.elapsed();

    match res {
        Ok(res) => {
            let status = res.status();
            println!("{} {} (took {})", "Received status:".dimmed(), status, format_duration(elapsed)); 

            if status.is_success() {
                let text = res.text().await?;
                if text.is_empty() {
                    println!("{}", "<empty response>".dimmed());
                }
                else {
                    if let Ok(datas) = serde_json::from_str::<Vec<Datas>>(&text) {
                        println!("{} Datas received:", "✅ Success!".green());
                        println!("{}", serde_json::to_string_pretty(&datas)?);
                    }
                    else if let Ok(item) = serde_json::from_str::<Datas>(&text) {
                        println!("{} Data received:", "✅ Success!".green());
                        println!("{}", serde_json::to_string_pretty(&item)?);
                    }
                    else {
                        println!("{} Response body:", "✅ Success!".green());
                        println!("{}", text);
                    }
                }
            }
            else {
                eprintln!("{}: {}", "❌ Error".red(), res.text().await.unwrap_or("Failed to read error body".to_string()));
            }
        }
        Err(e) => eprintln!("{}: {}", "Request failed".red(), e),
    }

    Ok(())
}

async fn handle_post(client: &Client) -> Result<()> {
    println!("{}", "\n--- POST Request ---".green().bold());
    println!("{}", "Enter details for the new item:".green());

    let name = read_line_prompt("Name").await?;
    let flags = read_int_prompt("Flags").await?;
    let sys = read_int_prompt("Sys").await?;

    let payload = DatasPayload { name, flags, sys };

    println!("\n{}", "Payload to be sent:".yellow());
    println!("{}", serde_json::to_string_pretty(&payload)?);

    if !read_confirmation("Confirm sending this POST request?").await? {
        println!("{}", "POST request cancelled.".yellow());
        return Ok(());
    }

    let url = format!("{}/datas", BASE_URL);
    println!("{} {} {}...", "Sending".dimmed(), "POST".green(), url);
    let rq = client.post(&url).json(&payload);
    
    let start_time = Instant::now();
    let res = rq.send().await;
    let elapsed = start_time.elapsed();

    match res {
        Ok(res) => {
            let status = res.status();
            println!("{} {} (took {})", "Received status:".dimmed(), status, format_duration(elapsed)); 

            if status == StatusCode::CREATED {
                match res.json::<Datas>().await {
                    Ok(item) => {
                        println!("{} Item created:", "✅ Success!".green());
                        println!("{}", serde_json::to_string_pretty(&item)?);
                    }
                    Err(_) => {
                        println!("{} Item created, but response body couldn't be parsed as Datas.", "✅ Success!".green());
                    }
                }
            }
            else {
                eprintln!(
                    "{}: {}",
                    "❌ Error".red(),
                    res.text().await.unwrap_or_else(|_| "Failed to read error body".to_string())
                );
            }
        }
        Err(e) => eprintln!("{}: {}", "Request failed".red(), e)
    }
    Ok(())
}

async fn handle_put(client: &Client) -> Result<()> {
    println!("{}", "\n--- PUT Request ---".yellow().bold());
    let id = read_usize_prompt("Enter ID of item to update").await?;
    let url = format!("{}/datas/{}", BASE_URL, id);

    println!("{} Fetching current item data...", "Step 1:".dimmed());
    let get_response = client.get(&url).send().await?;
    let get_status = get_response.status();

    if !get_status.is_success() {
        eprintln!(
            "{}: Could not fetch item {}. Status: {}. Body: {}",
            "❌ Error".red(),
            id,
            get_status,
            get_response.text().await.unwrap_or_default()
        );
        
        return Ok(());
    }

    let mut item = match get_response.json::<Datas>().await {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}: Failed to parse current item data: {}", "❌ Error".red(), e);
            return Ok(());
        }
    };

    println!("{} Current item data:", "Step 2:".dimmed());
    println!("{}", serde_json::to_string_pretty(&item)?);

    println!("{}", "Step 3: Select fields to update (enter field name or 'done'):".dimmed());
    let mut updated = false;
    loop {
        let field = read_line_prompt("Field (name, flags, sys)").await?;

        match field.to_lowercase().as_str() {
            "name" | "n" => {
                item.name = read_line_prompt("New name").await?;
                updated = true;
            }
            "flags" | "f" => {
                item.flags = read_int_prompt("New flags (i64)").await?;
                updated = true;
            }
            "sys" | "s" => {
                item.sys = read_int_prompt("New sys (i16)").await?;
                updated = true;
            }
            "done" | "quit" | "q" => break,
            _ => eprintln!("{}", "Invalid field name.".red()),
        }
    }

    if !updated {
        println!("{}", "No fields were modified. PUT request cancelled.".yellow());
        return Ok(());
    }

    let payload = DatasPayload {
        name: item.name,
        flags: item.flags,
        sys: item.sys
    };

    println!("\n{}", "Updated payload to be sent:".yellow());
    println!("{}", serde_json::to_string_pretty(&payload)?);

    if !read_confirmation("Confirm sending this PUT request?").await? {
        println!("{}", "PUT request cancelled.".yellow());
        return Ok(());
    }

    println!("{} {} {}...", "Sending".dimmed(), "PUT".yellow(), url);
    let rq = client.put(&url).json(&payload);

    let start_time = Instant::now();
    let res = rq.send().await;
    let elapsed = start_time.elapsed();
    
    match res {
        Ok(res) => {
            let status = res.status();
            println!("{} {} (took {})", "Received status:".dimmed(), status, format_duration(elapsed)); 

            if status.is_success() {
                match res.json::<Datas>().await {
                    Ok(updated_item) => {
                        println!("{} Item updated:", "✅ Success!".green());
                        println!("{}", serde_json::to_string_pretty(&updated_item)?);
                    },
                    Err(_) => {
                        println!("{} Item updated, but response body couldn't be parsed as Item.", "✅ Success!".green());
                    }
                }
            }
            else {
                eprintln!(
                    "{}: {}",
                    "❌ Error".red(),
                    res.text().await.unwrap_or_else(|_| "Failed to read error body".to_string())
                );
            }
        }
        Err(e) => eprintln!("{}: {}", "Request failed".red(), e)
    }

    Ok(())
}

async fn handle_delete(client: &Client) -> Result<()> {
    println!("{}", "\n--- DELETE Request ---".red().bold());
    let id = read_usize_prompt("Enter ID of item to delete").await?;
    let url = format!("{}/datas/{}", BASE_URL, id);

    if !read_confirmation(&format!("Confirm deleting item {}?", id)).await? {
        println!("{}", "DELETE request cancelled.".yellow());
        return Ok(());
    }

    println!("{} {} {}...", "Sending".dimmed(), "DELETE".red(), url);
    let start_time = Instant::now();
    match client.delete(&url).send().await {
        Ok(res) => {
            let elapsed = start_time.elapsed();
            let status = res.status();
            println!("{} {} (took {})", "Received status:".dimmed(), status, format_duration(elapsed)); 

            if status == StatusCode::NO_CONTENT {
                println!("{} Item {} deleted successfully.", "✅ Success!".green(), id);
            }
            else if status == StatusCode::NOT_FOUND {
                eprintln!("{}: Item {} not found.", "❌ Error".red(), id);
            }
            else {
                eprintln!(
                    "{}: Status {}. {}",
                    "❌ Error".red(),
                    status,
                    res.text().await.unwrap_or_else(|_| "Failed to read error body".to_string())
                );
            }
        }
        Err(e) => eprintln!("{}: {}", "Request failed".red(), e),
    }

    Ok(())
}

async fn prompt_for_method() -> Result<Option<Method>> {
    println!("\n{}", "Select action: [1] GET, [2] POST, [3] PUT, [4] DELETE, [EXIT]".bold());

    loop {
        let line = read_line_prompt("Action").await?;
        let method = match line.to_lowercase().as_str() {
            "1" | "get"           => Some(Method::GET),
            "2" | "post"          => Some(Method::POST),
            "3" | "put"           => Some(Method::PUT),
            "4" | "delete"        => Some(Method::DELETE),
            "exit" | "quit" | "q" => None,
            _ => {
                eprintln!("{}", "Invalid action. Please enter a number (1-4), method name, or EXIT.".red());
                continue;
            }
        };
        
        return Ok(method);
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = Client::new();
    println!("{}", "Client Started".bold().cyan());
    println!("{} {}", "Base URL:".dimmed(), BASE_URL);

    loop {
        match prompt_for_method().await? {
            Some(Method::GET) => handle_get(&client).await?,
            Some(Method::POST) => handle_post(&client).await?,
            Some(Method::PUT) => handle_put(&client).await?,
            Some(Method::DELETE) => handle_delete(&client).await?,
            Some(_) => unreachable!(),
            None => {
                println!("{}", "Exiting client.".yellow());
                break;
            }
        }
    }

    Ok(())
}

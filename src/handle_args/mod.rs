use std::{error::Error, process, str::FromStr};

use reqwest::{Client, StatusCode};
use serde_json::Value;

pub async fn args_handler() -> Result<(Value, usize, StatusCode), Box<dyn Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut long: usize = 5;
    args.remove(0);

    if args.len() == 0 {
        show_helps();
        process::exit(1);
    }

    //counting for options
    let mut total_options = 0;
    for i in &args {
        if i.as_bytes()[0] == b'-' {
            total_options += 1;
        }
    }

    //check for multiple options
    if total_options > 1 {
        return Err(Box::from("cannot using multiple options"));
    }

    let url = format!("https://api.github.com/users/{}/events", args[0]);

    let client = Client::new()
        .request(reqwest::Method::GET, reqwest::Url::from_str(url.as_str())?)
        .header("User-Agent", "gtvt");

    if let Some(value) = args.get(1) {
        if let Err(_) = value.parse::<usize>() {
            println!("argumnet of [n] must be a positive number");
            process::exit(1);
        } else if let Ok(value) = value.parse::<usize>() {
            long = value;
        }
    }

    let response =client.send().await?; 
    let status = response.status();

    return Ok((response.json::<Value>().await?, long, status));
}

fn show_helps() {
    println!("gtvt [user] [n(default = 5)]");
    println!("show [n] last activity of [user]");
}
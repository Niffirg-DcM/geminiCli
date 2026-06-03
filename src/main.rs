use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::io;
use dotenvy::dotenv;

//incoming payloads with deserialize
#[derive(Deserialize, Debug)]
struct in_candidates {
    content: Vec<in_contents>,
}

#[derive(Deserialize, Debug)]
struct in_contents {
    part: Vec<in_parts>,
}

#[derive(Deserialize, Debug)]
struct in_parts {
    data: Vec<String>,
}



//outgoing payloads with serialize
#[derive(Serialize)]
struct out_candidates {
    content: Vec<out_contents>,
}

#[derive(Serialize)]
struct out_contents {
    part: Vec<out_parts>,
}

#[derive(Serialize)]
struct out_parts {
    data: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    
    let key = match env::var("GEMINI_API_KEY") {
        Ok(val) => println!("key recieved"),
        Err(e) => eprintln!("Error reading gemini api key"),
    };

    println!("Hello, world!");
    println!("Exit anytime with 'exit'");
    
    //("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",apiKey.trim()))
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let get_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",apiKey.trim());
    
    let response = client.get(get_url).send().await?;

    let post_url = "https://typicode.com";
    let payload = out_candidates {
        content: vec![ out_contents {
            part: vec![ out_parts {
                data: vec![String::from("fail")]}
                ]
            } ]
    };

    let post_response = client.post(post_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if post_response.status().is_success() {
        println!("POST Success! Resource created.");
    }

    Ok(())
    
}

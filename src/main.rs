use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::io;
use dotenvy::dotenv;

//incoming payloads with deserialize
#[derive(Deserialize, Debug)]
struct inCandidates {
    content: Vec<inContents>,
}

#[derive(Deserialize, Debug)]
struct inContents {
    part: Vec<inParts>,
}

#[derive(Deserialize, Debug)]
struct inParts {
    data: Vec<String>,
}



//outgoing payloads with serialize
#[derive(Serialize)]
struct outCandidates {
    contents: Vec<outContents>,
}

#[derive(Serialize)]
struct outContents {
    parts: Vec<outParts>,
}

#[derive(Serialize)]
struct outParts {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    
    let key = match env::var("GEMINI_API_KEY") {
        Ok(val) => val,
        Err(e) => String::from("Error"),
    };

    if(key.eq("Error")) {
        println!("no Key");
    }



    println!("Hello, world!");
    println!("Exit anytime with 'exit'");

    println!("Please enter your query: ");
    let mut data_to_send = String::new();
    io::stdin().read_line(&mut data_to_send).expect("damn");


    //("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",apiKey.trim()))
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",key);


    let payload = outCandidates {
        contents: vec![ outContents {
            parts: vec![ outParts {
                text: String::from("fail")}
                ]
            } ]
    };

    let post_response:inCandidates = client.post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if post_response.status().is_success() {
        println!("POST Success! Resource created.");
        println!("{}",);
    } else {
        println!("Error Code: {}",post_response.status().to_string());
    }

    Ok(())
    
}

use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::io;
use dotenvy::dotenv;

//incoming payloads with deserialize
#[derive(Deserialize, Debug)]
struct inCandidates {
    content: inContents,
}

#[derive(Deserialize, Debug)]
struct inContents {
    parts: Vec<inParts>,
    role: String,
}

#[derive(Deserialize, Debug)]
struct inParts {
    text: String,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<inCandidates>,
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
    loop {
        println!("Please enter your query: ");
        let mut data_to_send = String::new();
        io::stdin().read_line(&mut data_to_send).expect("damn");
        if (data_to_send.trim()=="exit") {break;}

        //("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",apiKey.trim()))
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",key);


        let payload = outCandidates {
            contents: vec![ outContents {
                parts: vec![ outParts {
                    text: data_to_send}
                    ]
                } ]
        };

        let post_response:Response = client.post(url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;



        if post_response.status().is_success() {
            println!("POST Success! Resource created.");
            let parsed_response: GeminiResponse = post_response.json().await?;
            println!("Response is: {}",parsed_response.candidates[0].content.parts[0].text);
            
        } else {
            println!("Error Code: {}",post_response.status().to_string());
        }
    }

    Ok(())
    
}

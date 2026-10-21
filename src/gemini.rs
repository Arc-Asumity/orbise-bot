// ==============================================================
// Copyright (c) 2026 PhiARC
// Licensed under the MIT.
// https://github.com/Arc-Asumity/orbise-bot
// ==============================================================
// Path /src/gemini.rs
// Module of Gemini API

use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn ask(text: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let api_key = std::env::var("GEMINI_API_KEY")?;

    let response = Client::new()
        .post(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash-lite:generateContent",
        )
        .header("x-goog-api-key", api_key)
        .json(&json!({
            "contents": [{
                "parts": [{
                    "text": text
                }]
            }]
        }))
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = response.json().await?;

    let answer = body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Gemini response did not contain text")?
        .to_owned();

    Ok(answer)
}


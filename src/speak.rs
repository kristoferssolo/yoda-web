use std::sync::Arc;

use anyhow::{Context, Result};
use serde_json::{json, Value};

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tracing_log::log::error;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct InputText {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct OutputText {
    pub yoda_text: String,
}

pub async fn yoda_speak(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InputText>,
) -> impl IntoResponse {
    match convert_to_yoda_speak(&state.client, &input.text).await {
        Ok(yoda_text) => {
            let output = OutputText { yoda_text };
            (StatusCode::OK, Json(output)).into_response()
        }
        Err(e) => {
            error!("Error converting to Yoda speak: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":"Failed to convert text"})),
            )
                .into_response()
        }
    }
}

async fn convert_to_yoda_speak(client: &Client, text: &str) -> Result<String> {
    let url = "https://api.funtranslations.com/translate/yoda.json";
    let params = [("text", text)];
    let response = client
        .post(url)
        .form(&params)
        .send()
        .await
        .context("Failed to send request to Yoda translation API")?;
    let json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    let translated_text = json
        .get("contents")
        .context("Missing 'contents' key in JSON response")?
        .get("translated")
        .context("Missing 'translated' key in 'contents' object")?
        .as_str()
        .context("Failed to extract translated text from JSON")?
        .to_owned();
    Ok(translated_text)
}

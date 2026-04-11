use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
    pub created_at: String,
    pub done: bool,
}

pub struct SovereignInference {
    client: Client,
    model: String,
}

impl SovereignInference {
    pub fn new(model: &str) -> Self {
        SovereignInference {
            client: Client::new(),
            model: model.to_string(),
        }
    }

    pub async fn process(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client.post("http://localhost:11434/api/generate")
            .json(&request_body)
            .send()
            .await?;

        let body: OllamaResponse = response.json().await?;
        Ok(body.response)
    }
}

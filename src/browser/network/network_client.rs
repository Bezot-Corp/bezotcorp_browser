use reqwest::Client;
use tokio::sync::mpsc;

use crate::browser::network::{NetworkRequest, NetworkResponse};

const USER_AGENT: &str = "BezotCorp Browser/0.1 (bcb; Rust)";
const REQUEST_TIMEOUT_SECS: u64 = 15;

pub(crate) struct NetworkClient {
    client: Client,
    sender: mpsc::Sender<NetworkResponse>,
}

impl NetworkClient {
    pub(crate) fn new(sender: mpsc::Sender<NetworkResponse>) -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .gzip(true)
            .brotli(true)
            .build()
            .unwrap_or_default();
        Self { client, sender }
    }

    pub(crate) fn fetch(&self, request: NetworkRequest) {
        let client = self.client.clone();
        let sender = self.sender.clone();
        tokio::spawn(async move {
            let response = Self::execute(&client, &request).await;
            let _ = sender.send(response).await;
        });
    }

    async fn execute(client: &Client, request: &NetworkRequest) -> NetworkResponse {
        match client.get(&request.url).send().await {
            Ok(response) => {
                let url = response.url().to_string();
                let content_type = response
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();

                if !content_type.contains("text/html") && !content_type.contains("text/plain") {
                    return NetworkResponse::Error {
                        request_id: request.request_id,
                        url,
                        reason: format!("Type de contenu non supporté : {content_type}"),
                    };
                }

                match response.text().await {
                    Ok(html) => NetworkResponse::Html {
                        request_id: request.request_id,
                        url,
                        html,
                    },
                    Err(e) => NetworkResponse::Error {
                        request_id: request.request_id,
                        url: request.url.clone(),
                        reason: e.to_string(),
                    },
                }
            }
            Err(e) => NetworkResponse::Error {
                request_id: request.request_id,
                url: request.url.clone(),
                reason: e.to_string(),
            },
        }
    }
}

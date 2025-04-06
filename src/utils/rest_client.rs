use actix_web::http::header::{HeaderName, HeaderValue};
use awc::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::to_string;
use std::fmt::Debug;
use std::time::Instant;
use tracing::log::info;

#[derive(Clone)]
pub struct RestClient {
    client: Client,
}

impl RestClient {
    // Constructor to initialize the struct
    pub fn new() -> Self {
        RestClient {
            client: Client::default(),
        }
    }

    // Generic method for making POST requests
    // Generic method for making POST requests with headers
    pub async fn post_json<T: DeserializeOwned + Debug, U: Serialize>(
        &self,
        url: &str,
        payload: &U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T, String> {
        let start_time = Instant::now(); // Start timing the request

        let mut request = self
            .client
            .post(url)
            .insert_header(("Content-Type", "application/json"));

        // Add custom headers if provided
        if let Some(headers) = headers {
            for (key, value) in headers {
                if let (Ok(header_name), Ok(header_value)) =
                    (HeaderName::try_from(key), HeaderValue::try_from(value))
                {
                    request = request.insert_header((header_name, header_value));
                }
            }
        }

        let mut response = request
            .send_json(payload)
            .await
            .map_err(|e| e.to_string())?;

        let duration = start_time.elapsed(); // Measure elapsed time
        info!(
            "POST request to {} completed in {:.2?} seconds",
            url,
            duration.as_secs_f64()
        );

        info!("POST Body: {}", to_string(&payload).unwrap_or_else(|_| "Failed to serialize payload".to_string()));

        if response.status().is_success() {
            let body = response.json::<T>().await.map_err(|e| e.to_string())?;
            info!("Response: {:?}", body);
            Ok(body)
        } else {
            Err(format!(
                "Request failed: {} - {:?}",
                response.status(),
                response
                    .body()
                    .await
                    .unwrap_or_else(|_| "Unknown error".into())
            ))
        }

    }


    pub async fn get_json<T: DeserializeOwned + Debug>(
        &self,
        url: &str,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T, String> {
        let start_time = Instant::now(); // Start timing the request

        let mut request = self.client.get(url).append_header(("Content-Type", "application/json"));

        // Add custom headers if provided
        if let Some(headers) = headers {
            for (key, value) in headers {
                if let (Ok(header_name), Ok(header_value)) = (HeaderName::try_from(key), HeaderValue::try_from(value)) {
                    request = request.append_header((header_name, header_value));
                }
            }
        }

        let mut response = request.send().await.map_err(|e| e.to_string())?;

        let duration = start_time.elapsed(); // Measure elapsed time
        info!(
        "GET request to {} completed in {:.2?} seconds",
        url,
        duration.as_secs_f64()
    );
        info!("Response: {:?}", response.status());

        if response.status().is_success() {
            let body = response.json::<T>().await.map_err(|e| e.to_string())?;

            Ok(body)
        } else {
            let error_body = response.body().await
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(format!("Request failed: {} - {}", response.status(), error_body))
        }
    }
}

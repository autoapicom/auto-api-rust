use std::collections::HashMap;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;

use crate::error::Error;
use crate::types::*;

/// Client for the auto-api.com car listings API.
pub struct Client {
    api_key: String,
    base_url: String,
    api_version: String,
    http_client: reqwest::Client,
}

impl Client {
    /// Creates a new API client with the given API key.
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: "https://api1.auto-api.com".to_string(),
            api_version: "v2".to_string(),
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    /// Sets a custom base URL.
    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.trim_end_matches('/').to_string();
    }

    /// Sets a custom API version (default: "v2").
    pub fn set_api_version(&mut self, version: &str) {
        self.api_version = version.to_string();
    }

    /// Returns available filters for a source (brands, models, body types, etc.)
    pub async fn get_filters(&self, source: &str) -> Result<Value, Error> {
        let url = format!(
            "{}/api/{}/{}/filters",
            self.base_url, self.api_version, source
        );
        self.get(&url, &[]).await
    }

    /// Returns a paginated list of offers with optional filters.
    pub async fn get_offers(
        &self,
        source: &str,
        params: &OffersParams,
    ) -> Result<OffersResponse, Error> {
        let url = format!(
            "{}/api/{}/{}/offers",
            self.base_url, self.api_version, source
        );
        let pairs = params.to_query_pairs();
        let query: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        self.get(&url, &query).await
    }

    /// Returns a single offer by inner_id.
    pub async fn get_offer(
        &self,
        source: &str,
        inner_id: &str,
    ) -> Result<OffersResponse, Error> {
        let url = format!(
            "{}/api/{}/{}/offer",
            self.base_url, self.api_version, source
        );
        self.get(&url, &[("inner_id", inner_id)]).await
    }

    /// Returns a change_id for the given date (format: yyyy-mm-dd).
    pub async fn get_change_id(&self, source: &str, date: &str) -> Result<i64, Error> {
        let url = format!(
            "{}/api/{}/{}/change_id",
            self.base_url, self.api_version, source
        );
        let result: ChangeIdResponse = self.get(&url, &[("date", date)]).await?;
        Ok(result.change_id)
    }

    /// Returns a changes feed (added/changed/removed) starting from change_id.
    pub async fn get_changes(
        &self,
        source: &str,
        change_id: i64,
    ) -> Result<ChangesResponse, Error> {
        let url = format!(
            "{}/api/{}/{}/changes",
            self.base_url, self.api_version, source
        );
        let change_id_str = change_id.to_string();
        self.get(&url, &[("change_id", &change_id_str)]).await
    }

    /// Returns offer data by its URL on the marketplace.
    /// Uses POST /api/v1/offer/info with x-api-key header.
    pub async fn get_offer_by_url(&self, offer_url: &str) -> Result<Value, Error> {
        let url = format!("{}/api/v1/offer/info", self.base_url);

        let mut body = HashMap::new();
        body.insert("url", offer_url);

        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        query: &[(&str, &str)],
    ) -> Result<T, Error> {
        let mut all_query: Vec<(&str, &str)> = query.to_vec();
        all_query.push(("api_key", &self.api_key));

        let response = self
            .http_client
            .get(url)
            .query(&all_query)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, Error> {
        let status = response.status().as_u16();
        let body = response.text().await?;

        if status < 200 || status >= 300 {
            let mut message = format!("API error: {}", status);

            if let Ok(parsed) = serde_json::from_str::<Value>(&body) {
                if let Some(msg) = parsed.get("message").and_then(|m| m.as_str()) {
                    message = msg.to_string();
                }
            }

            if status == 401 || status == 403 {
                return Err(Error::Auth {
                    status_code: status,
                    message,
                });
            }

            return Err(Error::Api {
                status_code: status,
                message,
                body,
            });
        }

        serde_json::from_str(&body).map_err(|_| Error::Api {
            status_code: status,
            message: format!(
                "Invalid JSON response: {}",
                &body[..body.len().min(200)]
            ),
            body,
        })
    }
}

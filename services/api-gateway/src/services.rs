use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub struct ServiceClient {
    client: Client,
    nutrition_service_url: String,
    analytics_service_url: String,
}

impl ServiceClient {
    pub fn new() -> Self {
        let nutrition_service_url = env::var("NUTRITION_SERVICE_URL")
            .unwrap_or_else(|_| "http://nutrition-service:8081".to_string());
        
        let analytics_service_url = env::var("ANALYTICS_SERVICE_URL")
            .unwrap_or_else(|_| "http://analytics-service:8082".to_string());

        Self {
            client: Client::new(),
            nutrition_service_url,
            analytics_service_url,
        }
    }

    pub async fn analyze_nutrition(&self, data: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/analyze", self.nutrition_service_url);
        let response = self.client
            .post(&url)
            .json(data)
            .send()
            .await?;
        
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn get_recommendations(&self, user_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/recommendations/{}", self.nutrition_service_url, user_id);
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn log_analytics(&self, event: &serde_json::Value) -> Result<()> {
        let url = format!("{}/events", self.analytics_service_url);
        self.client
            .post(&url)
            .json(event)
            .send()
            .await?;
        
        Ok(())
    }
}

pub struct ServiceOrchestrator {
    // TODO: Implement service orchestration
}

impl ServiceOrchestrator {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }
}

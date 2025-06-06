use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ServiceOrchestrator {
    client: Client,
    service_urls: Arc<RwLock<HashMap<String, String>>>,
}

impl ServiceOrchestrator {
    pub async fn new() -> Result<Self> {
        let mut service_urls = HashMap::new();

        // Load service URLs from environment variables
        service_urls.insert(
            "nutrition".to_string(),
            std::env::var("NUTRITION_SERVICE_URL")
                .unwrap_or_else(|_| "http://nutrition-service:8081".to_string()),
        );

        service_urls.insert(
            "analytics".to_string(),
            std::env::var("ANALYTICS_SERVICE_URL")
                .unwrap_or_else(|_| "http://analytics-service:8082".to_string()),
        );

        service_urls.insert(
            "recipe-import".to_string(),
            std::env::var("RECIPE_IMPORT_SERVICE_URL")
                .unwrap_or_else(|_| "http://recipe-import-service:8083".to_string()),
        );

        Ok(Self {
            client: Client::new(),
            service_urls: Arc::new(RwLock::new(service_urls)),
        })
    }

    pub async fn call_nutrition_service(
        &self,
        endpoint: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let urls = self.service_urls.read().await;
        let base_url = urls
            .get("nutrition")
            .ok_or_else(|| anyhow::anyhow!("Nutrition service URL not found"))?;
        let url = format!("{}{}", base_url, endpoint);

        let mut request = self.client.get(&url);

        if let Some(body) = body {
            request = self.client.post(&url).json(&body);
        }

        let response = request.send().await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn call_analytics_service(
        &self,
        endpoint: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let urls = self.service_urls.read().await;
        let base_url = urls
            .get("analytics")
            .ok_or_else(|| anyhow::anyhow!("Analytics service URL not found"))?;
        let url = format!("{}{}", base_url, endpoint);

        let mut request = self.client.get(&url);

        if let Some(body) = body {
            request = self.client.post(&url).json(&body);
        }

        let response = request.send().await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn call_recipe_import_service(
        &self,
        endpoint: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let urls = self.service_urls.read().await;
        let base_url = urls
            .get("recipe-import")
            .ok_or_else(|| anyhow::anyhow!("Recipe import service URL not found"))?;
        let url = format!("{}{}", base_url, endpoint);

        let mut request = self.client.get(&url);

        if let Some(body) = body {
            request = self.client.post(&url).json(&body);
        }

        let response = request.send().await?;
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn health_check(&self) -> Result<HashMap<String, bool>> {
        let mut health_status = HashMap::new();
        let urls = self.service_urls.read().await;

        for (service, url) in urls.iter() {
            let health_url = format!("{}/health", url);
            let is_healthy = self
                .client
                .get(&health_url)
                .send()
                .await
                .map(|resp| resp.status().is_success())
                .unwrap_or(false);

            health_status.insert(service.clone(), is_healthy);
        }

        Ok(health_status)
    }
}

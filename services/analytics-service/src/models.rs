use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendData {
    pub trend_type: String,
    pub value: f64,
}

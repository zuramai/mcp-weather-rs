use rmcp::{handler::server::{tool::{Parameters, ToolRouter}}, model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, tool, tool_handler, tool_router, ServerHandler};

use crate::{model::{self, PointRequest, WeatherResponse}, util};

#[derive(Clone)]
pub struct WeatherService {
    client: reqwest::Client,
    tool_router: ToolRouter<WeatherService>,
}

#[tool_router]
impl WeatherService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { 
            client: reqwest::ClientBuilder::new()
                .build()
                .expect("Failed to create HTTP client"),
            tool_router: Self::tool_router()
        }
    }

    #[tool(description = "Get weather forecast using latitude and longitude coordinates")]
    pub async fn get_forecast(
        &self, 
        Parameters(PointRequest { latitude, longitude }): Parameters<PointRequest> 
    ) -> Result<CallToolResult, rmcp::Error> {
        match self.make_request(PointRequest {
            latitude,
            longitude
        }).await {
            Ok(v) => {
                Ok(CallToolResult::success(vec![
                    Content::text(util::format_forecast(v))
                ]))
            },
            Err(e) => {
                tracing::error!("Failed to fetch forecast: {}", e);
                Ok(CallToolResult::error(vec![
                    Content::text("No forecast found or an error occured".to_string())
                ]))
            }
        }
    }
    
    pub async fn make_request(&self, point: model::PointRequest) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m",
            point.latitude, point.longitude
        );
        
        let response = self.client.get(&url).send().await?;
        let weather_data = response.json::<WeatherResponse>().await?;

        Ok(weather_data)
    }
}

#[tool_handler]
impl ServerHandler for WeatherService {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some("A simple weather forecaster".to_string()),
            ..Default::default()
        }
    }
}
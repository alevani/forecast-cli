use anyhow::anyhow;
use dotenv::dotenv;
use hyper::client::HttpConnector;
use hyper::http::Error;
use hyper::{body as BodyParser, StatusCode};
use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
type HttpsClient = Client<HttpsConnector<HttpConnector>>;
type HyperResult = color_eyre::eyre::Result<Response<Body>, Error>;
type GenericResult<T> = color_eyre::eyre::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use color_eyre::eyre::Result;
use log::warn;

pub mod forecastapp_models;
use forecastapp_models::*;

#[derive(Clone)]
pub struct ForecastAppApi {
    pub https_client: HttpsClient,
    pub api_key: String,
    pub base_url: String,
    pub redis_cache_key: String,
}

impl ForecastAppApi {
    pub fn new() -> ForecastAppApi {
        dotenv().ok();

        // Init HTTP client
        let https = HttpsConnector::new();
        let https_client = Client::builder().build(https);

        let forecast_api_key = std::env::var("FORECAST_API_KEY").unwrap();

        ForecastAppApi {
            https_client,
            api_key: forecast_api_key,
            base_url: String::from("https://api.forecast.it/api"),
            redis_cache_key: String::from("ACTIVE_PERSONS_FORECAST"),
        }
    }

    fn build_request(
        &self,
        method: Method,
        uri: String,
        body: Body,
    ) -> Result<Request<Body>, hyper::http::Error> {
        Request::builder()
            .method(method)
            .uri(uri)
            .header("X-FORECAST-API-KEY", format!("{}", self.api_key))
            .header("content-type", "application/json")
            .body(body)
    }

    async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        req: Request<Body>,
    ) -> Result<T, anyhow::Error> {
        let http = &self.https_client;
        let raw_content = http.request(req).await?;
        parse_content(raw_content).await
    }

    pub async fn get_task_id_by_company_task_id(
        &self,
        company_task_id: i32,
    ) -> anyhow::Result<Task> {
        let uri = format!(
            "{}/v3/tasks/company_task_id/{company_task_id}",
            self.base_url
        );

        let req = self.build_request(Method::GET, uri, Body::empty())?;
        self.execute_request(req).await
    }

    pub async fn send_time_registration(
        &self,
        time_registration: TimeRegistrationBody,
    ) -> GenericResult<TimeRegistrationResponse> {
        let http = &self.https_client;

        let uri = format!("{}/v1/time_registrations", self.base_url);
        let parsed_body = serde_json::to_string_pretty(&time_registration)?;

        println!("{parsed_body:?}");

        let req = self.build_request(Method::POST, uri, Body::from(parsed_body))?;
        let response = http.request(req).await?;

        let b = parse_content::<RequestError>(response).await?;
        println!("{b:?}");

        Ok(TimeRegistrationResponse {
            recipient_id: time_registration.person,
            status_code: StatusCode::OK,
        })
    }
}

pub async fn parse_content<T: serde::de::DeserializeOwned>(
    response: Response<Body>,
) -> Result<T, anyhow::Error> {
    match BodyParser::to_bytes(response.into_body()).await {
        Ok(content) => match serde_json::from_slice::<T>(&content) {
            Ok(parsed_content) => Ok(parsed_content),
            Err(err) => {
                // println!("{content:?}");
                warn!("{err}");
                Err(anyhow!(err))
            }
        },
        Err(err) => Err(anyhow!(err)),
    }
}

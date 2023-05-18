use std::{fmt, time::Duration};

use reqwest::{Client, StatusCode};

use domain::types::IfconfigData;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(Box<dyn std::error::Error + Send + Sync>),
    Timeout,
    TryAgain,
}

pub struct App {
    client: Client,
}

impl App {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(concat!("ifconfig/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn get(&self) -> Result<IfconfigData> {
        let data = self
            .client
            .get("https://ifconfig.co/json")
            .send()
            .await?
            .json::<IfconfigData>()
            .await?;

        Ok(data)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(err) => write!(f, "error: {:?}", err.as_ref()),
            Error::Timeout => write!(f, "timeout"),
            Error::TryAgain => write!(f, "try again"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            return match status {
                StatusCode::REQUEST_TIMEOUT => Error::Timeout,
                StatusCode::TOO_MANY_REQUESTS => Error::TryAgain,
                _ => Error::Custom(Box::new(err)),
            };
        }

        Error::Custom(Box::new(err))
    }
}

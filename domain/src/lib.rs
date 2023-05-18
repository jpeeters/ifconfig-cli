use async_trait::async_trait;

pub mod error;
pub mod result;
pub mod types;

#[async_trait]
pub trait Fetcher {
    async fn get(&self) -> result::Result<types::IfconfigData>;
}

use domain::{Fetcher, types::IfconfigData, result::Result};

pub struct App<F> {
    fetcher: F,
}

impl<F: Fetcher> App<F> {
    pub fn build(f: F) -> Self {
        Self { fetcher: f }
    }

    pub async fn get(&self) -> Result<IfconfigData> {
        self.fetcher.get().await
    }
}

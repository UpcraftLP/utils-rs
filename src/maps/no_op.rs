use crate::maps::{Location, MapsProvider};
use async_trait::async_trait;

pub(crate) struct NoOpMapsProvider {}

#[async_trait]
impl MapsProvider for NoOpMapsProvider {
    async fn get_location(&self, _query: &str) -> anyhow::Result<Option<Location>> {
        Ok(None)
    }
}

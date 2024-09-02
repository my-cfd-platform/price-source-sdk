use std::sync::Arc;

use service_sdk::rust_extensions::MyTimerTick;
use tokio::sync::Mutex;

use crate::PriceSourceBridgeStats;

pub struct PriceSourceStatsTimer {
    stats: Arc<Mutex<PriceSourceBridgeStats>>,
}

#[async_trait::async_trait]
impl MyTimerTick for PriceSourceStatsTimer {
    async fn tick(&self) {
        let mut stats = self.stats.lock().await;
        stats.write_as_metrics();
    }
}

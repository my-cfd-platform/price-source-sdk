use std::collections::HashMap;

use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Default)]
pub struct PriceSourceBridgeStats {
    pub incoming_messages_count: u128,
    pub prices_count: HashMap<String, u128>,
    pub prices_timeout_count_ms: HashMap<String, u128>,
}

impl PriceSourceBridgeStats {
    pub fn track_incoming_message(&mut self) {
        self.incoming_messages_count += 1;
    }

    pub fn handle_new_price(&mut self, id: &str, date: DateTimeAsMicroseconds) {
        let change_millis = (DateTimeAsMicroseconds::now() - date)
            .as_positive_or_zero()
            .as_millis();

        self.prices_count
            .entry(id.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);

        self.prices_timeout_count_ms
            .entry(id.to_string())
            .and_modify(|e| *e += change_millis)
            .or_insert(change_millis);
    }

    pub fn write_as_metrics(&mut self) {
        service_sdk::metrics::gauge!("price_bridge_incoming_messages_count").increment(0);
        self.incoming_messages_count = 0;

        for (key, value) in &self.prices_count {
            service_sdk::metrics::gauge!("price_bridge_prices_count", "instrument_id" => key.to_string())
                .increment(*value as f64);
        }
        self.prices_count.clear();

        for (key, value) in &self.prices_timeout_count_ms {
            service_sdk::metrics::gauge!("price_bridge_prices_timeout_count_ms", "instrument_id" => key.to_string())
                .increment(*value as f64);
        }
        self.prices_timeout_count_ms.clear();
    }

    pub fn clean(&mut self) {
        self.incoming_messages_count = 0;
        self.prices_count.clear();
        self.prices_timeout_count_ms.clear();
    }
}

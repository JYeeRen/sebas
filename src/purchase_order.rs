use crate::spec::Spec;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub channel: String,
    pub brand: String,
    pub spec: Spec,
    pub qty: Decimal,
    pub price: Decimal,
    pub purchase_date: DateTime<Utc>,
}

impl PurchaseOrder {
    pub fn new(channel: String, brand: String, spec: Spec, qty: Decimal, price: Decimal, purchase_date: DateTime<Utc>) -> Self {
        Self {
            channel,
            brand,
            spec,
            qty,
            price,
            purchase_date,
        }
    }

    pub fn price_per_unit(&self) -> Decimal {
        self.price / self.spec.qty().round_dp(8)
    }
}

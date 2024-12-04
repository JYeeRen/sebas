use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Deserialize)]
pub struct SpecItem {
    unit: String,
    qty: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    items: Vec<SpecItem>,
}

impl Default for Spec {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Spec {
    pub fn new(items: Vec<(Decimal, String)>) -> Self {
        let items = items
            .into_iter()
            .map(|(qty, unit)| SpecItem { qty, unit })
            .collect();

        Self { items }
    }

    pub fn qty(&self) -> Decimal {
        self.items.iter().map(|item| item.qty).product()
    }
}

impl Serialize for Spec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let spec: String = self
            .items
            .iter()
            .map(|item| format!("{}{}", item.qty, item.unit))
            .collect::<Vec<String>>()
            .join("/");
        serializer.serialize_str(&spec)
    }
}

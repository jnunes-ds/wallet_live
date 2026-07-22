use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use time::OffsetDateTime;

pub fn format_brl(val: f64) -> String {
    let val_abs = val.abs();
    let formatted = format!("{:.2}", val_abs);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let fractional_part = parts[1];

    let mut formatted_integer = String::new();
    let chars: Vec<char> = integer_part.chars().rev().collect();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_integer.push('.');
        }
        formatted_integer.push(*c);
    }
    let integer_formatted: String = formatted_integer.chars().rev().collect();

    if val < 0.0 {
        format!("-R$ {},{}", integer_formatted, fractional_part)
    } else {
        format!("R$ {},{}", integer_formatted, fractional_part)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    pub bought_for: f64,
    pub quantity_bought: f64,
    pub value_delta: f64,
}

impl PurchaseHistory {
    pub fn format_bought_for(&self) -> String {
        format_brl(self.bought_for)
    }

    pub fn format_value_delta(&self) -> String {
        format_brl(self.value_delta)
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<PurchaseHistory>>,
}

impl OwnedAsset {
    pub fn format_unit_value(&self) -> String {
        format_brl(self.unit_value)
    }

    pub fn format_value_delta(&self) -> String {
        format_brl(self.value_delta)
    }
}

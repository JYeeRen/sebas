use rust_decimal::Decimal;
use std::str::FromStr;

pub fn dec(input: &str) -> Decimal {
  Decimal::from_str(input).expect("Invalid decimal format")
}
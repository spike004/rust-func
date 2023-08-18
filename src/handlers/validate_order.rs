use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[wasm_bindgen]
pub fn validate_order(order_json: &str) -> bool {
    let order: Order = serde_json::from_str(order_json).unwrap();

    order.value <= 10000.0
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    value: f64
}
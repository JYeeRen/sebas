mod purchase_order;
mod spec;
mod utils;

use chrono::Utc;
use purchase_order::PurchaseOrder;
use spec::Spec;
use utils::dec;

fn main() {
    println!("Hello, Sebas!");
    
    let channel = "淘宝";
    let brand = "九三";

    let spec = Spec::new(vec![
        (dec(&6.18.to_string()), "升".to_string()),
        (dec(&5.to_string()), "桶".to_string()),
        (dec(&1.to_string()), "箱".to_string()),
    ]);

    let po = PurchaseOrder::new(
        channel.to_string(),
        brand.to_string(),
        spec,
        dec(&1.to_string()),
        dec(&147.82.to_string()),
        Utc::now()
    );

    println!("Serialized Spec: {:#?}", serde_json::to_value(&po).unwrap());

    println!("price_per_unit: {}", po.price_per_unit());
}

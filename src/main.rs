extern crate serde_json;

use serde_json::{json, Result, Value};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let mut file = File::open("test_broadcast.json").unwrap();
    // Read the contents of the file into a string.
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&contents)?;
    // println!("{:?}", v);

    // Manipulate the data as needed
    let mut events = Vec::new();
    for txn in v["transactions"].as_array().unwrap() {
        let event = match txn["transactionType"].as_str().unwrap() {
            "CREATE" => "ContractCreated",
            "CALL" => "ContractCalled",
            _ => "Unknown",
        };
        let address = txn["contractAddress"].as_str().unwrap();
        let from = txn["transaction"]["from"].as_str().unwrap();
        let data = txn["transaction"]["data"].as_str().unwrap();
        let value = txn["transaction"]["value"].as_str().unwrap();
        let gas_used = txn["transaction"]["gas"].as_str().unwrap();
        // let gas_price = txn["transaction"]["gasPrice"].as_str().unwrap();
        let event_obj = json!({
            "event": event,
            "from": from,
            "contract_address": address,
            "data": data,
            "value": value,
            "gas_used": gas_used,
            // "gas_price": gas_price,
        });
        events.push(event_obj);
    }

    // Serialize the data back into a JSON string.
    let j = serde_json::to_string(&events)?;
    println!("{}", j);
    Ok(())
}

extern crate serde_json;

use serde_json::Value as JsonValue;

fn main() {
    let foundry_broadcast = r#"
        {
            "transactionType": "CREATE",
            "contractName": "Counter",
            "contractAddress": "0xD1aE64401d65E9B0d1bF7E08Fbf75bb2F26eF70a",
        }
    "#;
}

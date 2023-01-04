extern crate serde_json;

use serde_json::Value as JsonValue;

fn main() {
    let foundry_broadcast = r#"
        {
            "transactionType": "CREATE",
            "contractName": "Counter",
            "contractAddress": "0xD1aE64401d65E9B0d1bF7E08Fbf75bb2F26eF70a"
        }
    "#;

    let deserialized_result = serde_json::from_str(foundry_broadcast);

    if deserialized_result.is_ok() {
        let parsed_result: JsonValue = deserialized_result.unwrap();
        println!("Type of transaction: {}", parsed_result["transactionType"]);
        println!("Name of the contract: {}", parsed_result["contractName"]);
        println!("Deployed to: {}", parsed_result["contractAddress"]);
    } else {
        println!("Sorry, could not parse JSON.");
    }
}

mod serialization_tests {
    use super::*;

    #[test]
    fn it_should_parse_transaction_type_correctly() {
        assert_eq!(
            parse_foundry_broadcast(
                r#"
            {
                "transactionType": "CREATE"
            }
        "#
            ),
            "CREATE"
        );
    }
}

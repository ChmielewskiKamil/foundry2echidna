extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    transaction_type: String,
    contract_name: String,
    contract_address: String,
}

fn main() {
    let foundry_broadcast = r#"
        {
            "transactionType": "CREATE",
            "contractName": "Counter",
            "contractAddress": "0xD1aE64401d65E9B0d1bF7E08Fbf75bb2F26eF70a"
        }
    "#;
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

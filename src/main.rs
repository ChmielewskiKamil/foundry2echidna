use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    let transaction = parse_foundry_broadcast(foundry_broadcast);
    println!("{:?}", transaction);
}

fn parse_foundry_broadcast(broadcast: &str) -> Transaction {
    let parsed: Transaction = serde_json::from_str(broadcast).unwrap();
    parsed
}

#[cfg(test)]
mod serialization_tests {

    use super::parse_foundry_broadcast;

    #[test]
    fn it_should_parse_transaction_type_correctly() {
        assert_eq!(
            parse_foundry_broadcast(
                r#"
            {
            "transactionType": "CREATE",
            "contractName": "Counter",
            "contractAddress": "0xD1aE64401d65E9B0d1bF7E08Fbf75bb2F26eF70a"
        }
        "#
            )
            .transaction_type,
            "CREATE"
        );
    }

    #[test]
    fn it_should_parse_contract_name_correctly() {
        assert_eq!(
            parse_foundry_broadcast(
                r#"
            {
            "transactionType": "CREATE",
            "contractName": "Counter",
            "contractAddress": "0xD1aE64401d65E9B0d1bF7E08Fbf75bb2F26eF70a"
        }
        "#
            )
            .contract_name,
            "Counter"
        );
    }
}

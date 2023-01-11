use serde::Deserialize;
use serde_json::from_str;
use std::{fs::File, io::Read};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    transaction_type: String,
    contract_address: String,
    transaction: TransactionDetails,
}

#[derive(Deserialize, Debug, PartialEq)]
struct TransactionDetails {
    from: String,
    gas: String,
    value: String,
    data: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Receipt {
    from: String,
    to: Option<String>,
    contract_address: String,
    gas_used: String,
    effective_gas_price: String,
}

pub fn read_broadcast_file(path_to_file: &str) -> Result<String, String> {
    let mut file =
        File::open(path_to_file).map_err(|err| format!("Error while opening the file: {}", err))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("Error while reading to string: {}", err))?;
    Ok(content)
}

fn deserialize_single_transaction(
    transaction_to_deserialize: String,
) -> Result<Transaction, String> {
    let transaction: Transaction = serde_json::from_str(&transaction_to_deserialize)
        .map_err(|err| format!("Failed to deserialize transaction: {}", err))?;
    Ok(transaction)
}

fn deserialize_single_receipt(receipt_to_deserialize: String) -> Receipt {
    let receipt: Receipt = serde_json::from_str(&receipt_to_deserialize).unwrap();
    receipt
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn it_should_read_file_content_to_string() {
        let expected_content = r#"{
    "transactions": [
        {
            "transactionType": "CREATE",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "transaction": {
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "gas": "0x8f864",
                "value": "0x0",
                "data": "0x6080604"
            }
        }
    ]
}"#;

        let actual_content = read_broadcast_file("test_json_files/simple_broadcast_test.json");
        assert_eq!(expected_content, actual_content.unwrap());
    }

    #[test]
    fn it_should_properly_deserialize_single_transaction() {
        let transaction_to_deserialize = r#"
        {
            "transactionType": "CREATE",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "transaction": {
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "gas": "0x8f864",
                "value": "0x0",
                "data": "0x6080604"
            }
        }
        "#
        .to_string();

        let deserialized_transaction =
            deserialize_single_transaction(transaction_to_deserialize).unwrap();

        let expected_deserialization_result = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                gas: "0x8f864".to_string(),
                value: "0x0".to_string(),
                data: "0x6080604".to_string(),
            },
        };
        assert_eq!(deserialized_transaction, expected_deserialization_result);
    }

    #[test]
    fn it_should_properly_deserialize_single_receipt() {
        let receipt_to_deserialize = r#"
        {
            "transactionHash": "0xd532ff21e93eac89c2bbd5f4813ac0d9274e479b6eb09b2b2f45b82489faba1b",
            "transactionIndex": "0x0",
            "blockHash": "0xec94f9df892826b801574831de293f983ed8f3f81036a99faa616a8da694b2a9",
            "blockNumber": "0x1",
            "from": "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "to": null,
            "cumulativeGasUsed": "0x1234",
            "gasUsed": "0x6e675",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "effectiveGasPrice": "0xe0fed783"
        }
        "#
        .to_string();

        let deserialized_receipt: Receipt = deserialize_single_receipt(receipt_to_deserialize);

        let expected_deserialization_result = Receipt {
            from: "0x90F79bf6EB2c4f870365E785982E1f101E93b906".to_string(),
            to: None,
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            gas_used: "0x6e675".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        assert_eq!(deserialized_receipt, expected_deserialization_result);
    }
}

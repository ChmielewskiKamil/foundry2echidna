use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::from_str;
use std::{fs::File, io::Read};

/*//////////////////////////////////////////////////////////////
                        DATA MODEL STRUCTS
////////////////////////////////////////////////////////////// */

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    #[serde(rename(serialize = "event"))]
    transaction_type: String,
    contract_address: String,
    transaction: TransactionDetails,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct TransactionDetails {
    from: String,
    to: Option<String>,
    value: String,
    data: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Receipt {
    gas_used: String,
    #[serde(rename(serialize = "gas_price"))]
    effective_gas_price: String,
}

#[derive(Serialize, Debug, PartialEq)]
struct ContractCreationEvent {
    event: String,
    from: String,
    contract_address: String,
    gas_used: String,
    gas_price: String,
    data: String,
    value: String,
}

#[derive(Serialize, Debug, PartialEq)]
struct FunctionCallEvent {
    event: String,
    from: String,
    to: String,
    gas_used: String,
    gas_price: String,
    data: String,
    value: String,
}

/*//////////////////////////////////////////////////////////////
                    PARSING HELPER STRUCTS
////////////////////////////////////////////////////////////// */
#[derive(Deserialize, Debug, PartialEq)]
struct TransactionsList {
    transactions: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReceiptsList {
    receipts: Vec<String>,
}

/*//////////////////////////////////////////////////////////////
                         FILE HANDLING
////////////////////////////////////////////////////////////// */
pub fn read_broadcast_file(path_to_file: &str) -> Result<String, String> {
    let mut file =
        File::open(path_to_file).map_err(|err| format!("Error while opening the file: {err}"))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("Error while reading to string: {err}"))?;
    Ok(content)
}

/*//////////////////////////////////////////////////////////////
                    DESERIALIZATION FUNCTIONS
////////////////////////////////////////////////////////////// */
fn deserialize_single_transaction(transaction_to_deserialize: &str) -> Result<Transaction, String> {
    let transaction: Transaction = serde_json::from_str(transaction_to_deserialize)
        .map_err(|err| format!("Failed to deserialize transaction: {err}"))?;
    Ok(transaction)
}

#[allow(dead_code)]
fn deserialize_single_receipt(receipt_to_deserialize: &str) -> Result<Receipt, String> {
    let receipt: Receipt = serde_json::from_str(receipt_to_deserialize)
        .map_err(|err| format!("Failed to deserialize receipt: {err}"))?;
    Ok(receipt)
}

#[allow(dead_code)]
fn deserialize_multiple_transactions(
    transactions_to_deserialize: TransactionsList,
) -> Result<Vec<Transaction>, String> {
    let mut transactions = vec![];
    for transaction in transactions_to_deserialize.transactions.into_iter() {
        transactions.push(
            deserialize_single_transaction(&transaction)
                .map_err(|err| format!("Failed to deserialze an array of transactions: {err}"))?,
        );
    }
    Ok(transactions)
}

#[allow(dead_code)]
fn deserialize_multiple_receipts(
    receipts_to_deserialize: ReceiptsList,
) -> Result<Vec<Receipt>, String> {
    let mut receipts = vec![];
    for receipt in receipts_to_deserialize.receipts.into_iter() {
        receipts.push(
            deserialize_single_receipt(&receipt)
                .map_err(|err| format!("Failed to deserialize an array of receipts: {err}"))?,
        );
    }
    Ok(receipts)
}
/*//////////////////////////////////////////////////////////////
                   SERIALIZATION FUNCTIONS
////////////////////////////////////////////////////////////// */
#[allow(dead_code)]
fn serialize_transaction(transaction: Transaction, receipt: Receipt) -> Result<String, String> {
    let mut serialized_transaction = String::new();
    match transaction.transaction_type.as_ref() {
        "CREATE" => {
            let creation_event: ContractCreationEvent = ContractCreationEvent {
                event: "ContractCreated".to_string(),
                from: transaction.transaction.from,
                contract_address: transaction.contract_address,
                gas_used: receipt.gas_used,
                gas_price: receipt.effective_gas_price,
                data: transaction.transaction.data,
                value: transaction.transaction.value,
            };
            serialized_transaction.push_str(
                &serde_json::to_string(&creation_event)
                    .map_err(|err| format!("Failed to serialize creation event: {err}"))?,
            );
        }
        "CALL" => {
            let function_call_event: FunctionCallEvent = FunctionCallEvent {
                event: "FunctionCall".to_string(),
                from: transaction.transaction.from,
                to: transaction
                    .transaction
                    .to
                    .ok_or("The 'to' field is empty!")?,
                gas_used: receipt.gas_used,
                gas_price: receipt.effective_gas_price,
                data: transaction.transaction.data,
                value: transaction.transaction.value,
            };
            serialized_transaction.push_str(
                &serde_json::to_string(&function_call_event)
                    .map_err(|err| format!("Failed to serialize function call event: {err}"))?,
            );
        }
        _ => {}
    }

    Ok(serialized_transaction)
}

#[allow(dead_code)]
fn serialize_broadcast(
    tx_array: Vec<Transaction>,
    receipts_array: Vec<Receipt>,
) -> Result<Vec<String>, String> {
    let mut serialized_tx_and_receipts = vec![];
    for (tx, receipt) in tx_array.into_iter().zip(receipts_array.into_iter()) {
        serialized_tx_and_receipts.push(serialize_transaction(tx, receipt)?);
    }
    Ok(serialized_tx_and_receipts)
}

/*//////////////////////////////////////////////////////////////
                        UNIT TESTS
////////////////////////////////////////////////////////////// */
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
    /*//////////////////////////////////////////////////////////////
                        DESERIALIZATION TESTS
    ////////////////////////////////////////////////////////////// */

    #[test]
    fn it_should_deserialize_single_transaction() {
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
            deserialize_single_transaction(&transaction_to_deserialize).unwrap();

        let expected_deserialization_result = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: None,
                value: "0x0".to_string(),
                data: "0x6080604".to_string(),
            },
        };
        assert_eq!(deserialized_transaction, expected_deserialization_result);
    }

    #[test]
    fn it_should_deserialize_single_receipt() {
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

        let deserialized_receipt: Receipt =
            deserialize_single_receipt(&receipt_to_deserialize).unwrap();

        let expected_deserialization_result = Receipt {
            gas_used: "0x6e675".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        assert_eq!(deserialized_receipt, expected_deserialization_result);
    }
    #[test]
    fn it_should_deserialize_a_series_of_transactions() {
        let tx1 = r#"{
            "hash": "0xd532ff21e93eac89c2bbd5f4813ac0d9274e479b6eb09b2b2f45b82489faba1b",
            "transactionType": "CREATE",
            "contractName": "Ethernaut",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "function": null,
            "arguments": null,
            "transaction": {
                "type": "0x02",
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "gas": "0x8f864",
                "value": "0x0",
                "data": "0x6080604",
                "nonce": "0x0",
                "accessList": []
            },
            "additionalContracts": []
        }"#
        .to_string();
        let tx2 = r#"{
            "hash": "0x5370406a7d060079764126708230356640e3494965321ab622842123ebb71052",
            "transactionType": "CREATE",
            "contractName": "PrivacyFactory",
            "contractAddress": "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad",
            "function": null,
            "arguments": null,
            "transaction": {
                "type": "0x02",
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "gas": "0x936a5",
                "value": "0x0",
                "data": "0x608060405",
                "nonce": "0x1",
                "accessList": []
            },
            "additionalContracts": []
        }"#
        .to_string();
        let transactions_to_deserialize = TransactionsList {
            transactions: vec![tx1, tx2],
        };

        let deserialized_tx1 = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: None,
                value: "0x0".to_string(),
                data: "0x6080604".to_string(),
            },
        };
        let deserialized_tx2 = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: None,
                value: "0x0".to_string(),
                data: "0x608060405".to_string(),
            },
        };

        let expected_result = vec![deserialized_tx1, deserialized_tx2];

        let deserialization_result =
            deserialize_multiple_transactions(transactions_to_deserialize).unwrap();

        assert_eq!(expected_result, deserialization_result);
    }

    #[test]
    fn it_should_deserialize_a_series_of_receipts() {
        let receipt1 = r#"{
            "transactionHash": "0xd532ff21e93eac89c2bbd5f4813ac0d9274e479b6eb09b2b2f45b82489faba1b",
            "transactionIndex": "0x0",
            "blockHash": "0xec94f9df892826b801574831de293f983ed8f3f81036a99faa616a8da694b2a9",
            "blockNumber": "0x1",
            "from": "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "to": null,
            "cumulativeGasUsed": "0x6e675",
            "gasUsed": "0x6e675",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "logs": [
                {
                    "address": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
                    "topics": [
                        "0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0",
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                        "0x00000000000000000000000090f79bf6eb2c4f870365e785982e1f101e93b906"
                    ],
                    "data": "0x",
                    "blockHash": "0xec94f9df892826b801574831de293f983ed8f3f81036a99faa616a8da694b2a9",
                    "blockNumber": "0x1",
                    "transactionHash": "0xd532ff21e93eac89c2bbd5f4813ac0d9274e479b6eb09b2b2f45b82489faba1b",
                    "transactionIndex": "0x0",
                    "logIndex": "0x0",
                    "transactionLogIndex": "0x0",
                    "removed": false
                }
            ],
            "status": "0x1",
            "logsBloom": "0x000000",
            "effectiveGasPrice": "0xe0fed783"
        }"#.to_string();
        let receipt2 = r#"{
            "transactionHash": "0x5370406a7d060079764126708230356640e3494965321ab622842123ebb71052",
            "transactionIndex": "0x0",
            "blockHash": "0x89faf9173c057b0db7693d1fbe9e06618ec69ad634ac5e0e42022f07c1ffc492",
            "blockNumber": "0x2",
            "from": "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "to": null,
            "cumulativeGasUsed": "0x71658",
            "gasUsed": "0x71658",
            "contractAddress": "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad",
            "logs": [
                {
                    "address": "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad",
                    "topics": [
                        "0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0",
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                        "0x00000000000000000000000090f79bf6eb2c4f870365e785982e1f101e93b906"
                    ],
                    "data": "0x",
                    "blockHash": "0x89faf9173c057b0db7693d1fbe9e06618ec69ad634ac5e0e42022f07c1ffc492",
                    "blockNumber": "0x2",
                    "transactionHash": "0x5370406a7d060079764126708230356640e3494965321ab622842123ebb71052",
                    "transactionIndex": "0x0",
                    "logIndex": "0x0",
                    "transactionLogIndex": "0x0",
                    "removed": false
                }
            ],
            "status": "0x1",
            "logsBloom": "0x0000",
            "effectiveGasPrice": "0xe0fed783"
        }"#.to_string();

        let receipts_to_deserialize = ReceiptsList {
            receipts: vec![receipt1, receipt2],
        };

        let deserialized_receipt1 = Receipt {
            gas_used: "0x6e675".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };
        let deserialized_receipt2 = Receipt {
            gas_used: "0x71658".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        let expected_result = vec![deserialized_receipt1, deserialized_receipt2];
        let deserialization_result =
            deserialize_multiple_receipts(receipts_to_deserialize).unwrap();

        assert_eq!(expected_result, deserialization_result);
    }
    /*//////////////////////////////////////////////////////////////
                            SERIALIZATION TESTS
    ////////////////////////////////////////////////////////////// */

    #[test]
    fn it_should_serialize_single_contract_creation_event() {
        let transaction_to_serialize = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: None,
                value: "0x0".to_string(),
                data: "0x6080604".to_string(),
            },
        };

        let receipt_to_serialize = Receipt {
            gas_used: "0x6e675".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        let expected_serialization_result = r#"{"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x057ef64E23666F000b34aE31332854aCBd1c8544","gas_used":"0x6e675","gas_price":"0xe0fed783","data":"0x6080604","value":"0x0"}"#;

        let serialization_result =
            serialize_transaction(transaction_to_serialize, receipt_to_serialize).unwrap();

        assert_eq!(expected_serialization_result, serialization_result);
    }

    #[test]
    fn it_should_serialize_single_function_call_event() {
        let transaction_to_serialize = Transaction {
            transaction_type: "CALL".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: Some("0x057ef64e23666f000b34ae31332854acbd1c8544".to_string()),
                value: "0x0".to_string(),
                data: "0x202023".to_string(),
            },
        };

        let receipt_to_serialize = Receipt {
            gas_used: "0xb3bd".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        let expected_serialization_result = r#"{"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}"#;

        let serialization_result =
            serialize_transaction(transaction_to_serialize, receipt_to_serialize).unwrap();

        assert_eq!(expected_serialization_result, serialization_result);
    }

    #[test]
    fn it_should_serialize_both_transaction_and_receipt_from_tx_and_receipt_arrays() {
        let tx1 = Transaction {
            transaction_type: "CALL".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: Some("0x057ef64e23666f000b34ae31332854acbd1c8544".to_string()),
                value: "0x0".to_string(),
                data: "0x202023".to_string(),
            },
        };
        let receipt1 = Receipt {
            gas_used: "0xb3bd".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        let tx_array_to_serialize = vec![tx1];

        let receipts_array_to_serialize = vec![receipt1];

        let expected_serialization_result = vec![
            r#"{"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}"#,
        ];

        let serialization_result =
            serialize_broadcast(tx_array_to_serialize, receipts_array_to_serialize);

        assert_eq!(expected_serialization_result, serialization_result.unwrap());
    }
}

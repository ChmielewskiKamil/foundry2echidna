use crate::data_model::{Broadcast, Receipt, Transaction};
use serde::Serialize;
use serde_json::{json, Value};

/* //////////////////////////////////////////////////////////////
                ETHENO SERIALIZATION STRUCTS
////////////////////////////////////////////////////////////// */
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

fn serialize_transaction(transaction: Transaction, receipt: Receipt) -> Result<Value, String> {
    match transaction.transaction_type.as_ref() {
        "CREATE" => {
            let creation_event = ContractCreationEvent {
                event: "ContractCreated".to_string(),
                from: transaction.transaction.from,
                contract_address: transaction.contract_address,
                gas_used: receipt.gas_used,
                gas_price: receipt.effective_gas_price,
                data: transaction.transaction.data,
                value: transaction.transaction.value,
            };
            Ok(serde_json::to_value(creation_event)
                .map_err(|err| format!("Failed to serialize creation event: {err}"))?)
        }
        "CALL" => {
            let function_call_event = FunctionCallEvent {
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
            Ok(serde_json::to_value(function_call_event)
                .map_err(|err| format!("Failed to serialize function call event: {err}"))?)
        }
        _ => Ok(serde_json::Value::Null),
    }
}

pub fn serialize_broadcast(broadcast: Broadcast) -> Result<Vec<serde_json::Value>, String> {
    let mut serialized_tx_and_receipts = vec![];
    for (tx, receipt) in broadcast
        .transactions
        .into_iter()
        .zip(broadcast.receipts.into_iter())
    {
        serialized_tx_and_receipts.push(serialize_transaction(tx, receipt)?);
    }
    Ok(serialized_tx_and_receipts)
}

pub fn add_account_created_events(
    serialized_broadcast: Vec<serde_json::Value>,
) -> Result<Vec<serde_json::Value>, String> {
    let account_created_objects = vec![
        json!({"event":"AccountCreated", "address": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"}),
        json!({"event":"AccountCreated", "address": "0x70997970c51812dc3a010c7d01b50e0d17dc79c8"}),
        json!({"event":"AccountCreated", "address": "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc"}),
        json!({"event":"AccountCreated", "address": "0x90f79bf6eb2c4f870365e785982e1f101e93b906"}),
        json!({"event":"AccountCreated", "address": "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65"}),
        json!({"event":"AccountCreated", "address": "0x9965507d1a55bcc2695c58ba16fb37d819b0a4dc"}),
        json!({"event":"AccountCreated", "address": "0x976ea74026e726554db657fa54763abd0c3a0aa9"}),
        json!({"event":"AccountCreated", "address": "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955"}),
        json!({"event":"AccountCreated", "address": "0x23618e81e3f5cdf7f54c3d65f7fbc0abf5b21e8f"}),
        json!({"event":"AccountCreated", "address": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720"}),
    ];
    let mut etheno_like_broadcast = account_created_objects;
    etheno_like_broadcast.extend(serialized_broadcast);
    Ok(etheno_like_broadcast)
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    use crate::data_model::TransactionDetails;
    use crate::deserialization::deserialize_broadcast;
    use serde_json::json;

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

        let expected_serialization_result: serde_json::Value = serde_json::from_str(
            r#"{"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x057ef64E23666F000b34aE31332854aCBd1c8544","gas_used":"0x6e675","gas_price":"0xe0fed783","data":"0x6080604","value":"0x0"}"#,
        ).unwrap();

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

        let expected_serialization_result: serde_json::Value = serde_json::from_str(r#"{"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}"#).unwrap();

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
        let rcp1 = Receipt {
            gas_used: "0xb3bd".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };

        let transactions = vec![tx1];
        let receipts = vec![rcp1];

        let broadcast_to_serialize = Broadcast {
            transactions,
            receipts,
        };

        let expected_serialization_result = vec![json!(
            {"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}
        )];

        let serialization_result = serialize_broadcast(broadcast_to_serialize).unwrap();
        assert_eq!(expected_serialization_result, serialization_result);
    }
    #[test]
    fn it_should_serialize_broadcast() {
        let broadcast_to_deserialize = r#"{
    "transactions": [
        {
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
        },
        {
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
        },
        {
            "hash": "0xea12bb08d4a6a3c5179900391cb592912a0fa3d5c2752bdd4a0f3d9d598a1393",
            "transactionType": "CALL",
            "contractName": "Ethernaut",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "function": "registerLevel(address)",
            "arguments": [
                "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad"
            ],
            "transaction": {
                "type": "0x02",
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "to": "0x057ef64e23666f000b34ae31332854acbd1c8544",
                "gas": "0xf842",
                "value": "0x0",
                "data": "0x202023",
                "nonce": "0x2",
                "accessList": []
            },
            "additionalContracts": []
        },
        {
            "hash": "0xaaa30e5c281ffeb0fbb624e826f2e8e452da0dda0986b97ec13053ffb1cb4630",
            "transactionType": "CALL",
            "contractName": "Ethernaut",
            "contractAddress": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "function": "createLevelInstance(address):(address)",
            "arguments": [
                "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad"
            ],
            "transaction": {
                "type": "0x02",
                "from": "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
                "to": "0x057ef64e23666f000b34ae31332854acbd1c8544",
                "gas": "0x6831e",
                "value": "0x0",
                "data": "0xdfc86b17000000000000000000000000261d8c5e9742e6f7f1076fa1f560894524e19cad",
                "nonce": "0x3",
                "accessList": []
            },
            "additionalContracts": [
                {
                    "transactionType": "CREATE",
                    "address": "0xF3dfB0A70010735B0A14B4A69aFC242b19600049",
                    "initCode": "6080"
                }
            ]
        }
    ],
    "receipts": [
        {
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
        },
        {
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
        },
        {
            "transactionHash": "0xea12bb08d4a6a3c5179900391cb592912a0fa3d5c2752bdd4a0f3d9d598a1393",
            "transactionIndex": "0x1",
            "blockHash": "0x89faf9173c057b0db7693d1fbe9e06618ec69ad634ac5e0e42022f07c1ffc492",
            "blockNumber": "0x2",
            "from": "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "to": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "cumulativeGasUsed": "0x7ca15",
            "gasUsed": "0xb3bd",
            "contractAddress": null,
            "logs": [],
            "status": "0x1",
            "logsBloom": "0x0000",
            "effectiveGasPrice": "0xe0fed783"
        },
        {
            "transactionHash": "0xaaa30e5c281ffeb0fbb624e826f2e8e452da0dda0986b97ec13053ffb1cb4630",
            "transactionIndex": "0x2",
            "blockHash": "0x89faf9173c057b0db7693d1fbe9e06618ec69ad634ac5e0e42022f07c1ffc492",
            "blockNumber": "0x2",
            "from": "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "to": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
            "cumulativeGasUsed": "0xc3dfe",
            "gasUsed": "0x473e9",
            "contractAddress": null,
            "logs": [
                {
                    "address": "0x057ef64E23666F000b34aE31332854aCBd1c8544",
                    "topics": [
                        "0x7bf7f1ed7f75e83b76de0ff139966989aff81cb85aac26469c18978d86aac1c2",
                        "0x00000000000000000000000090f79bf6eb2c4f870365e785982e1f101e93b906"
                    ],
                    "data": "0x000000000000000000000000f3dfb0a70010735b0a14b4a69afc242b19600049",
                    "blockHash": "0x89faf9173c057b0db7693d1fbe9e06618ec69ad634ac5e0e42022f07c1ffc492",
                    "blockNumber": "0x2",
                    "transactionHash": "0xaaa30e5c281ffeb0fbb624e826f2e8e452da0dda0986b97ec13053ffb1cb4630",
                    "transactionIndex": "0x2",
                    "logIndex": "0x2",
                    "transactionLogIndex": "0x0",
                    "removed": false
                }
            ],
            "status": "0x1",
            "logsBloom": "0x080000",
            "effectiveGasPrice": "0xe0fed783"
        }
    ],
    "libraries": [],
    "pending": [],
    "path": "/Users/kamilchmielewski/Projects/ethernaut-foundry/broadcast/Privacy.s.sol/31337/run-latest.json",
    "returns": {},
    "timestamp": 1668342002,
    "commit": "cba0070"
}"#;
        let deserialized_broadcast = deserialize_broadcast(broadcast_to_deserialize).unwrap();

        let expected_serialization_result = vec![
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x057ef64E23666F000b34aE31332854aCBd1c8544","gas_used":"0x6e675","gas_price":"0xe0fed783","data":"0x6080604","value":"0x0"}),
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x261D8c5e9742e6f7f1076Fa1F560894524e19cad","gas_used":"0x71658","gas_price":"0xe0fed783","data":"0x608060405","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0x473e9","gas_price":"0xe0fed783","data":"0xdfc86b17000000000000000000000000261d8c5e9742e6f7f1076fa1f560894524e19cad","value":"0x0"}),
        ];

        let events = serialize_broadcast(deserialized_broadcast).unwrap();

        assert_eq!(expected_serialization_result, events);
    }
    #[test]
    fn it_should_add_account_created_events_at_the_top() {
        let serialized_broadcast_events: Vec<serde_json::Value> = vec![
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x057ef64E23666F000b34aE31332854aCBd1c8544","gas_used":"0x6e675","gas_price":"0xe0fed783","data":"0x6080604","value":"0x0"}),
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x261D8c5e9742e6f7f1076Fa1F560894524e19cad","gas_used":"0x71658","gas_price":"0xe0fed783","data":"0x608060405","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0x473e9","gas_price":"0xe0fed783","data":"0xdfc86b17000000000000000000000000261d8c5e9742e6f7f1076fa1f560894524e19cad","value":"0x0"}),
        ];
        let expected_result: Vec<serde_json::Value> = vec![
            json!({"event":"AccountCreated", "address": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"}),
            json!({"event":"AccountCreated", "address": "0x70997970c51812dc3a010c7d01b50e0d17dc79c8"}),
            json!({"event":"AccountCreated", "address": "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc"}),
            json!({"event":"AccountCreated", "address": "0x90f79bf6eb2c4f870365e785982e1f101e93b906"}),
            json!({"event":"AccountCreated", "address": "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65"}),
            json!({"event":"AccountCreated", "address": "0x9965507d1a55bcc2695c58ba16fb37d819b0a4dc"}),
            json!({"event":"AccountCreated", "address": "0x976ea74026e726554db657fa54763abd0c3a0aa9"}),
            json!({"event":"AccountCreated", "address": "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955"}),
            json!({"event":"AccountCreated", "address": "0x23618e81e3f5cdf7f54c3d65f7fbc0abf5b21e8f"}),
            json!({"event":"AccountCreated", "address": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720"}),
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x057ef64E23666F000b34aE31332854aCBd1c8544","gas_used":"0x6e675","gas_price":"0xe0fed783","data":"0x6080604","value":"0x0"}),
            json!({"event":"ContractCreated","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","contract_address":"0x261D8c5e9742e6f7f1076Fa1F560894524e19cad","gas_used":"0x71658","gas_price":"0xe0fed783","data":"0x608060405","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0xb3bd","gas_price":"0xe0fed783","data":"0x202023","value":"0x0"}),
            json!({"event":"FunctionCall","from":"0x90f79bf6eb2c4f870365e785982e1f101e93b906","to":"0x057ef64e23666f000b34ae31332854acbd1c8544","gas_used":"0x473e9","gas_price":"0xe0fed783","data":"0xdfc86b17000000000000000000000000261d8c5e9742e6f7f1076fa1f560894524e19cad","value":"0x0"}),
        ];
        let addition_result = add_account_created_events(serialized_broadcast_events);
        assert_eq!(expected_result, addition_result);
    }
}

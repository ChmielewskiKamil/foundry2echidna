use crate::data_model::Broadcast;
/*//////////////////////////////////////////////////////////////
                    DESERIALIZATION FUNCTIONS
////////////////////////////////////////////////////////////// */
pub fn deserialize_broadcast(broadcast_to_deserialize: &str) -> Result<Broadcast, String> {
    let broadcast: Broadcast = serde_json::from_str(broadcast_to_deserialize)
        .map_err(|err| format!("Failed to deserialize broadcast: {}", err))?;
    Ok(broadcast)
}

#[cfg(test)]
mod deserialization_tests {
    use super::*;
    use crate::data_model::{Receipt, Transaction, TransactionDetails};

    #[test]
    fn it_should_deserialize_broadcast() {
        let broadcast_to_deserialize = r#"{
    "transactions": [
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
        }
    ],
    "receipts": [
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
        }
    ],
    "libraries": [],
    "pending": [],
    "path": "/Users/kamilchmielewski/Projects/ethernaut-foundry/broadcast/Privacy.s.sol/31337/run-latest.json",
    "returns": {},
    "timestamp": 1668342002,
    "commit": "cba0070"
}"#;
        let expected_tx1 = Transaction {
            transaction_type: "CREATE".to_string(),
            contract_address: "0x261D8c5e9742e6f7f1076Fa1F560894524e19cad".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: None,
                value: "0x0".to_string(),
                data: "0x608060405".to_string(),
            },
        };

        let expected_tx2 = Transaction {
            transaction_type: "CALL".to_string(),
            contract_address: "0x057ef64E23666F000b34aE31332854aCBd1c8544".to_string(),
            transaction: TransactionDetails {
                from: "0x90f79bf6eb2c4f870365e785982e1f101e93b906".to_string(),
                to: Some("0x057ef64e23666f000b34ae31332854acbd1c8544".to_string()),
                value: "0x0".to_string(),
                data: "0x202023".to_string(),
            },
        };
        let expected_receipt1 = Receipt {
            gas_used: "0x71658".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };
        let expected_receipt2 = Receipt {
            gas_used: "0xb3bd".to_string(),
            effective_gas_price: "0xe0fed783".to_string(),
        };
        let transactions = vec![expected_tx1, expected_tx2];
        let receipts = vec![expected_receipt1, expected_receipt2];
        let expected_broadcast = Broadcast {
            transactions,
            receipts,
        };
        let deserialization_result = deserialize_broadcast(broadcast_to_deserialize).unwrap();
        assert_eq!(expected_broadcast, deserialization_result);
    }
}

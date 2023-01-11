use std::{fs::File, io::Read};

#[derive(Debug, PartialEq)]
struct Transaction {
    transaction_type: String,
    contract_address: String,
    transaction: TransactionDetails,
}

#[derive(Debug, PartialEq)]
struct TransactionDetails {
    from: String,
    gas: String,
    value: String,
    data: String,
}

fn deserialize_single_transaction(transaction_to_deserialize: String) -> Transaction {
    todo!();
}

pub fn read_broadcast_file(path_to_file: &str) -> Result<String, String> {
    let mut file =
        File::open(path_to_file).map_err(|err| format!("Error while opening the file: {}", err))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("Error while reading to string: {}", err))?;
    Ok(content)
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn it_should_read_file_content_to_string() {
        let expected_content = r#"
        {
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
        }
"#;

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

        let deserialized_transaction = deserialize_single_transaction(transaction_to_deserialize);

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
}

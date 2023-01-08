#[cfg(test)]
mod parser_tests {
    use super::read_broadcast_file;
    #[test]
    fn it_should_read_file_content_to_string() {
        assert_eq!(
            read_broadcast_file("../test_json_files/simple_broadcast_test.json"),
            r#"{
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
}"#
        )
    }
}

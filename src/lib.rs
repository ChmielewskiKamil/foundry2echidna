use std::{fs::File, io::Read};

pub fn read_broadcast_file(path_to_file: &str) -> String {
    let mut file = File::open(path_to_file).unwrap();

    let mut content = String::new();
    println!("{:?}", file);
    file.read_to_string(&mut content).unwrap();
    content
}

#[cfg(test)]
mod parser_tests {
    use super::read_broadcast_file;

    #[test]
    fn it_should_read_file_content_to_string() {
        assert_eq!(
            read_broadcast_file("test_json_files/simple_broadcast_test.json"),
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

use serde_json::{json, to_string_pretty, Value};
use std::{fs::File, io::Read, io::Write};
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

pub fn write_transformed_broadcast_to_file(
    events: &[Value],
    path_to_file: &str,
) -> Result<(), String> {
    let json = json!(events);
    let pretty_json =
        to_string_pretty(&json).map_err(|err| format!("Failed to make json look pretty: {err}"))?;
    let mut file = File::create(path_to_file)
        .map_err(|err| format!("Error while creating the file: {err}"))?;

    file.write_all(pretty_json.as_bytes())
        .map_err(|err| format!("Error while writing to file: {err}"))?;
    Ok(())
}

#[cfg(test)]
mod file_handling_tests {
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
}

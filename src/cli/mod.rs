use crate::deserialization::deserialize_broadcast;
use crate::file_handling::{read_broadcast_file, write_transformed_broadcast_to_file};
use crate::serialization::{add_account_created_events, serialize_broadcast};
use clap::Parser;
use glob::glob;
use std::{fs::create_dir_all, path::Path};

/// Transforms a Foundry broadcast file into a format that is compatible with Echidna.
///
/// # Arguments
/// * `input_path` - Path to the Foundry broadcast file to be transformed.
/// * `output_path` - Path to a file where you want to save the transformed broadcast.
///
/// If `input_path` is not provided, the default path is `broadcast/*.s.sol/31337/run-latest.json`.
/// If `output_path` is not provided, the default path is `src/crytic/init.json`.
///
/// # Examples
///
/// ```
/// use foundry2echidna::cli::transform_broadcast;
/// transform_broadcast("tests/data/input.json", "tests/data/output.json").unwrap();
/// ```
pub fn transform_broadcast(input_path: &str, output_path: &str) -> Result<(), String> {
    let broadcast_to_deserialize = read_broadcast_file(input_path)?;
    let broadcast = deserialize_broadcast(&broadcast_to_deserialize)?;
    let broadcast = serialize_broadcast(broadcast)?;
    let etheno_like_broadcast = add_account_created_events(broadcast)?;
    write_transformed_broadcast_to_file(&etheno_like_broadcast, output_path)?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
pub struct Args {
    #[clap(
        short,
        long,
        help = r#"Path to the Foundry broadcast file to be transformed.
If not provided, the default path is `broadcast/*.s.sol/31337/run-latest.json`. 
Please note that if you have a couple of directories in the `broadcast` dir, 
the first one found will be used by default."#
    )]
    pub input_path: Option<String>,

    #[clap(
        short,
        long,
        help = "Path to a file where you want to save the transformed broadcast. If not provided, the default path is `src/crytic/init.json`."
    )]
    pub output_path: Option<String>,
}

impl Args {
    pub fn new() -> Result<Self, String> {
        let mut args = Self::parse();
        if args.input_path.is_none() {
            let glob_pattern = "broadcast/*.s.sol/31337/run-latest.json";
            let mut paths = match glob(glob_pattern) {
                Ok(paths) => paths,
                Err(e) => return Err(e.to_string()),
            };
            let path = match paths.next() {
                Some(path) => path,
                None => return Err("No matching input paths found".to_string()),
            };
            args.input_path = Some(match path.unwrap().to_str() {
                Some(s) => s.to_string(),
                None => return Err("Failed to convert input path to string".to_string()),
            });
        }
        if args.output_path.is_none() {
            let output_dir = Path::new("src/crytic");
            match create_dir_all(output_dir) {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            };
            args.output_path = Some(match output_dir.join("init.json").to_str() {
                Some(s) => s.to_string(),
                None => return Err("Failed to convert output path to string".to_string()),
            });
        } else {
            let output_path = match Path::new(args.output_path.as_ref().unwrap()).parent() {
                Some(p) => p,
                None => {
                    return Err("Failed to extract parent directory from output path".to_string())
                }
            };
            match create_dir_all(output_path) {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            };
        }
        Ok(args)
    }
}

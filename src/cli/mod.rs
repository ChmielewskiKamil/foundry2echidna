use crate::deserialization::deserialize_broadcast;
use crate::file_handling::{read_broadcast_file, write_transformed_broadcast_to_file};
use crate::serialization::serialize_broadcast;
use glob::glob;
use std::{fs::create_dir_all, path::Path};

use clap::Parser;

pub fn run(input_path: &str, output_path: &str) -> Result<(), String> {
    let broadcast_to_deserialize = read_broadcast_file(input_path)?;
    let broadcast = deserialize_broadcast(&broadcast_to_deserialize)?;
    let broadcast = serialize_broadcast(broadcast)?;
    write_transformed_broadcast_to_file(&broadcast, output_path)?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
pub struct Args {
    #[clap(
        short,
        long,
        help = "Path to the Foundry broadcast file to be transformed. If not provided, the default path is `broadcast/*.s.sol/31337/run-latest.json`. Please note that if you have a couple of directories in the `broadcast` dir, the first one found will be used by default."
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
    pub fn new() -> Self {
        let mut args = Self::parse();
        if args.input_path.is_none() {
            let glob_pattern = "broadcast/*.s.sol/31337/run-latest.json";
            let mut paths = glob(glob_pattern).expect("Failed to match glob pattern");
            let path = paths.next().expect("No matching paths found");
            args.input_path = Some(path.unwrap().to_str().unwrap().to_string());
        }
        if args.output_path.is_none() {
            let dir_path = "src/crytic".to_string();
            create_dir_all(&dir_path).expect("Failed to create directory");
            args.output_path = Some(format!("{}/init.json", dir_path));
        } else {
            let output_path = Path::new(args.output_path.as_ref().unwrap());
            let output_dir = output_path.parent().unwrap();
            create_dir_all(output_dir).expect("Failed to create directory");
        }
        args
    }
}

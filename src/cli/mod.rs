use crate::deserialization::deserialize_broadcast;
use crate::file_handling::{read_broadcast_file, write_transformed_broadcast_to_file};
use crate::serialization::serialize_broadcast;

use clap::Parser;

pub fn run(input_path: &str, output_path: &str) -> Result<(), String> {
    let broadcast_to_deserialize = read_broadcast_file(input_path)?;
    let broadcast = deserialize_broadcast(&broadcast_to_deserialize)?;
    let broadcast = serialize_broadcast(broadcast)?;
    write_transformed_broadcast_to_file(&broadcast, output_path)?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(
    name = "Foundry to Echidna",
    about = "Easily seed Echidna with Foundry broadcast files."
)]

pub struct Args {}

//! # Foundry 2 echidna
//! It lets you transform a Foundry broadcast file into a format that is compatible with Echidna.
//! The transformed broadcast file is in an Etheno-like format, which means that you can easily seed
//! Echidna with it.
//!
//! ## Usage
//! Run the following command in the root of your Foundry project:
//! `foundry2echidna`
//!
//! If no paths are provided, the default paths are:
//! Input path: `broadcast/*.s.sol/31337/run-latest.json`
//! Output path: `src/crytic/init.json`
//!
//! You can also specify the paths manually:
//! `foundry2echidna --input-path <path> --output-path <path>`
//!
//! After you have your transformed broadcast file, you can seed Echidna with it. To do so, add the
//! following to your `echidna_config.yaml` file:
//! `initialize: init.json`
//!
//! If you have any problems with the software, please open an issue on the [GitHub repo](https://github.com/ChmielewskiKamil/foundry2echidna)
pub mod cli;
mod data_model;
mod deserialization;
mod file_handling;
mod serialization;

pub use self::cli::transform_broadcast;

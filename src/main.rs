use foundry2echidna::cli::run;
use std::process;

fn main() {
    run(
        "test_json_files/test_broadcast.json",
        "test_json_files/test_broadcast_transformed.json",
    )
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}

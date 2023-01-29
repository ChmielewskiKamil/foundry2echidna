use foundry2echidna::cli::{run, Args};

fn main() {
    let args = Args::new();

    match run(
        args.input_path.as_ref().unwrap(),
        args.output_path.as_ref().unwrap(),
    ) {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

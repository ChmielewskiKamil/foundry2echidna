use foundry2echidna::cli::{run, Args};

fn main() {
    let args = Args::new();

    if let Err(e) = run(
        args.input_path.as_ref().unwrap(),
        args.output_path.as_ref().unwrap(),
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!("Success");
}

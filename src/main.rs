use foundry2echidna::cli::{run, Args};

fn main() {
    Args::new()
        .map(|args| (args.input_path.unwrap(), args.output_path.unwrap()))
        .and_then(|(input_path, output_path)| run(&input_path, &output_path))
        .map(|_| println!("Success"))
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });
}

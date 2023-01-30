use foundry2echidna::cli::{transform_broadcast, Args};

fn main() {
    Args::new()
        .map(|args| (args.input_path.unwrap(), args.output_path.unwrap()))
        .and_then(|(input_path, output_path)| transform_broadcast(&input_path, &output_path))
        .map(|_| println!("Transformed broadcast successfully!"))
        .unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        });
}

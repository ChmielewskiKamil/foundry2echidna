use foundry2echidna::run;

fn main() {
    run(
        "test_json_files/test_broadcast.json",
        "test_json_files/test_broadcast_transformed.json",
    );
}

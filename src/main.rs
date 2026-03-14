use hash_cli::get_input::get_args;
use hash_cli::hasher;

fn main() {

    let user_input = get_args();

    match user_input {
        Ok(_) => {},
        Err(e) => {
            eprint!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let hash_output = hasher::hash().expect("failed to hash input");

    let hash_hex = hasher::hash_hex(&hash_output);
    println!("Hash output: {}", hash_hex);
}

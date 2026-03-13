use hash_cli::get_input::get_args;

fn main() {

    let user_input = get_args();

    match user_input {
        Ok(_) => {},
        Err(e) => {
            eprint!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

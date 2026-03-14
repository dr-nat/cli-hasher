use std::{
    env,
    error::Error,
};

pub fn get_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguents were provided").into());
    }

    let user_input = args.get(1).ok_or("index out of bounds")?.to_string();

    Ok(user_input)
}

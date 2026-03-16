use std::{
    env,
    error::Error,
    fs::*,
    path::PathBuf,
};

pub fn get_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguents were provided").into());
    }

    let user_input = args.get(1).ok_or("index out of bounds")?;

    let flag_arg = args.get(2).ok_or("invalid flag")?;

    let file_input = PathBuf::from(user_input);

    if flag_arg == "-f" && user_input != "" {
        return Ok(read_to_string(&file_input)?);
    }
    
    Ok(user_input.to_string())
}

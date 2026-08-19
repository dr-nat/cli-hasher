use std::{
    env,
    error::Error,
    fs::*,
    path::PathBuf,
};

//so we use the environment variable to accept arguments from the command line, 
//then we store them in a list which we use a vector, and then check if the argument
//is less than two, cause the name of the program stands as the first argument in the 
//list, and then what ever you put serves as the second argument, so this tool checks 
//if no argument was provided, and if not, then it returns and error message and panics
//so here, we want  
pub fn get_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguents were provided").into());
    }

    let user_input = args.get(1).ok_or("index out of bounds")?;

    if let Some(flag_arg) = args.get(2) {

        let file_input = PathBuf::from(user_input);

        if user_input != "" && flag_arg == "-f" {
            return Ok(read_to_string(&file_input)?);
        }
    }
    
    Ok(user_input.to_string())
}

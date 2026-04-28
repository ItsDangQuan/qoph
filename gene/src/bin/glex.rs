use std::{
    io::{self, Read},
    process::ExitCode,
};

fn main() -> ExitCode {
    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("IO Error: Failed to read from stdin: {}", err);
        return ExitCode::FAILURE;
    }

    let generated_code = match gene::generate_lexer_source(&input) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Codegen Error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    print!("{}", generated_code);

    ExitCode::SUCCESS
}

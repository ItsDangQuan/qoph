use std::{
    env,
    io::{self, Read},
    process::ExitCode,
};

use gene::LexerCompilerOptions;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let mut compiler_options = LexerCompilerOptions::default();
    match args.next() {
        None => {}
        Some(arg) => {
            if args.next().is_some() {
                eprintln!("Argument Error: Too many options.");
                return ExitCode::FAILURE;
            }

            let Some(stripped) = arg.strip_prefix("--") else {
                eprintln!("Argument Error: Invalid option '{}'.", arg);
                return ExitCode::FAILURE;
            };

            let Some((key, value)) = stripped.split_once('=') else {
                eprintln!("Argument Error: Invalid option '{}'.", arg);
                return ExitCode::FAILURE;
            };

            match key {
                "max_expansion_depth" => match value.parse::<usize>() {
                    Ok(num) => {
                        compiler_options.max_expansion_depth = num;
                    }
                    Err(_) => {
                        eprintln!("Argument Error: Invalid value in option '{}'.", arg);
                        return ExitCode::FAILURE;
                    }
                },
                _ => {
                    eprintln!("Argument Error: Invalid option '--{}'.", key);
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    let mut input = String::new();

    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("IO Error: Failed to read from stdin: {}", err);
        return ExitCode::FAILURE;
    }

    let generated_code = match gene::generate_lexer_source(&input, Some(compiler_options)) {
        Ok(code) => code,
        Err(errs) => {
            for err in errs {
                eprintln!("Codegen Error: {}", err);
            }
            return ExitCode::FAILURE;
        }
    };

    print!("{}", generated_code);

    ExitCode::SUCCESS
}

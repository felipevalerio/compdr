use std::io;
use clap::{Arg, Command};

mod cli;

fn main() -> io::Result<()> {
    let matches = Command::new("rlox")
        .version("1.0")
        .about("Lox language interpreter in Rust")
        .arg(
            Arg::new("script")
                .help("The Lox script file to execute")
                .index(1)
                .required(false),
        )
        .get_matches();


    if let Some(script_path) = matches.get_one::<String>("script") {
        cli::run_file(script_path)?;
    } else {
        cli::run_prompt()?;
    }

    Ok(())
}


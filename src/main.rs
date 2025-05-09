use std::{
    env::{self},
    process,
};

use anyhow::Result;
use executor::Executor;
use handler::Handler;

mod command;
mod executor;
mod file;
mod handler;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let handler = Handler::new(args);
    let (cmd, file_path) = handler.run()?;

    let executor = Executor::new(cmd, file_path);
    let cli_result = executor.run()?;

    handler.display(cli_result);

    Ok(())
}

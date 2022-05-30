mod common;
mod cli;

use anyhow::Result;
use chrono::{Local, Datelike};
use clap::{Parser, CommandFactory, ErrorKind};

fn main() -> Result<()> {
    let now = Local::now();
    let args = cli::Cli::parse();
    
    let month: usize = match args.month.unwrap_or(now.month().to_string()).parse() {
        Ok(num) => {
            if num > 12 || num < 1 {
                let mut cmd = cli::Cli::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "Only accept month value range from 1 to 12.",
                )
                .exit();
            }
            num
        }
        Err(_) => {
            let mut cmd = cli::Cli::command();
            cmd.error(
                ErrorKind::InvalidValue,
                "Invalid value for '<MONTH>': invalid digit found in string"
            )
            .exit();
        }
    };

    let year: usize = match args.year.unwrap_or(now.year().to_string()).parse() {
        Ok(num) => {
            if num > 7000 || num < 1645 {
                let mut cmd = cli::Cli::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "Only accept year value range from 1645 to 7000.",
                )
                .exit();
            }
            num
        }
        Err(_) => {
            let mut cmd = cli::Cli::command();
            cmd.error(
                ErrorKind::InvalidValue,
                "Invalid value for '<YEAR>': invalid digit found in string"
            )
            .exit();
        }
    };

    Ok(())
}

//! Lazycoder - A simple snippet generator for expanso
//!
//! `lazycoder start </filepath/demo.lazycoder>`
//! - works with only one demo at a time
//! - save file name
//! - save initial next position: 0
//! - config file location depends on OS. saved in ~/.lazycoder
//!
//! `lazycoder next`
//! - reads from config file
//! - reads next snippet
//! - increments counter to next snippet
//!
//! `lazycoder rewind [number]`
//! - decrements counter (number times)
//! - returns nothing
//!
//! `lazycoder forward [number]`
//! - increments counter (number times)
//! - returns nothing
mod config;
mod lazy_coder_error;
mod snippet_handler;

use clap::{Parser, Subcommand};
use config::Config;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(help_template = "{before-help}{name} {version}
{author-with-newline}{about-section}
{usage-heading} {usage}

{all-args}{after-help}")] // This is required to show the author
struct CliArgs {
    /// Verbosity level
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Use *FILENAME* to provide snippets
    Start {
        /// Path to snippet file
        filename: String,
    },
    /// Provide next snippet
    Next {},
    /// Rewind *n* snippet(s)
    Rewind {
        /// Set n (by default is 1)
        count: Option<usize>,
    },
    /// Forward *n* snippet(s)
    Forward {
        /// Set n (by default is 1)
        count: Option<usize>,
    },
}

fn main() {
    let value = CliArgs::parse();

    match &value.command {
        Command::Start { filename } => {
            start(filename, value.verbose);
        }
        Command::Next {} => {
            next(value.verbose);
        }
        Command::Forward { count } => {
            let count = count.unwrap_or(1);
            forward(count, value.verbose);
        }
        Command::Rewind { count } => {
            let count = count.unwrap_or(1);
            rewind(count, value.verbose);
        }
    }
}

fn start(filename: &str, verbose_level: u8) {
    if verbose_level > 0 {
        println!("Setting to work {}", filename);
    }
    match Config::new(filename, verbose_level) {
        Ok(_) => {
            if verbose_level > 0 {
                eprintln!("Configuration successfully created.");
            }
            exit(0);
        }
        Err(err) => {
            eprintln!("Failed to create configuration: {err}.");
            exit(1);
        }
    }
}

fn next(verbose_level: u8) {
    if verbose_level > 0 {
        eprintln!("Next");
    }
    match Config::read(verbose_level) {
        Ok(mut cfg) => {
            match cfg.next(verbose_level) {
                Ok(snippet) => {
                    print!("{snippet}");
                    exit(0);
                }
                Err(err) => {
                    eprintln!("Failed to obtain next snippet: {err}.");
                    exit(1);
                }
            };
        }
        Err(err) => {
            eprintln!("Failed to obtain next snippet: {err}.");
            exit(1);
        }
    };
}

fn forward(count: usize, verbose_level: u8) {
    if verbose_level > 0 {
        eprintln!("Forward {count}");
    }
    match Config::read(verbose_level) {
        Ok(mut cfg) => {
            if let Err(err) = cfg.forward(count, verbose_level) {
                eprintln!("Failed to foward: {err}.");
                exit(1);
            } else {
                exit(0);
            }
        }
        Err(err) => {
            eprintln!("Failed to foward: {err}.");
            exit(1);
        }
    };
}

fn rewind(count: usize, verbose_level: u8) {
    if verbose_level > 0 {
        eprintln!("Rewind {}", count);
    }
    match Config::read(verbose_level) {
        Ok(mut cfg) => {
            if let Err(err) = cfg.rewind(count, verbose_level) {
                eprintln!("Failed to rewind: {err}.");
                exit(1);
            } else {
                exit(0);
            }
        }
        Err(err) => {
            eprintln!("Failed to rewind: {err}.");
            exit(1);
        }
    };
}

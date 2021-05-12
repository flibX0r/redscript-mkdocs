//use std::fs::File;
//use std::io::{BufReader, BufWriter};
//use std::path::PathBuf;

use gumdrop::Options;
//use log::LevelFilter;
//use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::error::Error;

pub mod error;

#[derive(Debug, Options)]
struct Opts {
}

fn main() -> Result<(), Error> {


    Ok(())
}

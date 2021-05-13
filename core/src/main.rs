use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use askama::Template;
use gumdrop::Options;
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::error::Error;
use crate::doctype::DocumentationType;
use crate::type_enum::{EnumField, Enum, EnumGroup};

pub mod error;
pub mod type_enum;
pub mod doctype;

#[derive(Debug, Options)]
struct Opts {
    #[options(required, short = "i", help = "input bundle file (eg. r6\\cache\\final.redscripts)")]
    input: PathBuf,
    #[options(required, short = "o", help = "output directory (eg. mkdocs\\docs)")]
    output: PathBuf,
}

fn main() -> Result<(), Error> {
    let log_config = simplelog::ConfigBuilder::new()
        .set_time_format_str("")
        .build();

    TermLogger::init(
        LevelFilter::Info,
        log_config,
        TerminalMode::Stdout,
        ColorChoice::Auto
    ).unwrap();

    // Skips the program name
    let args: Vec<String> = std::env::args().skip(1).collect();
    match Opts::parse_args_default(&args) {
        Ok(opts) => {
            run(opts).map_err(|err| {
                log::error!("{:?}", err);
                err
            }
        )},
        Err(err) => {
            log::error!("{}", err);
            log::info!("Usage:\n{}", Opts::usage());
            
            Ok(())
        }
    }
}

fn run(opts: Opts) -> Result<(), Error> {

    let fields1 = vec![
        EnumField {name:"STATUS_INVALID", value: "0"},
        EnumField {name:"STATUS_BOUND", value: "1"},
        EnumField {name:"STATUS_READY", value: "2"},
        EnumField {name:"STATUS_PROGRESS", value: "3"},
        EnumField {name:"STATUS_COMPLETE", value: "4"},
        EnumField {name:"STATUS_FAILURE", value: "5"},
    ];
    let fields2 = vec![
        EnumField {name:"STATUS_INVALID", value: "0"},
        EnumField {name:"STATUS_BOUND", value: "1"},
        EnumField {name:"STATUS_READY", value: "2"},
        EnumField {name:"STATUS_PROGRESS", value: "3"},
        EnumField {name:"STATUS_COMPLETE", value: "4"},
        EnumField {name:"STATUS_FAILURE", value: "5"},
    ];

    let test_enums = vec![
        Enum {
            name: "moveMovementType",
            fields: &fields1
        },
        Enum {
            name: "gameEActionStatus",
            fields: &fields2
        },
    ];

    let enum_group = EnumGroup {
        name: "All Enums",
        enums: &test_enums,
    };

    println!("Enum::get_type_link() = {}", Enum::get_type_link());

    let path = opts.output.as_path().join("testenum.md");
    let mut output = BufWriter::new(File::create(path)?);

    write!(&mut output, "{}", enum_group.render().unwrap())?;

    Ok(())
}

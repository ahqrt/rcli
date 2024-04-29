// rcli csv -i input.csv -o output.json --header -d ","

use std::path::Path;

use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about = "A simple CLI tool for CSV processing", long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")] // default_value => "output.json".into()
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() -> anyhow::Result<()> {
    // parse opts from command line
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;

            let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
            println!("{:?}", records);
        }
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

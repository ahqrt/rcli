use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    nationality: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}

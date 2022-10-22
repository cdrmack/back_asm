use std::error::Error;
use std::fs;

use crate::config;
use crate::parser;

pub fn assembly(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    parser::parse(contents);

    Ok(())
}

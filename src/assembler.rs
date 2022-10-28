use std::error::Error;
use std::fs;
use std::io::Write;

use crate::code;
use crate::config;
use crate::parser;
use crate::parser::InstructionType;

pub fn assembly(config: config::Config) -> Result<(), Box<dyn Error>> {
    let output_filename: String;

    if config.filename.find(".").is_some() {
        let filename: Vec<&str> = config.filename.split(".").collect();
        output_filename = format!("{}.hack", filename[0]);
    } else {
        output_filename = format!("{}.hack", config.filename);
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(output_filename)
        .unwrap();

    let contents = fs::read_to_string(config.filename)?;
    let mut parser = parser::Parser::new(contents);

    while parser.has_more_lines() {
        parser.advance();
        let instruction_type = parser.instruction_type();

        match instruction_type {
            InstructionType::AINSTRUCTION => {
                // println!("A-INSTRUCTION");
                let symbol = parser.symbol();
                // println!("symbol: {}", symbol);
                write!(file, "{}", format!("{}\n", code::variable(&symbol)))?;
            }
            InstructionType::LINSTRUCTION => {
                println!("L-INSTRUCTION");
                let label = parser.symbol();
                println!("label: {}", label);
                // TODO
            }
            InstructionType::CINSTRUCTION => {
                // println!("C-INSTRUCTION");
                let dest = parser.dest();
                let dest_bin = code::dest(&dest);

                let comp = parser.comp();
                let comp_bin = code::comp(&comp);

                let jump = parser.jump();
                let jump_bin = code::jump(&jump);

                // println!("symbolic: {}:{}:{}", dest, comp, jump);
                // println!("  binary: {}:{}:{}", dest_bin, comp_bin, jump_bin);
                write!(
                    file,
                    "{}",
                    format!("111{}{}{}\n", dest_bin, comp_bin, jump_bin)
                )?;
            }
        }
    }

    Ok(())
}

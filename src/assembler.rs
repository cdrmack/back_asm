use std::error::Error;
use std::fs;

use crate::code;
use crate::config;
use crate::parser;
use crate::parser::InstructionType;

pub fn assembly(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let mut parser = parser::Parser::new(contents);

    while parser.has_more_lines() {
        parser.advance();
        let instruction_type = parser.instruction_type();

        match instruction_type {
            InstructionType::AINSTRUCTION => {
                println!("A-INSTRUCTION");
                parser.symbol();
            }
            InstructionType::LINSTRUCTION => {
                println!("L-INSTRUCTION");
                parser.symbol();
            }
            InstructionType::CINSTRUCTION => {
                println!("C-INSTRUCTION");
                let dest = parser.dest();
                println!("dest = {}, cond =, jmp =", dest);
            }
        }
    }

    Ok(())
}

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
                let symbol = parser.symbol();
                println!("symbol: {}", symbol);
            }
            InstructionType::LINSTRUCTION => {
                println!("L-INSTRUCTION");
                let label = parser.symbol();
                println!("label: {}", label);
            }
            InstructionType::CINSTRUCTION => {
                println!("C-INSTRUCTION");
                let dest = parser.dest();
                let dest_bin = code::dest(&dest);
                // println!("dest: {} -> {}", dest, dest_bin);

                let comp = parser.comp();
                let comp_bin = code::comp(&comp);
                // println!("comp: {} -> {}", comp, comp_bin);

                let jump = parser.jump();
                let jump_bin = code::jump(&jump);
                // println!("jump: {} -> {}", jump, jump_bin);

                println!("symbolic: {}:{}:{}", dest, comp, jump);
                println!("  binary: {}:{}:{}", dest_bin, comp_bin, jump_bin);
            }
        }
    }

    Ok(())
}

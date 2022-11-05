pub struct Parser {
    pub lines: Vec<String>,
    pub line_number: u32,
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    AINSTRUCTION,
    CINSTRUCTION,
    LINSTRUCTION,
}

impl Parser {
    pub fn new(contents: String) -> Parser {
        let mut lines: Vec<String> = vec![];

        for line in contents.lines() {
            let trimmed_line = line.trim();
            if should_ignore(&trimmed_line) == false {
                lines.push(String::from(trimmed_line));
            }
        }

        Parser {
            lines,
            line_number: 0,
        }
    }

    pub fn instruction_type(&self) -> InstructionType {
        let line = &self.lines[self.line_number as usize];

        if line.as_bytes()[0] == b'@' {
            return InstructionType::AINSTRUCTION;
        } else if line.as_bytes()[0] == b'(' {
            return InstructionType::LINSTRUCTION;
        }

        InstructionType::CINSTRUCTION
    }

    pub fn has_more_lines(&self) -> bool {
        return self.line_number + 1 < self.lines.len() as u32;
    }

    pub fn advance(&mut self) {
        if self.has_more_lines() == false {
            return;
        }

        self.line_number += 1;
    }

    // @17 -> 17
    // @sum -> sum
    // (LOOP) -> LOOP
    pub fn symbol(&self) -> String {
        let line = &self.lines[self.line_number as usize];

        if line.as_bytes()[0] == b'@' {
            return String::from(line.get(1..).unwrap());
        }

        if line.as_bytes()[0] == b'(' {
            let length = line.len();
            return String::from(line.get(1..length - 1).unwrap());
        }

        String::from("") // TODO: return error
    }

    pub fn dest(&self) -> String {
        let line = &self.lines[self.line_number as usize];

        let mut symbols: Vec<&str> = vec![];

        if line.find("=").is_some() {
            symbols = line.split("=").collect();
        }

        if symbols.is_empty() {
            return String::from(""); // TODO: return error
        }

        String::from(symbols[0])
    }

    pub fn comp(&self) -> String {
        let line = &self.lines[self.line_number as usize];

        let mut result = String::from("");

        let dest = line.find("=");
        let jump = line.find(";");

        if dest.is_some() && jump.is_some() {
            // D=D+1;JLE
            let symbols_dest: Vec<&str> = line.split("=").collect();
            let tmp = String::from(symbols_dest[1]); // D+1;JLE
            let symbols_jump: Vec<&str> = tmp.split(";").collect();
            result = String::from(symbols_jump[0]);
        } else if dest.is_some() {
            //D=D-M
            let symbols: Vec<&str> = line.split("=").collect();
            result = String::from(symbols[1]);
        } else if jump.is_some() {
            // D;JGT
            let symbols: Vec<&str> = line.split(";").collect();
            result = String::from(symbols[0]);
        }

        result
    }

    pub fn jump(&self) -> String {
        let line = &self.lines[self.line_number as usize];

        let mut symbols: Vec<&str> = vec![];

        if line.find(";").is_some() {
            symbols = line.split(";").collect();
        }

        if symbols.is_empty() {
            return String::from(""); // TODO: return error
        }

        String::from(symbols[symbols.len() - 1])
    }
}

fn should_ignore(line: &str) -> bool {
    // ignore empty lines
    if line.len() < 1 {
        return true;
    }

    // ignore comments
    if line.as_bytes()[0] == b'/' {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_empty_file() {
        let input = String::from("");
        let parser = Parser::new(input);
        assert_eq!(0, parser.lines.len());
    }

    #[test]
    fn ignore_file_with_no_content() {
        let input = String::from(
            "

// Comment

",
        );
        let parser = Parser::new(input);
        assert_eq!(0, parser.lines.len());
    }

    #[test]
    fn remove_whitespaces() {
        let input = String::from("  D=M ");
        let parser = Parser::new(input);
        assert_eq!(1, parser.lines.len());
        assert_eq!("D=M", parser.lines[0]);
    }

    #[test]
    fn remove_comment() {
        let input = String::from("// Comment");
        let parser = Parser::new(input);
        assert_eq!(0, parser.lines.len());
    }

    #[test]
    fn remove_empty_lines() {
        let input = String::from(
            "D=M


@8",
        );
        let parser = Parser::new(input);
        assert_eq!(2, parser.lines.len());
        assert_eq!("D=M", parser.lines[0]);
        assert_eq!("@8", parser.lines[1]);
    }

    #[test]
    fn remove_comments() {
        let input = String::from(
            "D=M
// Comment
@8
// Another comment",
        );
        let parser = Parser::new(input);
        assert_eq!(2, parser.lines.len());
        assert_eq!("D=M", parser.lines[0]);
        assert_eq!("@8", parser.lines[1]);
    }

    #[test]
    fn symbol_number() {
        let contents = String::from("@8");
        let parser = Parser::new(contents);
        assert_eq!("8", parser.symbol());
    }

    #[test]
    fn symbol_text() {
        let contents = String::from("@sum");
        let parser = Parser::new(contents);
        assert_eq!("sum", parser.symbol());
    }

    #[test]
    fn symbol_label() {
        let contents = String::from("(LOOP)");
        let parser = Parser::new(contents);
        assert_eq!("LOOP", parser.symbol());
    }

    #[test]
    fn has_more_lines_one_line() {
        let contents = String::from("@foo");
        let parser = Parser::new(contents);
        assert_eq!(false, parser.has_more_lines());
    }

    #[test]
    fn has_more_lines_two_lines() {
        let contents = String::from(
            "@foo
@bar",
        );
        let parser = Parser::new(contents);
        assert_eq!(true, parser.has_more_lines());
    }

    #[test]
    fn advance_two_lines() {
        let contents = String::from(
            "@foo
@bar",
        );
        let mut parser = Parser::new(contents);
        assert_eq!(true, parser.has_more_lines());
        parser.advance();
        assert_eq!(false, parser.has_more_lines());
    }

    #[test]
    fn ignore_non_symbols() {
        let contents = String::from("D=M");
        let parser = Parser::new(contents);
        assert_eq!("", parser.symbol());
    }

    #[test]
    fn return_ainstruction_number() {
        let contents = String::from("@8");
        let parser = Parser::new(contents);
        assert_eq!(InstructionType::AINSTRUCTION, parser.instruction_type());
    }

    #[test]
    fn return_ainstruction_symbol() {
        let contents = String::from("@foo");
        let parser = Parser::new(contents);
        assert_eq!(InstructionType::AINSTRUCTION, parser.instruction_type());
    }

    #[test]
    fn return_linstruction() {
        let contents = String::from("(FOO)");
        let parser = Parser::new(contents);
        assert_eq!(InstructionType::LINSTRUCTION, parser.instruction_type());
    }

    #[test]
    fn return_cinstruction_() {
        let contents = String::from("D=M");
        let parser = Parser::new(contents);
        assert_eq!(InstructionType::CINSTRUCTION, parser.instruction_type());
    }

    #[test]
    fn dest_01() {
        let contents = String::from("D=M");
        let parser = Parser::new(contents);
        assert_eq!("D", parser.dest());
    }

    #[test]
    fn dest_02() {
        let contents = String::from("0;JMP");
        let parser = Parser::new(contents);
        assert_eq!("", parser.dest());
    }

    #[test]
    fn dest_03() {
        let contents = String::from("@foo");
        let parser = Parser::new(contents);
        assert_eq!("", parser.dest());
    }

    #[test]
    fn dest_04() {
        let contents = String::from(
            "D=M
// comment
@foo
M=M+D",
        );
        let mut parser = Parser::new(contents);
        assert_eq!("D", parser.dest());
        parser.advance(); // @foo
        assert_eq!("", parser.dest()); // wrong instruction
        parser.advance(); // M=M+D
        assert_eq!("M", parser.dest());
    }

    #[test]
    fn comp_01() {
        let contents = String::from("D=M");
        let parser = Parser::new(contents);
        assert_eq!("M", parser.comp());
    }

    #[test]
    fn comp_02() {
        let contents = String::from("D=D-M");
        let parser = Parser::new(contents);
        assert_eq!("D-M", parser.comp());
    }

    #[test]
    fn comp_03() {
        let contents = String::from("0;JMP");
        let parser = Parser::new(contents);
        assert_eq!("0", parser.comp());
    }

    #[test]
    fn comp_04() {
        let contents = String::from("D=M;JMP");
        let parser = Parser::new(contents);
        assert_eq!("M", parser.comp());
    }

    #[test]
    fn comp_05() {
        let contents = String::from("D=D+1;JLE");
        let parser = Parser::new(contents);
        assert_eq!("D+1", parser.comp());
    }

    #[test]
    fn jump_01() {
        let contents = String::from("@foo");
        let parser = Parser::new(contents);
        assert_eq!("", parser.jump());
    }

    #[test]
    fn jump_02() {
        let contents = String::from("D=M");
        let parser = Parser::new(contents);
        assert_eq!("", parser.jump());
    }

    #[test]
    fn jump_03() {
        let contents = String::from("0;JEQ");
        let parser = Parser::new(contents);
        assert_eq!("JEQ", parser.jump());
    }

    #[test]
    fn dest_and_jmp() {
        let contents = String::from("D=D+1;JLE");
        let parser = Parser::new(contents);
        assert_eq!("D", parser.dest());
        assert_eq!("JLE", parser.jump());
    }
}

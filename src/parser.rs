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
        let code = contents;
        let mut lines: Vec<String> = vec![];

        for line in code.lines() {
            lines.push(String::from(line));
        }

        Parser {
            lines,
            line_number: 0,
        }
    }

    // assumed we are on valid instruction already
    pub fn instruction_type(&self) -> InstructionType {
        let line = &self.lines[self.line_number as usize].trim();

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

        let mut current_line_index = self.line_number as usize;
        current_line_index += 1;
        let mut line = &self.lines[current_line_index];

        // skip empty lines and comments
        while should_ignore(&line) {
            if current_line_index + 1 < self.lines.len() {
                current_line_index += 1;
                line = &self.lines[current_line_index];
            } else {
                return;
            }
        }

        self.line_number = current_line_index as u32;
    }

    // @17 -> 17
    // @sum -> sum
    // (LOOP) -> LOOP
    pub fn symbol(&self) -> String {
        if self.line_number as usize >= self.lines.len() {
            return String::from("");
        }

        if self.instruction_type() == InstructionType::CINSTRUCTION {
            return String::from("");
        }

        // trim line to remove whitespaces
        let line = &self.lines[self.line_number as usize].trim();

        if line.len() < 1 {
            return String::from("");
        }

        if line.as_bytes()[0] == b'@' {
            return String::from(line.get(1..).unwrap());
        }

        if line.as_bytes()[0] == b'(' {
            let length = line.len();
            return String::from(line.get(1..length - 1).unwrap());
        }

        String::from("")
    }

    pub fn dest(&self) -> String {
        if self.instruction_type() != InstructionType::CINSTRUCTION {
            println!("parser::dest, wrong instruction");
            return String::from("");
        }

        // trim line to remove whitespaces
        let line = &self.lines[self.line_number as usize].trim();

        let mut symbols: Vec<&str> = vec![];

        match line.find("=") {
            None => println!("= not found"),
            Some(index) => {
                println!("= found at {}", index);
                symbols = line.split("=").collect();
            }
        }

        if symbols.is_empty() {
            return String::from("");
        }

        String::from(symbols[0])
    }

    pub fn jump(&self) -> String {
        if self.instruction_type() != InstructionType::CINSTRUCTION {
            println!("parser::jump, wrong instruction");
            return String::from("");
        }

        // trim line to remove whitespaces
        let line = &self.lines[self.line_number as usize].trim();

        let mut symbols: Vec<&str> = vec![];

        match line.find(";") {
            None => println!("= not found"),
            Some(index) => {
                println!("; found at {}", index);
                symbols = line.split(";").collect();
            }
        }

        if symbols.is_empty() {
            return String::from("");
        }

        String::from(symbols[symbols.len() - 1])
    }
}

fn should_ignore(line: &String) -> bool {
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
    fn ignore_empty_line() {
        let contents = String::from("");
        let parser = Parser::new(contents);
        assert_eq!("", parser.symbol());
    }

    #[test]
    fn ignore_comments() {
        let contents = String::from("// FOO");
        let parser = Parser::new(contents);
        assert_eq!("", parser.symbol());
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
    fn symbol_ignore_whitespaces() {
        let contents = String::from("    @8 ");
        let parser = Parser::new(contents);
        assert_eq!("8", parser.symbol());
    }

    #[test]
    fn has_more_lines_empty() {
        let contents = String::from("");
        let parser = Parser::new(contents);
        assert_eq!(false, parser.has_more_lines());
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
    fn has_more_lines_include_empty_line() {
        let contents = String::from(
            "@foo

",
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
    fn advance_ignore_empty_lines() {
        let contents = String::from(
            "@foo

@bar",
        );
        let mut parser = Parser::new(contents);

        assert_eq!("foo", parser.symbol());
        parser.advance();
        assert_eq!("bar", parser.symbol());
    }

    #[test]
    fn advance_ignore_empty_lines_at_the_end() {
        let contents = String::from(
            "@foo

@bar

",
        );
        let mut parser = Parser::new(contents);
        assert_eq!("foo", parser.symbol());
        parser.advance();
        assert_eq!("bar", parser.symbol());
        parser.advance();
        assert_eq!(2, parser.line_number);
        assert_eq!("bar", parser.symbol());
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

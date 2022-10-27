pub fn dest(mnemonic: &str) -> String {
    let mut binary = "";

    match mnemonic {
        "null" => binary = "000",
        "M" => binary = "001",
        "D" => binary = "010",
        "DM" => binary = "011",
        "A" => binary = "100",
        "AM" => binary = "101",
        "AD" => binary = "110",
        "ADM" => binary = "111",
        _ => binary = "000",
    }

    String::from(binary)
}

pub fn comp(mnemonic: &str) -> String {
    let mut binary = "";

    match mnemonic {
        "0" => binary = "0101010",
        "1" => binary = "0111111",
        "-1" => binary = "0111010",
        "D" => binary = "0001100",
        "A" => binary = "0110000",
        "M" => binary = "1110000",
        "!D" => binary = "0001101",
        "!A" => binary = "0110001",
        "!M" => binary = "1110001",
        "-D" => binary = "0001111",
        "-A" => binary = "0110011",
        "-M" => binary = "1110011",
        "D+1" => binary = "0011111",
        "A+1" => binary = "0110111",
        "M+1" => binary = "1110111",
        "D-1" => binary = "0001110",
        "A-1" => binary = "0110010",
        "M-1" => binary = "1110010",
        "D+A" => binary = "0000010",
        "D+M" => binary = "1000010",
        "D-A" => binary = "0010011",
        "D-M" => binary = "1010011",
        "A-D" => binary = "0000111",
        "M-D" => binary = "1000111",
        "D&A" => binary = "0000000",
        "D&M" => binary = "1000000",
        "D|A" => binary = "0010101",
        "D|M" => binary = "1010101",
        _ => binary = "",
    }

    String::from(binary)
}

pub fn jump(mnemonic: &str) -> String {
    let mut binary = "";

    match mnemonic {
        "null" => binary = "000",
        "JGT" => binary = "001",
        "JEQ" => binary = "010",
        "JGE" => binary = "011",
        "JLT" => binary = "100",
        "JNE" => binary = "101",
        "JLE" => binary = "110",
        "JMP" => binary = "111",
        _ => binary = "000",
    }

    String::from(binary)
}

pub fn variable(a_instruction: &str) -> String {
    let mut binary = "";

    // pre-defined
    match a_instruction {
        "R0" => binary = "0000000000000000",
        "R1" => binary = "0000000000000001",
        "R2" => binary = "0000000000000010",
        "R3" => binary = "0000000000000011",
        "R4" => binary = "0000000000000100",
        "R5" => binary = "0000000000000101",
        "R6" => binary = "0000000000000110",
        "R7" => binary = "0000000000000111",
        "R8" => binary = "0000000000001000",
        "R9" => binary = "0000000000001001",
        "R10" => binary = "0000000000001010",
        "R11" => binary = "0000000000001011",
        "R12" => binary = "0000000000001100",
        "R13" => binary = "0000000000001101",
        "R14" => binary = "0000000000001110",
        "R15" => binary = "0000000000001111",
        "SP" => binary = "0000000000000000",
        "LCL" => binary = "0000000000000001",
        "ARG" => binary = "0000000000000010",
        "THIS" => binary = "0000000000000011",
        "THAT" => binary = "0000000000000100",
        "SCREEN" => binary = "0100000000000000",
        "KBD" => binary = "0110000000000000",
        _ => binary = "",
    }

    if binary.is_empty() {
        match a_instruction.parse::<i32>() {
            Ok(number) => {
                return format!("{:016b}", number);
            }
            Err(e) => {
                // TODO
                return String::from("variable");
            }
        }
    } else {
        String::from(binary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dest_null() {
        assert_eq!("000", dest("null"));
    }

    #[test]
    fn dest_m() {
        assert_eq!("001", dest("M"));
    }

    #[test]
    fn dest_d() {
        assert_eq!("010", dest("D"));
    }

    #[test]
    fn dest_dm() {
        assert_eq!("011", dest("DM"));
    }

    #[test]
    fn dest_a() {
        assert_eq!("100", dest("A"));
    }

    #[test]
    fn dest_am() {
        assert_eq!("101", dest("AM"));
    }

    #[test]
    fn dest_ad() {
        assert_eq!("110", dest("AD"));
    }

    #[test]
    fn dest_adm() {
        assert_eq!("111", dest("ADM"));
    }

    #[test]
    fn dest_invalid_symbol() {
        assert_eq!("000", dest("INVALID_SYMBOL"));
    }

    #[test]
    fn comp_0() {
        assert_eq!("0101010", comp("0"));
    }

    #[test]
    fn comp_1() {
        assert_eq!("0111111", comp("1"));
    }

    #[test]
    fn comp_minus1() {
        assert_eq!("0111010", comp("-1"));
    }

    #[test]
    fn comp_d() {
        assert_eq!("0001100", comp("D"));
    }

    #[test]
    fn comp_a() {
        assert_eq!("0110000", comp("A"));
    }

    #[test]
    fn comp_m() {
        assert_eq!("1110000", comp("M"));
    }

    #[test]
    fn comp_not_d() {
        assert_eq!("0001101", comp("!D"));
    }

    #[test]
    fn comp_not_a() {
        assert_eq!("0110001", comp("!A"));
    }

    #[test]
    fn comp_not_m() {
        assert_eq!("1110001", comp("!M"));
    }

    #[test]
    fn comp_minus_d() {
        assert_eq!("0001111", comp("-D"));
    }

    #[test]
    fn comp_minus_a() {
        assert_eq!("0110011", comp("-A"));
    }

    #[test]
    fn comp_minus_m() {
        assert_eq!("1110011", comp("-M"));
    }

    #[test]
    fn comp_d_plus1() {
        assert_eq!("0011111", comp("D+1"));
    }

    #[test]
    fn comp_a_plus1() {
        assert_eq!("0110111", comp("A+1"));
    }

    #[test]
    fn comp_m_plus1() {
        assert_eq!("1110111", comp("M+1"));
    }

    #[test]
    fn comp_d_minus1() {
        assert_eq!("0001110", comp("D-1"));
    }

    #[test]
    fn comp_a_minus1() {
        assert_eq!("0110010", comp("A-1"));
    }

    #[test]
    fn comp_m_minus1() {
        assert_eq!("1110010", comp("M-1"));
    }

    #[test]
    fn comp_d_plus_a() {
        assert_eq!("0000010", comp("D+A"));
    }

    #[test]
    fn comp_d_plus_m() {
        assert_eq!("1000010", comp("D+M"));
    }

    #[test]
    fn comp_d_minus_a() {
        assert_eq!("0010011", comp("D-A"));
    }

    #[test]
    fn comp_d_minus_m() {
        assert_eq!("1010011", comp("D-M"));
    }

    #[test]
    fn comp_a_minus_d() {
        assert_eq!("0000111", comp("A-D"));
    }

    #[test]
    fn comp_m_minus_d() {
        assert_eq!("1000111", comp("M-D"));
    }

    #[test]
    fn comp_d_and_a() {
        assert_eq!("0000000", comp("D&A"));
    }

    #[test]
    fn comp_d_and_m() {
        assert_eq!("1000000", comp("D&M"));
    }

    #[test]
    fn comp_d_or_a() {
        assert_eq!("0010101", comp("D|A"));
    }

    #[test]
    fn comp_d_or_m() {
        assert_eq!("1010101", comp("D|M"));
    }

    #[test]
    fn comp_invalid_symbol() {
        assert_eq!("", comp("INVALID_SYMBOL"));
    }

    #[test]
    fn jump_null() {
        assert_eq!("000", jump("null"));
    }

    #[test]
    fn jump_jgt() {
        assert_eq!("001", jump("JGT"));
    }

    #[test]
    fn jump_jeq() {
        assert_eq!("010", jump("JEQ"));
    }

    #[test]
    fn jump_jge() {
        assert_eq!("011", jump("JGE"));
    }

    #[test]
    fn jump_jlt() {
        assert_eq!("100", jump("JLT"));
    }

    #[test]
    fn jump_jne() {
        assert_eq!("101", jump("JNE"));
    }

    #[test]
    fn jump_jle() {
        assert_eq!("110", jump("JLE"));
    }

    #[test]
    fn jump_jmp() {
        assert_eq!("111", jump("JMP"));
    }

    #[test]
    fn jump_invalid_symbol() {
        assert_eq!("000", dest("INVALID_SYMBOL"));
    }

    #[test]
    fn variable_r0() {
        assert_eq!("0000000000000000", variable("R0"));
    }

    #[test]
    fn variable_r1() {
        assert_eq!("0000000000000001", variable("R1"));
    }

    #[test]
    fn variable_r2() {
        assert_eq!("0000000000000010", variable("R2"));
    }

    #[test]
    fn variable_r3() {
        assert_eq!("0000000000000011", variable("R3"));
    }

    #[test]
    fn variable_r4() {
        assert_eq!("0000000000000100", variable("R4"));
    }

    #[test]
    fn variable_r5() {
        assert_eq!("0000000000000101", variable("R5"));
    }

    #[test]
    fn variable_r6() {
        assert_eq!("0000000000000110", variable("R6"));
    }

    #[test]
    fn variable_r7() {
        assert_eq!("0000000000000111", variable("R7"));
    }

    #[test]
    fn variable_r8() {
        assert_eq!("0000000000001000", variable("R8"));
    }

    #[test]
    fn variable_r9() {
        assert_eq!("0000000000001001", variable("R9"));
    }

    #[test]
    fn variable_r10() {
        assert_eq!("0000000000001010", variable("R10"));
    }

    #[test]
    fn variable_r11() {
        assert_eq!("0000000000001011", variable("R11"));
    }

    #[test]
    fn variable_r12() {
        assert_eq!("0000000000001100", variable("R12"));
    }

    #[test]
    fn variable_r13() {
        assert_eq!("0000000000001101", variable("R13"));
    }

    #[test]
    fn variable_r14() {
        assert_eq!("0000000000001110", variable("R14"));
    }

    #[test]
    fn variable_r15() {
        assert_eq!("0000000000001111", variable("R15"));
    }

    #[test]
    fn variable_sp() {
        assert_eq!("0000000000000000", variable("SP"));
    }

    #[test]
    fn variable_lcl() {
        assert_eq!("0000000000000001", variable("LCL"));
    }

    #[test]
    fn variable_arg() {
        assert_eq!("0000000000000010", variable("ARG"));
    }

    #[test]
    fn variable_this() {
        assert_eq!("0000000000000011", variable("THIS"));
    }

    #[test]
    fn variable_that() {
        assert_eq!("0000000000000100", variable("THAT"));
    }

    #[test]
    fn variable_screen() {
        assert_eq!("0100000000000000", variable("SCREEN"));
    }

    #[test]
    fn variable_kbd() {
        assert_eq!("0110000000000000", variable("KBD"));
    }

    #[test]
    fn variable_42() {
        assert_eq!("0000000000101010", variable("42"));
    }

    #[test]
    fn variable_8() {
        assert_eq!("0000000000001000", variable("8"));
    }

    #[test]
    fn variable_16() {
        assert_eq!("0000000000010000", variable("16"));
    }
}

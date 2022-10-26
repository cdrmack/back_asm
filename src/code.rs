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
        "0" => binary = "101010",
        "1" => binary = "111111",
        "-1" => binary = "111010",
        "D" => binary = "001100",
        "A" => binary = "110000",
	"M" => binary = "110000",
        "!D" => binary = "001101",
        "!A" => binary = "110001",
	"!M" => binary = "110001",
        "-D" => binary = "001111",
	"-A" => binary = "110011",
	"-M" => binary = "110011",
	"D+1" => binary = "011111",
	"A+1" => binary = "110111",
	"M+1" => binary = "110111",
	"D-1" => binary = "001110",
	"A-1" => binary = "110010",
	"M-1" => binary = "110010",
	"D+A" => binary = "000010",
	"D+M" => binary = "000010",
	"D-A" => binary = "010011",
	"D-M" => binary = "010011",
	"A-D" => binary = "000111",
	"M-D" => binary = "000111",
	"D&A" => binary = "000000",
	"D&M" => binary = "000000",
	"D|A" => binary = "010101",
	"D|M" => binary = "010101",
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
        assert_eq!("101010", comp("0"));
    }

    #[test]
    fn comp_1() {
        assert_eq!("111111", comp("1"));
    }

    #[test]
    fn comp_minus1() {
        assert_eq!("111010", comp("-1"));
    }

    #[test]
    fn comp_d() {
        assert_eq!("001100", comp("D"));
    }

    #[test]
    fn comp_a() {
        assert_eq!("110000", comp("A"));
    }

    #[test]
    fn comp_m() {
        assert_eq!("110000", comp("M"));
    }

    #[test]
    fn comp_not_d() {
        assert_eq!("001101", comp("!D"));
    }

    #[test]
    fn comp_not_a() {
        assert_eq!("110001", comp("!A"));
    }

    #[test]
    fn comp_not_m() {
        assert_eq!("110001", comp("!M"));
    }

    #[test]
    fn comp_minus_d() {
        assert_eq!("001111", comp("-D"));
    }

    #[test]
    fn comp_minus_a() {
        assert_eq!("110011", comp("-A"));
    }

    #[test]
    fn comp_minus_m() {
        assert_eq!("110011", comp("-M"));
    }

    #[test]
    fn comp_d_plus1() {
        assert_eq!("011111", comp("D+1"));
    }

    #[test]
    fn comp_a_plus1() {
        assert_eq!("110111", comp("A+1"));
    }

    #[test]
    fn comp_m_plus1() {
        assert_eq!("110111", comp("M+1"));
    }

    #[test]
    fn comp_d_minus1() {
        assert_eq!("001110", comp("D-1"));
    }

    #[test]
    fn comp_a_minus1() {
        assert_eq!("110010", comp("A-1"));
    }
    
    #[test]
    fn comp_m_minus1() {
        assert_eq!("110010", comp("M-1"));
    }
    
    #[test]
    fn comp_d_plus_a() {
        assert_eq!("000010", comp("D+A"));
    }

    #[test]
    fn comp_d_plus_m() {
        assert_eq!("000010", comp("D+M"));
    }
    
    #[test]
    fn comp_d_minus_a() {
        assert_eq!("010011", comp("D-A"));
    }

    #[test]
    fn comp_d_minus_m() {
        assert_eq!("010011", comp("D-M"));
    }
    
    #[test]
    fn comp_a_minus_d() {
        assert_eq!("000111", comp("A-D"));
    }

    #[test]
    fn comp_m_minus_d() {
        assert_eq!("000111", comp("M-D"));
    }
    
    #[test]
    fn comp_d_and_a() {
        assert_eq!("000000", comp("D&A"));
    }

    #[test]
    fn comp_d_and_m() {
        assert_eq!("000000", comp("D&M"));
    }
    
    #[test]
    fn comp_d_or_a() {
        assert_eq!("010101", comp("D|A"));
    }

    #[test]
    fn comp_d_or_m() {
        assert_eq!("010101", comp("D|A"));
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
}

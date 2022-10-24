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
        _ => binary = "",
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
}

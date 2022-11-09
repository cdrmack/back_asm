use std::collections::HashMap;

pub struct SymbolTable {
    pub symbols: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn add_predefined_symbols(&mut self) {
        self.add_entry(String::from("R0"), 0);
        self.add_entry(String::from("R1"), 1);
        self.add_entry(String::from("R2"), 2);
        self.add_entry(String::from("R3"), 3);
        self.add_entry(String::from("R4"), 4);
        self.add_entry(String::from("R5"), 5);
        self.add_entry(String::from("R6"), 6);
        self.add_entry(String::from("R7"), 7);
        self.add_entry(String::from("R8"), 8);
        self.add_entry(String::from("R9"), 9);
        self.add_entry(String::from("R10"), 10);
        self.add_entry(String::from("R11"), 11);
        self.add_entry(String::from("R12"), 12);
        self.add_entry(String::from("R13"), 13);
        self.add_entry(String::from("R14"), 14);
        self.add_entry(String::from("R15"), 15);
        self.add_entry(String::from("SP"), 0);
        self.add_entry(String::from("LCL"), 1);
        self.add_entry(String::from("ARG"), 2);
        self.add_entry(String::from("THIS"), 3);
        self.add_entry(String::from("THAT"), 4);
        self.add_entry(String::from("SCREEN"), 16384);
        self.add_entry(String::from("KBD"), 24576);
    }

    pub fn add_entry(&mut self, symbol: String, address: u32) {
        println!("add_entry, {} -> {}", symbol, address);
        self.symbols.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u32 {
        let result = self.symbols.get(symbol);
        return result.unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_r0() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(0, st.get_address("R0"));
    }

    #[test]
    fn variable_r1() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(1, st.get_address("R1"));
    }

    #[test]
    fn variable_r2() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(2, st.get_address("R2"));
    }

    #[test]
    fn variable_r3() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(3, st.get_address("R3"));
    }

    #[test]
    fn variable_r4() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(4, st.get_address("R4"));
    }

    #[test]
    fn variable_r5() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(5, st.get_address("R5"));
    }

    #[test]
    fn variable_r6() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(6, st.get_address("R6"));
    }

    #[test]
    fn variable_r7() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(7, st.get_address("R7"));
    }

    #[test]
    fn variable_r8() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(8, st.get_address("R8"));
    }

    #[test]
    fn variable_r9() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(9, st.get_address("R9"));
    }

    #[test]
    fn variable_r10() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(10, st.get_address("R10"));
    }

    #[test]
    fn variable_r11() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(11, st.get_address("R11"));
    }

    #[test]
    fn variable_r12() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(12, st.get_address("R12"));
    }

    #[test]
    fn variable_r13() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(13, st.get_address("R13"));
    }

    #[test]
    fn variable_r14() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(14, st.get_address("R14"));
    }

    #[test]
    fn variable_r15() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(15, st.get_address("R15"));
    }

    #[test]
    fn variable_sp() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(0, st.get_address("SP"));
    }

    #[test]
    fn variable_lcl() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(1, st.get_address("LCL"));
    }

    #[test]
    fn variable_arg() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(2, st.get_address("ARG"));
    }

    #[test]
    fn variable_this() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(3, st.get_address("THIS"));
    }

    #[test]
    fn variable_that() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(4, st.get_address("THAT"));
    }

    #[test]
    fn variable_screen() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(16384, st.get_address("SCREEN"));
    }

    #[test]
    fn variable_kbd() {
        let mut st = SymbolTable::new();
        st.add_predefined_symbols();
        assert_eq!(24576, st.get_address("KBD"));
    }
}

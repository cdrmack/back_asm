pub fn parse(contents: String) -> Vec<String> {
    let mut parsed_result = vec![];

    for line in contents.lines() {
        // remove white spaces
        let trimmed_line = line.trim();

        if should_ignore(String::from(trimmed_line)) {
            // noop
        } else {
            parsed_result.push(String::from(trimmed_line));
        }
    }

    for element in parsed_result.iter() {
        println!("{}", element);
    }

    parsed_result
}

fn should_ignore(line: String) -> bool {
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
        let line = String::from("");
        assert_eq!(true, should_ignore(line));
    }

    #[test]
    fn ignore_comments() {
        let line = String::from("// FOO");
        assert_eq!(true, should_ignore(line));
    }

    #[test]
    fn valid_code() {
        let line = String::from("D=M");
        assert_eq!(false, should_ignore(line));
    }

    #[test]
    fn valid_symbol() {
        let line = String::from("@8");
        assert_eq!(false, should_ignore(line));
    }
}

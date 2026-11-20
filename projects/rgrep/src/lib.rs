pub fn search<'a>(input: &'a str, matching_string: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for line in input.lines() {
        if line.contains(matching_string) {
            out.push(line)
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_word() {
        let contents = "first line\nsecond line\nthird";
        let contents_3 = "Rust: \ntrust me.";

        assert_eq!(search(contents, "first"), vec!["first line"]);
        assert_eq!(search(contents, "line"), vec!["first line", "second line"]);
        assert_eq!(search(contents_3, "Rust"), vec!["Rust: "]);
        assert_eq!(search(contents_3, ""), vec!["Rust: ", "trust me."]);
    }

    #[test]
    fn no_match() {
        let contents = "first line\nsecond line\nthird";

        assert_eq!(search(contents, "zzz"), Vec::<&str>::new());
    }

    #[test]
    fn inner_chars() {
        let contents = "first line\nsecond line\nthird";

        assert_eq!(search(contents, "cond"), vec!["second line"]);
    }
}

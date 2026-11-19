pub fn check(text: &str, content: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        if line.rfind(content).is_some() {
            out.push_str(line)
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let contents = "\
        first line
        second line
        third";

        assert_eq!(check(contents, "first"), "first line")
    }
}

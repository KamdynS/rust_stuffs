use std::io::BufRead;
use std::io::Error;

pub fn search<T: BufRead>(mut file: T, pattern: &str) -> Result<Vec<String>, Error> {
    let mut out = Vec::new();
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.contains(pattern) {
            out.push(std::mem::take(&mut line));
        }
        line.clear();
    }

    Ok(out)
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

//! Pattern 1 (docs/01-grep.md): reading lines from a file or stdin.
//! Toy task: count the lines that contain a `t`.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Counts the lines from any buffered reader that contain a `t`.
pub fn count_t_lines<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut count = 0;
    for line in reader.lines() {
        if line?.contains('t') {
            count += 1;
        }
    }
    Ok(count)
}

/// Counts the lines in the file at `path` that contain a `t`.
pub fn count_t_lines_in_file(path: &Path) -> io::Result<usize> {
    count_t_lines(BufReader::new(File::open(path)?))
}

/// Counts the lines on stdin that contain a `t`.
pub fn count_t_lines_in_stdin() -> io::Result<usize> {
    count_t_lines(io::stdin().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_only_lines_with_t() {
        let input = "cat\ndog\nbat\n\nturtle";
        assert_eq!(count_t_lines(input.as_bytes()).unwrap(), 3);
    }

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(count_t_lines("".as_bytes()).unwrap(), 0);
    }

    #[test]
    fn crlf_endings_are_handled() {
        assert_eq!(count_t_lines("at\r\nno\r\n".as_bytes()).unwrap(), 1);
    }

    #[test]
    fn invalid_utf8_is_an_error_not_a_panic() {
        let input: &[u8] = b"fine\n\xff\xfe\n";
        assert!(count_t_lines(input).is_err());
    }
}

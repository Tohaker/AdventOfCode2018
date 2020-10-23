use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_lines() {
        let filename = "./inputs/test/test.txt";
        let expected = vec![
            String::from("line 1"),
            String::from("line 2"),
            String::from("line 3"),
        ];

        if let Ok(lines) = read_lines(filename) {
            let result: Vec<String> = lines.map(|l| l.unwrap()).collect();
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn should_return_none() {
        let filename = "./inputs/test/non-existent.txt";

        assert!(read_lines(filename).is_err(), "File should not be read");
    }
}

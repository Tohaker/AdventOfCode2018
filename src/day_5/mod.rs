use crate::common;

fn check_unit(unit: &str) -> bool {
    let mut chars: Vec<char> = unit.chars().collect();
    chars.sort_unstable();

    chars[0].is_uppercase()
        && chars[1].is_lowercase()
        && (chars[0].to_ascii_lowercase() == chars[1].to_ascii_lowercase())
}

fn process_polymer(polymer: &str) -> String {
    let mut result: String = polymer.to_string();

    loop {
        let chars: Vec<char> = result.chars().collect();
        let mut changed = 0;
        if !chars.is_empty() {
            for i in 0..chars.len() - 1 {
                let unit: String = chars[i..=i + 1].iter().collect();
                if check_unit(unit.as_str()) {
                    result.replace_range(i..=i + 1, "");
                    changed += 1;
                    break;
                }
            }
        }

        if changed == 0 {
            break;
        }
    }

    result
}

pub fn part_1() {
    let filename = "./inputs/day_5/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let polymer = lines
            .map(|l| l.expect("Could not parse line"))
            .collect::<String>();
        println!("Day 5 - Part 1: {}", process_polymer(&polymer).len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_units() {
        let input = "aA";
        assert!(check_unit(input));

        let input = "Aa";
        assert!(check_unit(input));
    }

    #[test]
    fn should_not_match_units() {
        let input = "AA";
        assert!(!check_unit(input));

        let input = "aa";
        assert!(!check_unit(input));

        let input = "Ab";
        assert!(!check_unit(input));
    }

    #[test]
    fn should_destroy_polymers() {
        let input = "abBA";
        assert_eq!("".to_string(), process_polymer(input));

        let input = "abAB";
        assert_eq!("abAB".to_string(), process_polymer(input));

        let input = "aabAAB";
        assert_eq!("aabAAB".to_string(), process_polymer(input));

        let input = "dabAcCaCBAcCcaDA";
        assert_eq!("dabCBAcaDA".to_string(), process_polymer(input));
    }
}

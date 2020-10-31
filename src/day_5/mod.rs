use crate::common;
use std::collections::HashMap;

fn check_unit(unit: &str) -> bool {
    let mut chars: Vec<char> = unit.chars().collect();
    chars.sort_unstable();

    chars[0].is_uppercase()
        && chars[1].is_lowercase()
        && (chars[0].to_ascii_lowercase() == chars[1].to_ascii_lowercase())
}

fn process_polymer(polymer: &str) -> String {
    let mut buffer: Vec<char> = Vec::new();

    for c in polymer.chars() {
        if !buffer.is_empty() && check_unit(&format!("{}{}", c, buffer.last().unwrap())) {
            buffer.pop();
        } else {
            buffer.push(c);
        }
    }

    buffer.iter().collect()
}

fn shortest_possible_polymer(polymer: &str) -> usize {
    let current = process_polymer(&polymer);
    let lowercase = current.to_ascii_lowercase();

    let mut frequency: HashMap<char, u32> = HashMap::new();
    for c in lowercase.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }

    let present_letters: Vec<char> = frequency
        .iter()
        .filter(|&(_, v)| *v > 0)
        .map(|(k, _)| *k)
        .collect();

    let mut lowest = current.len();

    for l in present_letters.iter() {
        let new_polymer = polymer.replace(&[*l, l.to_ascii_uppercase()][..], "");
        let len = process_polymer(&new_polymer).len();

        if len < lowest {
            lowest = len;
        }
    }

    lowest
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

pub fn part_2() {
    let filename = "./inputs/day_5/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let polymer = lines
            .map(|l| l.expect("Could not parse line"))
            .collect::<String>();
        println!("Day 5 - Part 2: {}", shortest_possible_polymer(&polymer));
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
    #[test]
    fn should_get_shortest_possible_polymer() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(4, shortest_possible_polymer(input));
    }
}

use crate::common;
use std::collections::HashMap;
mod candidate;

fn list_combinations(input: &String) -> candidate::Box {
    let mut has_two = false;
    let mut has_three = false;

    let mut map: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for (_, value) in &map {
        if *value == 2 {
            has_two = true;
        } else if *value == 3 {
            has_three = true;
        }
    }

    candidate::Box { has_two, has_three }
}

fn calculate_checksum(input: &Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for i in input.iter() {
        let b = list_combinations(i);
        let (next_two, next_three) = b.count();
        twos += next_two;
        threes += next_three;
    }

    twos * threes
}

pub fn part_1() {
    let filename = "./inputs/day_2/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!("Day 2 - Part 1: {}", calculate_checksum(&instructions));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_no_combinations() {
        let input = "abcdef".to_string();

        assert_eq!(
            candidate::Box {
                has_two: false,
                has_three: false
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_find_both_combinations() {
        let input = "bababc".to_string();

        assert_eq!(
            candidate::Box {
                has_two: true,
                has_three: true
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_find_it_has_two_only() {
        let input = "abbcde".to_string();

        assert_eq!(
            candidate::Box {
                has_two: true,
                has_three: false
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_find_it_has_three_only() {
        let input = "abcccd".to_string();

        assert_eq!(
            candidate::Box {
                has_two: false,
                has_three: true
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_find_two_counted_twice() {
        let input = "aabcdd".to_string();

        assert_eq!(
            candidate::Box {
                has_two: true,
                has_three: false
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_find_three_counted_twice() {
        let input = "ababab".to_string();

        assert_eq!(
            candidate::Box {
                has_two: false,
                has_three: true
            },
            list_combinations(&input)
        );
    }

    #[test]
    fn should_count_all_twos_and_threes() {
        let input = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];

        assert_eq!(12, calculate_checksum(&input));
    }
}

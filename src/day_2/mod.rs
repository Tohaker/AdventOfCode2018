use crate::common;
use std::collections::HashMap;
use std::convert::TryInto;
mod candidate;

fn list_combinations(input: &str) -> candidate::Box {
    let mut has_two = false;
    let mut has_three = false;

    let mut map: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for value in map.values() {
        if *value == 2 {
            has_two = true;
        } else if *value == 3 {
            has_three = true;
        }
    }

    candidate::Box { has_two, has_three }
}

/// Returns whether the boxes match (by a one char difference) and whatever the first match index was.
fn compare_boxes(box1: &str, box2: &str) -> (bool, i32) {
    let mut count = 0;
    let mut index = -1;
    let box1_chars: Vec<char> = box1.chars().collect();
    let box2_chars: Vec<char> = box2.chars().collect();

    if box1.len() != box2.len() {
        return (false, index);
    }

    for i in 0..(box1.len() - 1) {
        if box1_chars[i] != box2_chars[i] {
            count += 1;
            if index == -1 {
                index = i.try_into().unwrap();
            }
        }
    }

    (count == 1, index)
}

fn calculate_checksum(input: Vec<String>) -> i32 {
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

fn common_letters(input: Vec<String>) -> String {
    let mut result = String::new();

    'outer: for s1 in input.iter() {
        for s2 in input.iter() {
            let (correct, index) = compare_boxes(s1, s2);
            if correct {
                // Convert the index to a u32 for string indexing.
                let index = index.try_into().unwrap();
                result = format!("{}{}", &s1[..index], &s1[index + 1..]);
                break 'outer;
            }
        }
    }

    result
}

pub fn part_1() {
    let filename = "./inputs/day_2/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!("Day 2 - Part 1: {}", calculate_checksum(instructions));
    }
}

pub fn part_2() {
    let filename = "./inputs/day_2/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!("Day 2 - Part 2: {}", common_letters(instructions));
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

        assert_eq!(12, calculate_checksum(input));
    }

    #[test]
    fn should_compare_two_correct_boxes() {
        let box1 = "fghij";
        let box2 = "fguij";

        let (correct, index) = compare_boxes(box1, box2);
        assert!(correct);
        assert_eq!(2, index);
    }

    #[test]
    fn should_compare_two_incorrect_boxes() {
        let box1 = "abcde";
        let box2 = "axcye";

        let (correct, index) = compare_boxes(box1, box2);
        assert!(!correct);
        assert_eq!(1, index);
    }

    #[test]
    fn should_compare_two_different_lengths() {
        let box1 = "abcdef";
        let box2 = "abcde";

        let (correct, index) = compare_boxes(box1, box2);
        assert!(!correct);
        assert_eq!(-1, index);
    }

    #[test]
    fn should_find_common_letters() {
        let input = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];
        let answer = "fgij".to_string();
        assert_eq!(answer, common_letters(input));
    }
}

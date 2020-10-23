use crate::common;
use std::collections::HashMap;

fn parse_frequency_change(input: &str, value: i32) -> i32 {
    let modifier = &input[..1];
    let change: i32 = input[1..].parse().unwrap();

    if modifier == "+" {
        return value + change;
    } else if modifier == "-" {
        return value - change;
    }
    0
}

fn calculate_total(start: i32, instructions: Vec<String>) -> i32 {
    let mut result = start;

    for s in instructions.iter() {
        result = parse_frequency_change(s, result);
    }

    result
}

fn calculate_first_repetition(start: i32, instructions: Vec<String>) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    let mut result = start;

    loop {
        for s in instructions.iter() {
            match map.get(&result) {
                Some(_) => return result,
                None => map.insert(result, 0),
            };

            result = parse_frequency_change(s, result);
        }
    }
}

pub fn part_1() {
    let filename = "./inputs/day_1/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!("Day 1 - Part 1: {}", calculate_total(0, instructions));
    }
}

pub fn part_2() {
    let filename = "./inputs/day_1/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!(
            "Day 1 - Part 2: {}",
            calculate_first_repetition(0, instructions)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_increment_value() {
        let input = "+1";
        let value = 0;

        assert_eq!(1, parse_frequency_change(&input, value));
    }

    #[test]
    fn should_decrement_value() {
        let input = "-4";
        let value = 0;

        assert_eq!(-4, parse_frequency_change(&input, value));
    }

    #[test]
    fn should_adjust_over_all_values() {
        let list1 = vec![
            String::from("+1"),
            String::from("-2"),
            String::from("+3"),
            String::from("+1"),
        ];
        let answer1 = 3;
        assert_eq!(answer1, calculate_total(0, list1));

        let list2 = vec![String::from("+1"), String::from("+1"), String::from("+1")];
        let answer2 = 3;
        assert_eq!(answer2, calculate_total(0, list2));

        let list3 = vec![String::from("+1"), String::from("+1"), String::from("-2")];
        let answer3 = 0;
        assert_eq!(answer3, calculate_total(0, list3));

        let list4 = vec![String::from("-1"), String::from("-2"), String::from("-3")];
        let answer4 = -6;
        assert_eq!(answer4, calculate_total(0, list4));
    }

    #[test]
    fn should_find_same_value_twice() {
        let list1 = vec![String::from("+1"), String::from("-1")];
        let answer1 = 0;
        assert_eq!(answer1, calculate_first_repetition(0, list1));

        let list2 = vec![
            String::from("+3"),
            String::from("+3"),
            String::from("+4"),
            String::from("-2"),
            String::from("-4"),
        ];
        let answer2 = 10;
        assert_eq!(answer2, calculate_first_repetition(0, list2));

        let list3 = vec![
            String::from("-6"),
            String::from("+3"),
            String::from("+8"),
            String::from("+5"),
            String::from("-6"),
        ];
        let answer3 = 5;
        assert_eq!(answer3, calculate_first_repetition(0, list3));

        let list4 = vec![
            String::from("+7"),
            String::from("+7"),
            String::from("-2"),
            String::from("-7"),
            String::from("-4"),
        ];
        let answer4 = 14;
        assert_eq!(answer4, calculate_first_repetition(0, list4));
    }
}

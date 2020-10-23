use crate::common;

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

pub fn part_1() {
    let filename = "./inputs/day_1/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let instructions = lines.map(|l| l.expect("Could not parse line")).collect();
        println!("Day 1 - Part 1: {}", calculate_total(0, instructions));
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
}

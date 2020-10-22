pub fn parse_frequency_change(input: &str, value: i32) -> i32 {
    let modifier = &input[..1];
    let change: i32 = input[1..].parse().unwrap();

    if modifier == "+" {
        return value + change;
    } else if modifier == "-" {
        return value - change;
    }
    0
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
}

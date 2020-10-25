use guard::Schedule;
mod guard;

fn order_records(input: Vec<String>) -> Vec<Schedule> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_order_records() {
        let input = vec![
            "[1518-06-27 00:21] falls asleep".to_string(),
            "[1518-06-05 00:46] falls asleep".to_string(),
            "[1518-11-10 23:52] Guard #881 begins shift".to_string(),
        ];

        let expected = vec![
            "[1518-06-05 00:46] falls asleep".to_string(),
            "[1518-06-27 00:21] falls asleep".to_string(),
            "[1518-11-10 23:52] Guard #881 begins shift".to_string(),
        ];

        assert_eq!(expected, order_records(input));
    }
}

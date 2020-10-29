use crate::common;
use chrono::Timelike;
use guard::Schedule;
use std::collections::HashMap;
mod guard;

fn order_records(input: Vec<String>) -> Vec<Schedule> {
    let mut schedules: Vec<Schedule> = input.iter().map(|s| Schedule::new(s.to_string())).collect();
    schedules.sort();
    schedules.reverse();
    schedules
}

// Key: Guard ID, Value: (Sleep length, Most common minute)
fn determine_shifts(input: Vec<Schedule>) -> HashMap<u32, (u32, u32, u32)> {
    // Map the Guard ID to all the minutes he's asleep, and how many times he is asleep at that minute.
    let mut minute_tracker: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut start_min = 0;
    let mut end_min;
    let mut last_id = 0;

    for schedule in input.iter() {
        let msg = schedule.message();
        let time = schedule.datetime().minute();

        if msg.contains("Guard") {
            let start_idx = msg.find('#').unwrap();
            let end_idx = msg.find("begins").unwrap();

            last_id = (&msg[start_idx + 1..end_idx - 1]).parse::<u32>().unwrap();
            minute_tracker.entry(last_id).or_insert_with(HashMap::new);
            continue;
        }

        if msg == "falls asleep" {
            start_min = time;
            continue;
        }

        if msg == "wakes up" {
            end_min = time;

            for i in start_min..end_min {
                minute_tracker
                    .entry(last_id)
                    .and_modify(|map| *map.entry(i).or_insert(0) += 1);
            }
            continue;
        }
    }

    let mut result: HashMap<u32, (u32, u32, u32)> = HashMap::new();

    for (key, value) in minute_tracker.iter() {
        let (most_common_min, common_min_count) = value
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or((&0, &0));
        let total_minutes: u32 = value.values().sum();

        result.insert(*key, (total_minutes, *most_common_min, *common_min_count));
    }

    result
}

fn calculate_strategy_1(input: HashMap<u32, (u32, u32, u32)>) -> u32 {
    let mut max_len = 0;
    let mut current_guard = 0;
    let mut current_common = 0;

    for (guard, (length, common, _)) in input.iter() {
        if max_len < *length {
            max_len = *length;
            current_guard = *guard;
            current_common = *common;
        }
    }

    current_common * current_guard
}

fn calculate_strategy_2(input: HashMap<u32, (u32, u32, u32)>) -> u32 {
    let mut max_count = 0;
    let mut current_guard = 0;
    let mut current_min = 0;

    for (guard, (_, minute, count)) in input.iter() {
        if max_count < *count {
            max_count = *count;
            current_guard = *guard;
            current_min = *minute;
        }
    }

    current_min * current_guard
}

pub fn part_1() {
    let filename = "./inputs/day_4/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let input = lines.map(|l| l.expect("Could not parse line")).collect();
        let sorted = order_records(input);
        let shifts = determine_shifts(sorted);
        let result = calculate_strategy_1(shifts);
        println!("Day 4 - Part 1: {}", result);
    }
}

pub fn part_2() {
    let filename = "./inputs/day_4/input.txt";

    if let Ok(lines) = common::read_lines(filename) {
        let input = lines.map(|l| l.expect("Could not parse line")).collect();
        let sorted = order_records(input);
        let shifts = determine_shifts(sorted);
        let result = calculate_strategy_2(shifts);
        println!("Day 4 - Part 2: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn should_order_records() {
        let input = vec![
            "[1518-06-27 00:21] falls asleep".to_string(),
            "[1518-06-05 00:46] falls asleep".to_string(),
            "[1518-11-10 23:52] Guard #881 begins shift".to_string(),
        ];

        let result = order_records(input);

        assert_eq!(
            &NaiveDate::from_ymd(1518, 6, 5).and_hms(0, 46, 0),
            result[0].datetime()
        );

        assert_eq!(
            &NaiveDate::from_ymd(1518, 6, 27).and_hms(0, 21, 0),
            result[1].datetime()
        );

        assert_eq!(
            &NaiveDate::from_ymd(1518, 11, 10).and_hms(23, 52, 0),
            result[2].datetime()
        );
    }

    #[test]
    fn should_separate_guard_shifts() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ];

        let sorted = order_records(input);
        let shifts = determine_shifts(sorted);

        let mut expected: HashMap<u32, (u32, u32, u32)> = HashMap::new();
        expected.insert(10, (50, 24, 2));
        expected.insert(99, (30, 45, 3));

        assert_eq!(expected, shifts);
    }

    #[test]
    fn should_find_sleepiest_guard() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ];

        let sorted = order_records(input);
        let shifts = determine_shifts(sorted);
        let result = calculate_strategy_1(shifts);

        assert_eq!(240, result);
    }

    #[test]
    fn should_find_sleepiest_guard_part_2() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up".to_string(),
        ];

        let sorted = order_records(input);
        let shifts = determine_shifts(sorted);
        let result = calculate_strategy_2(shifts);

        assert_eq!(4455, result);
    }
}

use chrono::{NaiveDate, NaiveDateTime};
use std::cmp;

#[derive(Eq, Debug)]
pub struct Schedule {
    datetime: NaiveDateTime,
    message: String,
}

impl Schedule {
    pub fn new(line: String) -> Schedule {
        let date_str = &line[1..17];
        let message = (&line[19..]).to_string();

        let datetime = match NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
            Ok(datetime) => datetime,
            Err(_) => NaiveDate::from_ymd(1518, 1, 1).and_hms(0, 0, 0),
        };

        Schedule { datetime, message }
    }

    pub fn datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl PartialOrd for Schedule {
    fn partial_cmp(&self, other: &Schedule) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Schedule {
    fn cmp(&self, other: &Schedule) -> cmp::Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Schedule) -> bool {
        self.datetime == other.datetime && self.message == other.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn should_create_schedule() {
        let line = "[1518-06-27 00:21] falls asleep".to_string();
        let result = Schedule {
            datetime: NaiveDate::from_ymd(1518, 6, 27).and_hms(0, 21, 0),
            message: "falls asleep".to_string(),
        };

        assert_eq!(result, Schedule::new(line));
    }

    #[test]
    fn should_order_schedules() {
        let mut input = vec![
            Schedule::new("[1518-06-27 00:21] falls asleep".to_string()),
            Schedule::new("[1518-06-05 00:46] falls asleep".to_string()),
            Schedule::new("[1518-11-10 23:52] Guard #881 begins shift".to_string()),
        ];
        let expected = vec![
            Schedule {
                message: "falls asleep".to_string(),
                datetime: NaiveDate::from_ymd(1518, 6, 5).and_hms(0, 46, 0),
            },
            Schedule {
                message: "falls asleep".to_string(),
                datetime: NaiveDate::from_ymd(1518, 6, 27).and_hms(0, 21, 0),
            },
            Schedule {
                message: "Guard #881 begins shift".to_string(),
                datetime: NaiveDate::from_ymd(1518, 11, 10).and_hms(23, 52, 0),
            },
        ];
        input.sort();
        input.reverse();

        assert_eq!(input, expected);
    }
}

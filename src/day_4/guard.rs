use chrono::{NaiveDate, NaiveDateTime};

pub struct Schedule {
    datetime: NaiveDateTime,
    message: String,
}

impl Schedule {
    pub fn new(line: String) -> Schedule {
        let date_str = &line[1..16];
        let message = (&line[18..]).to_string();

        let datetime = match NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
            Ok(datetime) => datetime,
            Err(_) => NaiveDate::from_ymd(1518, 1, 1).and_hms(0, 0, 0),
        };

        Schedule { datetime, message }
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
}

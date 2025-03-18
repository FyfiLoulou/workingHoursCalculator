use crate::utils::log;
use chrono::{DateTime, Duration, Local};

pub struct Workday {
    date: DateTime<Local>,
    entry_time: String,
    end_time: DateTime<Local>,
}

impl Workday {
    fn get_sys_date() -> DateTime<Local> {
        Local::now()
    }

    fn calculate_end_time(entry_time: String) -> DateTime<Local> {
        let parsed = DateTime::parse_from_str(&entry_time, "%H:%M").expect("failed to parse date");
        let working_time = Duration::hours(8) + Duration::minutes(30);
        parsed.with_timezone(&Local) + working_time
    }

    fn new(entry_time: String) -> Workday {
        Workday {
            date: Self::get_sys_date(),
            entry_time: entry_time.clone(),
            end_time: Self::calculate_end_time(entry_time),
        }
    }

    pub fn log(&self) -> std::io::Result<()> {
        let log_message = format!(
            "Date: {}, Entry time: {}, End time: {}",
            self.date.format("%Y, %B %d"),
            self.entry_time,
            self.end_time.format("%H:%M")
        );
        log::log_to_file(log_message)
    }
}

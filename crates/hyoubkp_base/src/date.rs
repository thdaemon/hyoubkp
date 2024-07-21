use chrono::{Datelike, Local};

#[derive(Debug, Clone, Copy)]
pub struct Date(u16, u8, u8);

impl From<&str> for Date {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() == 3 {
            let year = parts[0].parse::<u16>().unwrap_or_default();
            let month = parts[1].parse::<u8>().unwrap_or_default();
            let day = parts[2].parse::<u8>().unwrap_or_default();
            Date(year, month, day)
        } else {
            Date::default()
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Date(1970, 1, 1)
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
    }
}

impl Date {
    pub fn today() -> Self {
        let now = Local::now();
        Date(now.year() as u16, now.month() as u8, now.day() as u8)
    }
}
use chrono::{Datelike, NaiveDate};

pub fn is_leap_year(yyyy: i32) -> bool {
    yyyy % 4 == 0 && yyyy % 100 != 0 || yyyy % 400 == 0
}

pub fn get_days_in_month(t: NaiveDate) -> u32 {
    let mm: u32 = t.month();
    let yyyy: i32 = t.year();
    28 + ((mm + (mm / 8)) % 2) + 2 % mm + 2 * (1 / mm) + ((mm == 2) && is_leap_year(yyyy)) as u32
}

pub fn get_eom(t: NaiveDate) -> NaiveDate {
    t.with_day(get_days_in_month(t)).unwrap()
}

pub fn get_current_year_end_of_february(t: NaiveDate) -> NaiveDate {
    let yyyy: i32 = t.year();
    NaiveDate::from_ymd_opt(yyyy, 2, 28 + is_leap_year(yyyy) as u32).unwrap()
}
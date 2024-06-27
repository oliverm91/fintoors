use chrono::{Datelike, NaiveDate, Months};

pub fn get_days_in_month(t: NaiveDate) -> u32 {
    let t_year = t.year();
    let t_month = t.month();
    NaiveDate::from_ymd_opt(t_year, t_month + 1, 1)
    .unwrap_or_else(|| NaiveDate::from_ymd_opt(t_year + 1, 1, 1).unwrap())
    .pred_opt().unwrap().day()
}

pub fn get_eom(t: NaiveDate) -> NaiveDate {
    (NaiveDate::from_ymd_opt(t.year(), t.month(), 1).unwrap() + Months::new(1)).pred_opt().unwrap()
}

pub fn get_current_year_end_of_february(t: NaiveDate) -> NaiveDate {
    let year = t.year();
    let is_leap_year = t.leap_year();
    NaiveDate::from_ymd_opt(year, 2, if is_leap_year { 29 } else { 28 }).unwrap()
}
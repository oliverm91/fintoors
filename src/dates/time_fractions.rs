use chrono::{Datelike, NaiveDate};
use crate::dates::day_counting::{DayCounter, ActualCounter};
use crate::dates::aux_funcs::is_leap_year;

#[allow(dead_code)]
pub trait TimeFractionCalc {
    fn time_fraction(&self, start_date: NaiveDate, end_date: NaiveDate) -> f64;
    fn time_fraction_vector(&self, start_date: NaiveDate, end_dates: &Vec<NaiveDate>) -> Vec<f64> {
        end_dates.iter().map(|end_date| self.time_fraction(start_date, *end_date)).collect()
    }
}

pub struct FixedBaseTimeFractionCalc {
    pub day_counter: Box<dyn DayCounter>,
    pub base: f64,
}
impl TimeFractionCalc for FixedBaseTimeFractionCalc{
    fn time_fraction(&self, start_date: NaiveDate, end_date: NaiveDate) -> f64 {
        self.day_counter.day_count(start_date, end_date) as f64 / self.base
    }
    fn time_fraction_vector(&self, start_date: NaiveDate, end_dates: &Vec<NaiveDate>) -> Vec<f64> {
        let days = self.day_counter.day_count_vector(start_date, end_dates);
        days.iter().map(|&x| x as f64 / self.base).collect()  // use intrinsics here
    }
}

pub struct ActualActualISDA {
    day_counter: ActualCounter
}
impl TimeFractionCalc for ActualActualISDA {
    fn time_fraction(&self, start_date: NaiveDate, end_date: NaiveDate) -> f64 {
        let start_year = start_date.year();
        let end_year = end_date.year();

        let start_year_days = if start_date.leap_year() { 366 } else { 365 };
    
        if start_year == end_year {
            return self.day_counter.day_count(start_date, end_date) as f64 / start_year_days as f64;
        }
    
        let mut year_fraction = 0.0;
    
        // Days from start date to the end of the start year
        let end_of_start_year = NaiveDate::from_ymd_opt(start_year, 12, 31).unwrap();
        year_fraction += self.day_counter.day_count(start_date, end_of_start_year) as f64 / start_year_days as f64;
    
        // Days from the beginning of the end year to the end date
        let start_of_end_year = NaiveDate::from_ymd_opt(end_year, 1, 1).unwrap();
        let end_year_days = if end_date.leap_year() { 366 } else { 365 };
        year_fraction += self.day_counter.day_count(start_of_end_year, end_date) as f64 / end_year_days as f64;
    
        // Days for the full years in between
        for year in (start_year + 1)..end_year {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            year_fraction += 1.0 * days_in_year as f64 / days_in_year as f64;
        }    
        year_fraction
    }
}

impl ActualActualISDA {
    pub fn new() -> Self {
        ActualActualISDA{
            day_counter: ActualCounter{}
        }
    }    
}
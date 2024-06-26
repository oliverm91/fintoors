use chrono::{Datelike, NaiveDate};
use std::cmp::min;


fn get_day_in_month(t: NaiveDate) -> u32 {
    let t_year = t.year();
    let t_month = t.month();
    NaiveDate::from_ymd_opt(t_year, t_month + 1, 1)
    .unwrap_or_else(|| NaiveDate::from_ymd_opt(t_year + 1, 1, 1).unwrap())
    .pred_opt().unwrap().day()
}

fn get_current_year_end_of_february(t: NaiveDate) -> NaiveDate {
    let year = t.year();
    let is_leap_year = t.leap_year();
    NaiveDate::from_ymd_opt(year, 2, if is_leap_year { 29 } else { 28 }).unwrap()
}

pub trait DayCounter {
    fn day_count(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32;

    fn day_count_vector(&self, start_date: NaiveDate, end_dates: &Vec<NaiveDate>) -> Vec<i32> {
        end_dates.iter().map(|end_date| self.day_count(start_date, *end_date)).collect()
    }
}

pub struct ActualCounter;
impl DayCounter for ActualCounter {
    fn day_count(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32 {
        return end_date.num_days_from_ce() - start_date.num_days_from_ce();
    }
    
    fn day_count_vector(&self, start_date: NaiveDate, end_dates: &Vec<NaiveDate>) -> Vec<i32> {
        let sdi = start_date.num_days_from_ce();
        end_dates.iter().map(|end_date| end_date.num_days_from_ce() - sdi).collect()
    }
}

pub trait Days30Backend {
    fn get_d1(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32;
    fn get_d2(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32;
}
pub struct Days30Counter {
    pub backend: Box<dyn Days30Backend>
}

impl DayCounter for Days30Counter {
    fn day_count(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32 {
        let d1 = self.backend.get_d1(start_date, end_date);
        let d2 = self.backend.get_d2(start_date, end_date);

        360 * (end_date.year() - start_date.year()) + 30 * (end_date.month() as i32 - start_date.month() as i32) + d2 - d1
    }
}

pub struct Days30BondCounter;
impl Days30Backend for Days30BondCounter {
    fn get_d1(&self, start_date: NaiveDate, _: NaiveDate) -> i32 {
        let d1: i32 = start_date.day() as i32;
        min(d1, 30)        
    }
    fn get_d2(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32 {
        let d1: i32 = self.get_d1(start_date, end_date);
        let mut d2: i32 = end_date.day() as i32;
        if d1 > 29 {
            d2 = min(d2, 30);
        }
        d2
    }
}

pub struct Days30ECounter ;
impl Days30Backend for Days30ECounter {
    fn get_d1(&self, start_date: NaiveDate, _: NaiveDate) -> i32 {
        let d1: i32 = start_date.day() as i32;
        min(d1, 30)
    }
    fn get_d2(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32 {
        let d1 = self.get_d1(start_date, end_date);
        let mut d2: i32 = end_date.day() as i32;
        if d1 > 29 {
            d2 = min(d2, 30);
        }
        d2
    }
}

pub struct Days30UCounter;
impl Days30Backend for Days30UCounter {
    fn get_d1(&self, start_date: NaiveDate, _: NaiveDate) -> i32 {
        let start_last_day_feb: NaiveDate = get_current_year_end_of_february(start_date);
        let d1: i32 = start_date.day() as i32;
        if start_date==start_last_day_feb {
            30
        }
        else {
            min(d1, 30)
        }        
    }
    fn get_d2(&self, start_date: NaiveDate, end_date: NaiveDate) -> i32 {
        let start_last_day_feb: NaiveDate = get_current_year_end_of_february(start_date);
        let end_last_day_feb: NaiveDate = get_current_year_end_of_february(end_date);
        
        let mut d1: i32 = start_date.day() as i32;
        let mut d2: i32 = end_date.day() as i32;
        if (start_date==start_last_day_feb) && (end_date==end_last_day_feb) {
            d2 = 30;
        }
        if start_date==start_last_day_feb {
            d1 = 30;
        }
        if (d2==31) && (d1==30 || d1 == 31){
            d2 = 30;
        }
        d2
    }
}

pub struct Days30EISDACounter;
impl Days30Backend for Days30EISDACounter {
    fn get_d1(&self, start_date: NaiveDate, _: NaiveDate) -> i32 {
        let s_eom: u32 = get_day_in_month(start_date);
        
        let mut d1: u32 = start_date.day();
        if d1 == s_eom {
            d1 = 30;
        }
        d1 as i32
    }

    fn get_d2(&self, _: NaiveDate, end_date: NaiveDate) -> i32 {
        let e_eom: u32 = get_day_in_month(end_date);

        let mut d2: u32 = end_date.day();
        if d2 == e_eom {
            d2 = 30;
        }
        d2 as i32
    }
}
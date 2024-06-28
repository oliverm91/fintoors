use chrono::{Datelike, Duration, Months, NaiveDate};
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::dates::date_adjusting::DateAdjustingMethod;
use crate::dates::aux_funcs::get_eom;


lazy_static!{
    static ref TENOR_MAP: HashMap<&'static str, (u8, char)> = {
        let mut map = HashMap::new();
        
        map.insert("1D", (1, 'D'));
        map.insert("2D", (2, 'D'));
        map.insert("1BD", (1, 'B'));
        map.insert("2BD", (2, 'B'));
        map.insert("1W", (1, 'W'));
        map.insert("2W", (2, 'W'));
        map.insert("3W", (3, 'W'));
        map.insert("1M", (1, 'M'));
        map.insert("2M", (2, 'M'));
        map.insert("3M", (3, 'M'));
        map.insert("4M", (4, 'M'));
        map.insert("5M", (5, 'M'));
        map.insert("6M", (6, 'M'));
        map.insert("7M", (7, 'M'));
        map.insert("8M", (8, 'M'));
        map.insert("9M", (9, 'M'));
        map.insert("10M", (10, 'M'));
        map.insert("11M", (11, 'M'));
        map.insert("12M", (12, 'M'));
        map.insert("1Y", (12, 'M'));
        map.insert("18M", (18, 'M'));        
        map.insert("1Y6M", (18, 'M'));
        map.insert("2Y", (2, 'Y'));
        map.insert("3Y", (3, 'Y'));
        map.insert("4Y", (4, 'Y'));
        map.insert("5Y", (5, 'Y'));
        map.insert("10Y", (10, 'Y'));
        map.insert("20Y", (20, 'Y'));
        map.insert("25Y", (25, 'Y'));
        map.insert("30Y", (30, 'Y'));
        map.insert("35Y", (35, 'Y'));
        map.insert("40Y", (40, 'Y'));
        map.insert("45Y", (45, 'Y'));
        map.insert("50Y", (50, 'Y'));
        
        map
    };
    static ref TENOR_UNIT_FUNC_MAP: HashMap<char, fn(&Tenor, NaiveDate, u8) -> NaiveDate> = {
        let mut map = HashMap::new();
        map.insert('B', Tenor::add_business_days as fn(&Tenor, NaiveDate, u8) -> NaiveDate);
        map.insert('D', Tenor::add_days as fn(&Tenor, NaiveDate, u8) -> NaiveDate);
        map.insert('W', Tenor::add_weeks as fn(&Tenor, NaiveDate, u8) -> NaiveDate);
        map.insert('M', Tenor::add_months as fn(&Tenor, NaiveDate, u8) -> NaiveDate);
        map.insert('Y', Tenor::add_years as fn(&Tenor, NaiveDate, u8) -> NaiveDate);
        map
    };
}

#[allow(dead_code)]
pub struct Tenor {
    value: u8,
    unit: char
}

impl Tenor {
    pub fn new(input: &str) -> Option<Self> {
        let tenor_map = &*TENOR_MAP;
        let input = input.trim().to_uppercase();
        
        if let Some(&(value, unit)) = tenor_map.get(&input[..]) {
            Some(Tenor {
                value,
                unit
            })
        } else {
            // Implement manual. Unit must be 'D', 'W', 'M' or 'Y'. Value must be u8 different from 0.
            None
        }
    }
}

impl Tenor {
    pub fn add_to_date(&self, date: NaiveDate, adjusting_method: Option<&dyn DateAdjustingMethod>, end_of_month_roll: Option<bool>) -> NaiveDate {
        let mut future_date: NaiveDate = if let Some(func) = TENOR_UNIT_FUNC_MAP.get(&self.unit) {
            func(self, date, self.value)
        } else {
            panic!("Unexpected value. Admitted values are 'D', 'W', 'M' and 'Y'.")
        };
        if let Some(mut eom_roll) = end_of_month_roll {
            if eom_roll {
                let date_eom: NaiveDate = get_eom(date);
                if date==date_eom {
                    future_date = get_eom(future_date);
                }
                eom_roll = !eom_roll;
            }
            if !eom_roll {
                let date_day: u32 = date.day();
                if date_day==28 {
                    future_date = future_date.with_day(28).unwrap();
                }
                if date_day==31 {
                    future_date = future_date.with_day(31)
                    .unwrap_or(future_date.with_day(30)
                    .unwrap_or(future_date.with_day(29)
                    .unwrap_or(future_date.with_day(28)
                    .unwrap())));
                }
            }
        }
        if let Some(adjuster) = adjusting_method {
            return adjuster.adjust(future_date);
        }
        else {
            return future_date;
        }
    }

    fn add_business_days(&self, date: NaiveDate, amount: u8) -> NaiveDate {

    }
    fn add_days(&self, date: NaiveDate, amount: u8) -> NaiveDate {
        date + Duration::days(amount as i64)
    }
    fn add_weeks(&self, date: NaiveDate, amount: u8) -> NaiveDate {
        self.add_days(date, amount * 7)
    }
    fn add_months(&self, date: NaiveDate, amount: u8) -> NaiveDate {
        date + Months::new(amount as u32)
    }
    fn add_years(&self, date: NaiveDate, amount: u8) -> NaiveDate {
        self.add_months(date, amount * 12)
    }
}


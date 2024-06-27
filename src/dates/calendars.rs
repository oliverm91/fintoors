use chrono::{Datelike, NaiveDate, Weekday, Month, Months, Duration};

pub trait HolidayRule {
    fn get_date(&self, year: i32) -> NaiveDate;
}

// Used for holidays like Columbus day (second monday of october => OrdinalWeekWeekdayRule::new(2, Weekday::Mon, Month::October))
pub struct OrdinalWeekWeekdayRule {
    pub month: u32,
    pub ordinal: u8,
    pub weekday: Weekday,
}
impl OrdinalWeekWeekdayRule {
    pub fn new(ordinal: u8, weekday: Weekday, month: Month) -> Self {
        OrdinalWeekWeekdayRule {
            month: month as u32 + 1,
            weekday: weekday,
            ordinal: ordinal
        }
    }
}
impl HolidayRule for OrdinalWeekWeekdayRule {
    fn get_date(&self, year: i32) -> NaiveDate {
        let mut date = NaiveDate::from_ymd_opt(year, self.month, 1).unwrap();

        // Inefficient but it does 6 iterations in worst case scenario.
        while date.weekday() != self.weekday {
            date = date.succ_opt().unwrap();
        }
        date + chrono::Duration::weeks(self.ordinal as i64 - 1)
    }
}

struct LastWeekWeekdayRule{
    month: u32,
    weekday: Weekday
}
impl LastWeekWeekdayRule{
    pub fn new(weekday: Weekday, month: Month) -> Self {
        LastWeekWeekdayRule{
            month: month as u32 + 1,
            weekday: weekday
        }
    }
}
impl HolidayRule for LastWeekWeekdayRule {
    fn get_date(&self, year: i32) -> NaiveDate {
        let mut last_day = (NaiveDate::from_ymd_opt(year, self.month, 1).unwrap() + Months::new(1)).pred_opt().unwrap();

        //(self.weekday - last_day.weekday())%7
        while last_day.weekday() != self.weekday {
            last_day = last_day.pred_opt().unwrap();
        }
        last_day
    }
}

// Used for holidays like Independance day (4th of July => MonthDayRule::new(Month::July, 4))
pub struct MonthDayRule {
    pub month: u32,
    pub day: u8,
}
impl MonthDayRule {
    pub fn new(month: Month, day: u8) -> Self {
        MonthDayRule {
            month: month as u32 + 1,
            day: day
        }
    }
}
impl HolidayRule for MonthDayRule {
    fn get_date(&self, year: i32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, self.month, self.day as u32).unwrap()
    }
}

fn easter_sunday(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;    
    
    NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}
pub struct MondayEasterRule;
impl HolidayRule for MondayEasterRule {
    fn get_date(&self, year: i32) -> NaiveDate {
        let es: NaiveDate = easter_sunday(year);
        es.succ_opt().unwrap()
    }
}
pub struct FridayEasterRule;
impl HolidayRule for FridayEasterRule {
    fn get_date(&self, year: i32) -> NaiveDate {
        let es: NaiveDate = easter_sunday(year);
        es - Duration::days(2)
    }
}

pub struct Calendar {
    pub holidays: Vec<NaiveDate>,
    pub holiday_rules: Vec<Box<dyn HolidayRule>>
}
impl Calendar {
    pub fn new(holiday_rules: Option<Vec<Box<dyn HolidayRule>>>, holidays: Option<Vec<NaiveDate>>) -> Self {
        let mut h_vec: Vec<NaiveDate> = holidays.unwrap_or_default();
        h_vec.sort();
        h_vec.dedup();
        Calendar{
            holidays: h_vec, 
            holiday_rules: holiday_rules.unwrap_or_default()
        }
    }

    pub fn delete_holidays(&mut self) {
        self.holidays.clear();
    }

    pub fn add_holidays_with_rules(&mut self, start_year: i32, end_year: i32) {
        let mut year = start_year;
        while year <= end_year {
            for rule in &self.holiday_rules {
                self.holidays.push(rule.get_date(year));
            }
            year += 1;
        }
        self.fix_holidays();
    }
    
    pub fn add_holidays_from_vec(&mut self, holidays_vec: Vec<NaiveDate>) {
        self.holidays.extend(holidays_vec);
        self.fix_holidays();
    }

    pub fn add_holiday(&mut self, holiday: NaiveDate) {
        self.holidays.push(holiday);
        self.fix_holidays();
    }

    fn fix_holidays(&mut self) {
        self.holidays.sort();
        self.holidays.dedup();
    }

    pub fn is_holiday(self, date: NaiveDate) -> bool {
        self.holidays.contains(&date)
    }
}

pub fn get_ny_calendar(holidays: Option<Vec<NaiveDate>>, start_year: Option<i32>, end_date: Option<i32>) -> Calendar {
    let rules: Vec<Box<dyn HolidayRule>> = vec![
        Box::new(MonthDayRule::new(Month::January, 1)), // New Year's Day
        Box::new(OrdinalWeekWeekdayRule::new(3, Weekday::Mon, Month::January)), // Martin Luther King Jr. Day
        Box::new(OrdinalWeekWeekdayRule::new(3, Weekday::Mon, Month::February)), // Presidents' Day
        Box::new(FridayEasterRule), // Good Friday
        Box::new(LastWeekWeekdayRule::new(Weekday::Mon, Month::May)), // Memorial Day
        Box::new(MonthDayRule::new(Month::June, 19)), // Juneteenth
        Box::new(MonthDayRule::new(Month::July, 4)), // Independence Day
        Box::new(OrdinalWeekWeekdayRule::new(1, Weekday::Mon, Month::September)), // Labor Day
        Box::new(OrdinalWeekWeekdayRule::new(2, Weekday::Mon, Month::October)), // Columbus Day
        Box::new(MonthDayRule::new(Month::November, 11)), // Veterans Day
        Box::new(OrdinalWeekWeekdayRule::new(4, Weekday::Thu, Month::November)), // Thanksgiving Day
        Box::new(MonthDayRule::new(Month::December, 25)), // Christmas Day
    ];
    let mut ny_c = Calendar::new(Some(rules), holidays);
    match (start_year, end_date) {
        (Some(sy), Some(ey))  => {
            if ey >= sy {
                ny_c.add_holidays_with_rules(sy, ey)
            } else {
                panic!("If start_year and end_year are set, start_year must be smaller or equal to end_year.");
            }
        },
        (Some(_), None)  | (None, Some(_)) => {
            panic!("If start_year or end_year are set, both must be set.")
        },
        (None, None) => ()
    }
    ny_c
}
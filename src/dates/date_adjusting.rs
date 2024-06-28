use chrono::{Datelike, NaiveDate, Weekday};

use crate::dates::calendars::Calendar;

#[allow(dead_code)]
pub trait DateAdjustingMethod {
    fn adjust(&self, date: NaiveDate) -> NaiveDate;
}

pub struct Following<'a> {
    calendar: &'a Calendar,
}
impl<'a> Following<'a> {
    pub fn new(calendar: &'a Calendar) -> Self {
        Following { calendar }
    }
}
impl<'a> DateAdjustingMethod for Following<'a> {
    fn adjust(&self, date: NaiveDate) -> NaiveDate {
        self.calendar.add_business_days(date, 1)
    }
}

pub struct ModifiedFollowing<'a> {
    f_adj: Following<'a>,
    p_adj: Preceding<'a>,
}
impl<'a> ModifiedFollowing<'a> {
    pub fn new(calendar: &'a Calendar) -> Self {
        let f_adj = Following::new(calendar);
        let p_adj = Preceding::new(calendar);
        ModifiedFollowing { f_adj, p_adj }
    }
}
impl<'a> DateAdjustingMethod for ModifiedFollowing<'a> {
    fn adjust(&self, date: NaiveDate) -> NaiveDate {
        let adjusted_date = self.f_adj.adjust(date);
        if adjusted_date.month() != date.month() {
            return self.p_adj.adjust(date);
        }
        adjusted_date
    }
}

pub struct Preceding<'a> {
    calendar: &'a Calendar,
}
impl<'a> Preceding<'a> {
    pub fn new(calendar: &'a Calendar) -> Self {
        Preceding { calendar }
    }
}
impl<'a> DateAdjustingMethod for Preceding<'a> {
    fn adjust(&self, date: NaiveDate) -> NaiveDate {
        self.calendar.substract_business_days(date, 1)
    }
}

pub struct ModifiedPreceding<'a> {
    f_adj: Following<'a>,
    p_adj: Preceding<'a>,
}

impl<'a> ModifiedPreceding<'a> {
    pub fn new(calendar: &'a Calendar) -> Self {
        let f_adj = Following::new(calendar);
        let p_adj = Preceding::new(calendar);
        ModifiedPreceding { f_adj, p_adj }
    }
}

impl<'a> DateAdjustingMethod for ModifiedPreceding<'a> {
    fn adjust(&self, date: NaiveDate) -> NaiveDate {
        let adjusted_date = self.p_adj.adjust(date);
        if adjusted_date.month() != date.month() {
            return self.f_adj.adjust(date);
        }
        adjusted_date
    }
}
use chrono::{Datelike, Duration, NaiveDate};

use crate::dates::tenors::Tenor;
use crate::dates::date_adjusting::DateAdjustingMethod;
use crate::dates::calendars::Calendar;
use crate::dates::aux_funcs::get_eom;

pub enum PaymentFrequency {
    Zero,
    Monthly = Tenor::new("1M"),
    Quarterly = Tenor::new("3M"),
    Semiannually = Tenor::new("6M"),
    Annually = Tenor::new("6M")
}

pub fn create_maturities_schedule(trade_date: NaiveDate, spot_lag: i64, maturity_tenor: Tenor, payment_frequency: PaymentFrequency
                                , stub_period_first: bool, end_of_month_roll: bool) -> Vec<NaiveDate> {
    let effective_date: NaiveDate = trade_date + Duration::days(spot_lag);
    let maturity_date_pre_eom_roll_adj: NaiveDate = maturity_tenor.add_to_date(effective_date, None);
    if !end_of_month_roll {
        
    } else {
        let effective_date_is_eom: bool = get_eom(effective_date) == effective_date;
    }
}
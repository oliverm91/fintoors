use chrono::{Datelike, Duration, NaiveDate};

use crate::dates::tenors::Tenor;
use crate::dates::date_adjusting::DateAdjustingMethod;
use crate::dates::calendars::Calendar;
use crate::dates::aux_funcs::get_eom;

pub enum CouponFrequency {
    Zero,
    Monthly = Tenor::new("1M"),
    Quarterly = Tenor::new("3M"),
    Semiannually = Tenor::new("6M"),
    Annually = Tenor::new("6M")
}

pub enum StubType {
    ShortFirst,
    ShortLast
}

pub enum RollType {
    Forward,
    ForwardEOM,
    Backward,
    BackwardEOM
}

fn get_forward_date(trade_date: NaiveDate, forward_lag: i64, forward_date_adjustment: Option<DateAdjustingMethod>) -> NaiveDate {
    let forward_date_pre = trade_date + Duration::days(forward_lag);
    if let fda = Some(forward_date_adjustment) {
        return fda.adjust(forward_date_pre);
    }
    else {
        return forward_date_pre;
    }
}

fn get_effective_date(trade_date: NaiveDate, forward_lag: i64, spot_lag: i64, payment_lag: i64
                    , effective_date_adjustment: Option<DateAdjustingMethod>
                    , forward_date_adjustment: Option<DateAdjustingMethod>) -> NaiveDate {
    let forward_date = get_forward_date(trade_date, forward_lag, forward_date_adjustment);
    
    let effective_date_pre = forward_date + Duration::days(spot_lag);
    if let fda = Some(forward_date_adjustment) {
        return fda.adjust(forward_date_pre);
    }
    else {
        return forward_date_pre;
    }
}

pub fn create_maturities_schedule(trade_date: NaiveDate, forward_lag: i64, spot_lag: i64, payment_lag: i64
                                , maturity_tenor: Tenor, payment_frequency: CouponFrequency
                                , stub_type: StubType, roll_type: RollType
                                , coupon_adjustment: DateAdjustingMethod
                                , effective_date_adjustment: Option<DateAdjustingMethod>
                                , forward_date_adjustment: Option<DateAdjustingMethod>) -> Vec<NaiveDate> {
    let effective_date_pre: NaiveDate = trade_date + Duration::days(spot_lag);
    if let edam = Some(effective_date_adjustment) {
        let effective_date = edam.adjust(effective_date_pre);
    }
    else {
        let effective_date = effective_date_pre;
    }
    
    let maturity_date_pre_eom_roll_adj: NaiveDate = maturity_tenor.add_to_date(effective_date, None);
    if !end_of_month_roll {

    } else {
        let effective_date_is_eom: bool = get_eom(effective_date) == effective_date;
    }
}
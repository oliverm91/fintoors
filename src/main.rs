use chrono::{Months, NaiveDate};
use std::time::Instant;

mod dates;
use dates::time_fractions::{ActualActualISDA, FixedBaseTimeFractionCalc, TimeFractionCalc};
use dates::day_counting::{ActualCounter, Days30Counter, Days30ECounter};
use dates::date_adjusting::{ModifiedFollowing, Following, Preceding, ModifiedPreceding, DateAdjustingMethod};
use dates::calendars::{Calendar, get_ny_calendar};
use dates::tenors::Tenor;


fn main() {
    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 12).unwrap();
    let mut end_dates: Vec<NaiveDate> = Vec::new();
    for i in 1..=20 {
        end_dates.push(start_date + Months::new(6*i));
    }
    
    println!("Actual/360:");
    let ac: ActualCounter = ActualCounter;
    let actf = FixedBaseTimeFractionCalc{day_counter: Box::new(ac), base: 360.0};
    let start_time = Instant::now();
    let tfv: Vec<f64> = actf.time_fraction_vector(start_date, &end_dates);
    let duration = start_time.elapsed();
    println!("{:?}", duration); 
    println!("{:?}", tfv);
    println!("---------");

    println!("Days 30E/360:");
    let d30ec: Days30Counter = Days30Counter{backend: Box::new(Days30ECounter{})};
    let d30tf = FixedBaseTimeFractionCalc{day_counter: Box::new(d30ec), base: 360.0};
    let start_time = Instant::now();
    let tfv: Vec<f64> = d30tf.time_fraction_vector(start_date, &end_dates);
    let duration = start_time.elapsed();
    println!("{:?}", duration);
    println!("{:?}", tfv);
    
    
    println!("---------");
    println!("Days Actual/Actual ISDA:");
    let aatf = ActualActualISDA::new();
    let start_time = Instant::now();
    let tfv: Vec<f64> = aatf.time_fraction_vector(start_date, &end_dates);
    let duration = start_time.elapsed();
    println!("{:?}", duration);
    println!("{:?}", tfv);
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2024,2,29).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2023,2,28).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2024,3, 31).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2024,4, 30).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, None));

    println!("---");
    println!("---");
    println!("---");


    let ny_calendar = get_ny_calendar(None, Some(2000), Some(2100));
    let tenor_date = NaiveDate::from_ymd_opt(2024,2,29).unwrap();
    let mf = ModifiedFollowing::new(&ny_calendar);
    let dadjm: Option<&dyn DateAdjustingMethod> = Some(&mf);
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2023,2,28).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2024,3, 31).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    println!("---");
    println!("---");

    let tenor_date = NaiveDate::from_ymd_opt(2024,4, 30).unwrap();
    let a: Tenor = Tenor::new("12m").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
    let a: Tenor = Tenor::new("3w").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));

    let tenor_date = NaiveDate::from_ymd_opt(2024,5, 31).unwrap();
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));

    let tenor_date = NaiveDate::from_ymd_opt(2023,3, 23).unwrap();
    let a: Tenor = Tenor::new("1y").unwrap();
    println!("{}", a.add_to_date(tenor_date, dadjm));
}
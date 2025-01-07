use chrono::{DateTime, Local, TimeZone};

pub static mut REALTIME_RETURN_PER_SECOND: f64 = 0.0;
pub static mut REALTIME_CURRENCY: f64 = 0.0;
pub static mut REALTIME_TOTAL_INVESTED: f64 = 0.0;

pub static mut ALL_INVESTMENTS: Vec<(DateTime<Local>, f64, f64, String)> = Vec::new();
pub static mut MUTABLE_ALL_INVESTMENTS: Vec<(DateTime<Local>, f64, f64, String)> = Vec::new();
pub static mut RETURN_PER_INVESTMENT: f64 = 10.00;
pub static mut TOTAL_INVESTED_PER_INVESTMENT: f64 = 1000.0;
pub static mut INVESTMENT_NAME: String = String::new();

pub static mut YEAR: i32 = 2025;
pub static mut MONTH: u32 = 1;
pub static mut DAY: u32 = 1;


#[allow(static_mut_refs)]
pub fn add_investment()
{
    unsafe
    {
        if INVESTMENT_NAME.replace(" ", "").is_empty()
        {
            INVESTMENT_NAME = "Generic Investment".to_string();
        }

        ALL_INVESTMENTS.push
        (
            (
                Local.with_ymd_and_hms(YEAR, MONTH, DAY, 0, 0, 0).unwrap(),
                RETURN_PER_INVESTMENT, 
                TOTAL_INVESTED_PER_INVESTMENT,
                INVESTMENT_NAME.clone()
             )
        );

        MUTABLE_ALL_INVESTMENTS.push
        (
            (
                Local.with_ymd_and_hms(YEAR, MONTH, DAY, 0, 0, 0).unwrap(),
                RETURN_PER_INVESTMENT, 
                TOTAL_INVESTED_PER_INVESTMENT,
                INVESTMENT_NAME.clone()
             )
        );

        YEAR = 2025;
        MONTH = 1;
        DAY = 1;
        RETURN_PER_INVESTMENT = 10.0;
        TOTAL_INVESTED_PER_INVESTMENT = 1000.0;
        INVESTMENT_NAME.clear();
   };
}

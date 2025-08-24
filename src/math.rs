use chrono::Local;
use crate::investment_wallet::{MUTABLE_ALL_INVESTMENTS, ALL_INVESTMENTS, REALTIME_CURRENCY, REALTIME_RETURN_PER_SECOND, REALTIME_TOTAL_INVESTED};





//CALCULATOR PAGE
pub static mut ONLINE_HISTORIC_RETURN_VALUE: Vec<String> = Vec::new();
pub static mut TOTAL_INVESTED: f64 =       1000.0;
pub static mut RETURN_VALUE: f64 =         10.0;
pub static mut YEARS_INVESTED: f64 =       1.0;
pub static mut MONTHS_INVESTED: f64 =      1.0;
pub static mut DAYS_INVESTED: f64 =        1.0;
pub static mut HOURS_INVESTED: f64 =       1.0;
pub static mut MINUTES_INVESTED: f64 =     1.0;
pub static mut SECS_INVESTED: f64 =        1.0;
pub static mut MONTHLY_CONTRIBUTION: f64 = 0.0;





#[allow(static_mut_refs)]
pub fn calculator_maths() -> (f64, f64, f64, f64, f64 ,f64)
{
    let total_invested = unsafe{TOTAL_INVESTED};
    let year_return_value: f64 = unsafe{RETURN_VALUE / 100.0};

    let month_return_value  = f64::powf(1.0 + year_return_value,   1.00 / 12.00) - 1.0;
    let day_return_value    = f64::powf(1.0 + month_return_value,  1.00 / 30.00) - 1.0;
    let hour_return_value   = f64::powf(1.0 + day_return_value,    1.00 / 24.00) - 1.0;
    let minute_return_value = f64::powf(1.0 + hour_return_value,   1.00 / 60.00) - 1.0;
    let secs_return_value   = f64::powf(1.0 + minute_return_value, 1.00 / 60.00) - 1.0;

    let formula_year: f64   = (total_invested * f64::powf(1.0 + year_return_value,   unsafe{YEARS_INVESTED})   + unsafe{MONTHLY_CONTRIBUTION  * 12.540536612264} * (f64::powf(1.0 + year_return_value,  unsafe{YEARS_INVESTED}) - 1.0)  / year_return_value - total_invested) - (unsafe{MONTHLY_CONTRIBUTION * 12.0});
    let formula_month: f64  = (total_invested * f64::powf(1.0 + month_return_value,   unsafe{MONTHS_INVESTED})  + unsafe{MONTHLY_CONTRIBUTION}                    * (f64::powf(1.0 + month_return_value, unsafe{MONTHS_INVESTED}) - 1.0) / month_return_value - total_invested) - (unsafe{MONTHLY_CONTRIBUTION * MONTHS_INVESTED});
    let formula_day: f64    = total_invested * f64::powf(1.0 + day_return_value,     unsafe{DAYS_INVESTED})    - total_invested;
    let formula_hour: f64   = total_invested * f64::powf(1.0 + hour_return_value,    unsafe{HOURS_INVESTED})   - total_invested;
    let formula_minute: f64 = total_invested * f64::powf(1.0 + minute_return_value,  unsafe{MINUTES_INVESTED}) - total_invested;
    let formula_secs: f64   = total_invested * f64::powf(1.0 + secs_return_value,    unsafe{SECS_INVESTED})    - total_invested;

    (
        formula_year,
        formula_month, 
        formula_day,
        formula_hour,
        formula_minute,
        formula_secs,
    )
}



#[allow(static_mut_refs)]
pub fn realtime_currency_maths()
{ 
    std::thread::spawn
    (||{
        unsafe 
        {
            let mut total_invested_vector = Vec::new();
            for investment in &ALL_INVESTMENTS { total_invested_vector.push(investment.2); }
            REALTIME_TOTAL_INVESTED = total_invested_vector.iter().sum();

            let mut return_value_per_second_vector = Vec::new();
            let mut realtime_currency_vector = Vec::new();
            for (investment_index, investment) in ALL_INVESTMENTS.iter().enumerate() 
            {
                let investment_year_rate      = investment.1 / 100.0;
                let investment_total_invested = investment.2;

                let month_rate       = f64::powf(1.0 + investment_year_rate, 1.00 / 12.00) - 1.0;
                let day_return_value = f64::powf(1.0 + month_rate,           1.00 / 30.00) - 1.0;

                let days_elapsed: f64 = Local::now().signed_duration_since(investment.0).num_days() as f64;
                let return_per_day: f64 =  MUTABLE_ALL_INVESTMENTS[investment_index].2 * f64::powf(1.0 + day_return_value, 1.0) - MUTABLE_ALL_INVESTMENTS[investment_index].2;
                MUTABLE_ALL_INVESTMENTS[investment_index].2 = investment_total_invested * f64::powf(1.0 + day_return_value, days_elapsed);
                
                //REALTIME CURRENCY
                realtime_currency_vector.push(MUTABLE_ALL_INVESTMENTS[investment_index].2);
                //RETURN PER SECOND
                return_value_per_second_vector.push(return_per_day);
            }

            REALTIME_CURRENCY = realtime_currency_vector.iter().sum();
            REALTIME_RETURN_PER_SECOND = return_value_per_second_vector.iter().sum();
        };
    });
}

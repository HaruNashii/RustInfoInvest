//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use crate::investment_wallet::{ALL_INVESTMENTS, REALTIME_CURRENCY, REALTIME_RETURN_PER_SECOND, REALTIME_TOTAL_INVESTED};





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------BACK-END--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
//CALCULATOR PAGE
pub static mut RETURN_VALUE: f64 = 10.0;
pub static mut TOTAL_INVESTED: f64 = 150000.0;
pub static mut ONLINE_HISTORIC_RETURN_VALUE: Vec<String> = Vec::new();





pub fn calculator_maths() -> (f64, f64, f64, f64, f64 ,f64)
{
    let years_invested = 1.0;
    let months_invested = 1.0;
    let days_invested = 1.0;
    let hours_invested = 1.0;
    let minutes_invested = 1.0;
    let secs_invested = 1.0;


        let total_invested = unsafe{TOTAL_INVESTED};

        // Move numbers one case to the right to fit the formula math (example = 1.0 -> 0.1)
        let year_return_value: f64 = unsafe{RETURN_VALUE / 100.0};

        let month_return_value  = f64::powf(1.0 + year_return_value,   1.00 / 12.00) - 1.0;
        let day_return_value    = f64::powf(1.0 + month_return_value,  1.00 / 30.00) - 1.0;
        let hour_return_value   = f64::powf(1.0 + day_return_value,    1.00 / 24.00) - 1.0;
        let minute_return_value = f64::powf(1.0 + hour_return_value,   1.00 / 60.00) - 1.0;
        let secs_return_value   = f64::powf(1.0 + minute_return_value, 1.00 / 60.00) - 1.0;
            
            // Formulas
            // formula = total_invested * (1 + return_value)^total_time_invested
            let formula_year: f64   = total_invested * f64::powf(1.0 + year_return_value,  years_invested) - total_invested;
            let formula_month: f64  = total_invested * f64::powf(1.0 + month_return_value, months_invested) - total_invested;
            let formula_day: f64    = total_invested * f64::powf(1.0 + day_return_value,   days_invested) - total_invested;
            let formula_hour: f64   = total_invested * f64::powf(1.0 + hour_return_value,  hours_invested) - total_invested;
            let formula_minute: f64 = total_invested * f64::powf(1.0 + minute_return_value,  minutes_invested) - total_invested;
            let formula_secs: f64   = total_invested * f64::powf(1.0 + secs_return_value,  secs_invested) - total_invested;


                // Return Values
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
    std::thread::spawn(||{
        unsafe 
        {
            let mut realtime_currency_vector = Vec::new();
            let mut return_value_per_second_vector = Vec::new();
            let mut total_invested_vector = Vec::new();



            for investment in ALL_INVESTMENTS.clone()
            {
                total_invested_vector.push(investment.2);

                let milisecs_since_checked_current_time = investment.0.elapsed().unwrap().as_millis();

                let year_return_value: f64 = investment.1 / 100.00;
                let secs_return_value = f64::powf(1.0 + year_return_value, 1.00 / (360.0 * 24.00 * 60.00 * 60.00)) - 1.0;
                let milisecs_return_value = f64::powf(1.0 + secs_return_value, 1.0 / 1000.0) - 1.0;

                //outdated
                let realtime_milisecs = investment.2 * f64::powf(1.0 + milisecs_return_value,  1.0) - investment.2;

                let holder_1 = investment.2 + (realtime_milisecs * milisecs_since_checked_current_time as f64);
                let new_realtime_secs =     holder_1 * f64::powf(1.0 + secs_return_value,  1.0)     - holder_1;
                let new_realtime_milisecs = holder_1 * f64::powf(1.0 + milisecs_return_value,  1.0) - holder_1;

                realtime_currency_vector.push( holder_1 + (new_realtime_milisecs * milisecs_since_checked_current_time as f64));
                return_value_per_second_vector.push(new_realtime_secs);
            }



            REALTIME_CURRENCY = realtime_currency_vector.iter().sum(); 
            REALTIME_TOTAL_INVESTED = total_invested_vector.iter().sum();
            REALTIME_RETURN_PER_SECOND = return_value_per_second_vector.iter().sum();
        };
    });
}

//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::time::SystemTime;
use std::sync::Once;



//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------BACK-END--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
pub static mut RETURN_VALUE: f64 = 10.0;
pub static mut TOTAL_INVESTED: f64 = 150000.0;
pub static mut ONLINE_HISTORIC_RETURN_VALUE: Vec<String> = Vec::new();

pub static mut CURRENT_TIME: Option<SystemTime> = None; 
pub static mut RETURN_VALUE_REALTIME_PAGE: f64 = 12.15;
pub static mut TOTAL_INVESTED_REALTIME_PAGE: f64 = 5000.0;
pub static mut REALTIME_SECS: f64 = 0.0;
pub static mut REALTIME_MILISECS: f64 = 0.0;
pub static mut REALTIME_CURRENCY: f64 = 0.0;
static START: Once = Once::new();
//static CALL_ONCE: Once = Once::new();
//static CALL_ONCE2: Once = Once::new();



pub fn basic_data() -> (f64, f64, f64, f64, f64, f64)
{
        //let cdi_percentage = 100.0;
        //let use_cdi_value = true;
        let years_invested = 1.0;
        let months_invested = 1.0;
        let days_invested = 1.0;
        let hours_invested = 1.0;
        let minutes_invested = 1.0;
        let secs_invested = 1.0;
  

                // Calcule the return value with CDI 
                // Example of the formula used : "CDI 110% = 10%xreturn_value - 0.10"
                //if use_cdi_value
                //{
                //    CALL_ONCE2.call_once(|| 
                //    {
                //        let percent = Percentage::from_decimal(cdi_percentage / 1000.0);
                //        unsafe{RETURN_VALUE = (percent.apply_to(RETURN_VALUE) - 0.01) * 10.0};
                //    });
                //}

                    // Return values
                    (
                        years_invested, 
                        months_invested,
                        days_invested,
                        hours_invested,
                        minutes_invested,
                        secs_invested,
                    ) 
}





pub fn calculator_maths() -> (f64, f64, f64, f64, f64 ,f64)
{
    // Pull info string from another function
    let (years_invested, months_invested, days_invested, hours_invested, minutes_invested, secs_invested) = basic_data();


        let total_invested = unsafe{TOTAL_INVESTED};

        // Move numbers one case to the right to fit the formula math (example = 1.0 -> 0.1)
        let mut year_return_value: f64 = unsafe{RETURN_VALUE};
        year_return_value /= 100.0;

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
    unsafe 
    {
        // Move numbers one case to the right to fit the formula math (example = 1.0 -> 0.1)
        let mut year_return_value: f64 = RETURN_VALUE_REALTIME_PAGE;
        year_return_value /= 100.0;

        let month_return_value  = f64::powf(1.0 + year_return_value,   1.00 / 12.00) - 1.0;
        let day_return_value    = f64::powf(1.0 + month_return_value,  1.00 / 30.00) - 1.0;
        let hour_return_value   = f64::powf(1.0 + day_return_value,    1.00 / 24.00) - 1.0;
        let minute_return_value = f64::powf(1.0 + hour_return_value,   1.00 / 60.00) - 1.0;
        let secs_return_value   = f64::powf(1.0 + minute_return_value, 1.00 / 60.00) - 1.0;
        let milisecs_return_value   = f64::powf(1.0 + secs_return_value, 1.00 / 1000.00) - 1.0;

        // Formulas
        // formula = total_invested * (1 + return_value)^total_time_invested
        REALTIME_SECS = REALTIME_CURRENCY * f64::powf(1.0 + secs_return_value,  1.0) - REALTIME_CURRENCY;
        REALTIME_MILISECS = REALTIME_CURRENCY * f64::powf(1.0 + milisecs_return_value,  1.0) - REALTIME_CURRENCY;


        START.call_once
        (|| {
            CURRENT_TIME = Some(SystemTime::now())
        });

        let milisecs_since_checked_current_time = CURRENT_TIME.unwrap().elapsed().unwrap().as_millis();
        REALTIME_CURRENCY = TOTAL_INVESTED_REALTIME_PAGE + (REALTIME_MILISECS * milisecs_since_checked_current_time as f64);
    };
}

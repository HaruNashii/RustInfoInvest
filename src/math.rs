//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::sync::Once;
use percentage::Percentage;
use crate::getonlineinfo::infos;



//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------BACK-END--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
static mut ONLINE_RETURN_VALUE: f64 = 0.0;
pub static mut ONLINE_HISTORIC_RETURN_VALUE: Vec<String> = Vec::new();
static CALL_ONCE: Once = Once::new();



pub fn basic_data() -> (f64, f64, bool, bool, f64, f64, f64)
{
        let total_invested = 250000.0;
        let mut year_return_value = 11.25;
        let cdi_percentage = 100.0;
        let use_online_selic_return_value = true;
        let use_cdi_value = false;
        let years_invested = 1.0;
        let months_invested = 1.0;
        let days_invested = 1.0;
  


        // Get infos online with the "infos()" function
        if use_online_selic_return_value
        {
            CALL_ONCE.call_once(|| 
            {
                let (online_historic_return_value, online_return_value) = infos();
                unsafe
                {
                    ONLINE_RETURN_VALUE = online_return_value;
                    ONLINE_HISTORIC_RETURN_VALUE = online_historic_return_value;
                };
            });
            unsafe{year_return_value = ONLINE_RETURN_VALUE};
        }



                // Calcule the return value with CDI 
                // Example of the formula used : "CDI 110% = 10%xreturn_value - 0.10"
                if use_cdi_value
                {
                   let percent = Percentage::from_decimal(cdi_percentage / 1000.0);
                   year_return_value = (percent.apply_to(year_return_value) - 0.01) * 10.0;
                }

                    // Return values
                    (
                        total_invested,
                        year_return_value,
                        use_online_selic_return_value,
                        use_cdi_value,
                        years_invested, 
                        months_invested,
                        days_invested,
                    )
}





pub fn maths() -> (f64, f64, f64, f64, f64 ,f64)
{
    // Pull info string from another function
    let (total_invested, mut year_return_value, _, _, years_invested, months_invested, days_invested) = basic_data();


        // Move numbers one case to the right to fit the formula math (example = 1.0 -> 0.1)
        year_return_value /= 100.0;

        let month_return_value = f64::powf(1.0 + year_return_value, 1.00 / 12.00) - 1.0;
        let day_return_value = f64::powf(1.0 + month_return_value, 1.00 / 30.00) - 1.0;


            // Formulas
            // formula = total_invested * (1 + return_value)^total_time_invested
            let formula: f64       = total_invested * f64::powf(1.0 + year_return_value,   years_invested) - total_invested;
            let formula_month: f64 = total_invested * f64::powf(1.0 + month_return_value, months_invested) - total_invested;
            let formula_day: f64   = total_invested * f64::powf(1.0 + day_return_value,     days_invested) - total_invested;


                // Define which node will use their respective formulas
                let one_year: f64 = formula;
                let one_month: f64 = formula_month;
                let one_day: f64 =  formula_day;
                let one_hour: f64 = one_day / 24.0;
                let one_min: f64 = one_hour / 60.0;
                let one_secs: f64 = one_min / 60.0;


                        // Return Values
                        (
                            one_year,
                            one_month, 
                            one_day,
                            one_hour,
                            one_min,
                            one_secs
                        )
}





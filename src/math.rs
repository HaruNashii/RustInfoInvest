//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::fs::File;
use std::io::Read;
use ron::de::from_str;
use serde_derive::Deserialize;
use percentage::Percentage;
use std::sync::Once;
use std::sync::Mutex;
use crate::getinfo::infos;

static MY_STATIC_VALUE: Mutex<f64> = Mutex::new(72.7);


//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------BACK-END--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
#[derive(Deserialize)]
struct RonConfigTypes 
{
    // data
    total_invested: f64,
    return_value: f64,
    cdi_value: f64,
    // modes
    use_online_selic_return_value: bool,
    use_cdi_value: bool,
    // time
    years_invested: u32,
    months_invested: u32,
}
static CALL_ONCE: Once = Once::new();






fn round_float(value: f64, precision: usize) -> f64
{
    let factor = 10.0_f64.powi(precision as i32);
    (value * factor).round() / factor
}






pub fn read_ron_file() -> (f64, f64, f64, f64, bool, bool, u32, u32)
{
 // Read the Ron file and parse the infos to the variable "content"
 let mut file = File::open("config/data.ron").unwrap();
 let mut content = String::new();
 file.read_to_string(&mut content).unwrap();
 let my_struct: RonConfigTypes = from_str(&content).expect("Failed to parse RON");

 // Round the float to remove unecessary decimal numbers
 let total_invested = round_float(my_struct.total_invested, 2);
 let mut return_value = round_float(my_struct.return_value, 2);

 // CDI && Online value
 let mut cdi_value_translated: f64 = 0.0;
 let percent = Percentage::from_decimal(my_struct.cdi_value / 1000.0);
 if !my_struct.use_online_selic_return_value
 {
    cdi_value_translated = (percent.apply_to(my_struct.return_value) - 0.01) * 10.0;
 }
 if my_struct.use_online_selic_return_value
 {

            CALL_ONCE.call_once(|| {
                    let mut value = MY_STATIC_VALUE.lock().unwrap();
                    let (_, online_return_value) = infos();
                    *value = online_return_value;
            });
            let value = MY_STATIC_VALUE.lock().unwrap();
            cdi_value_translated = (percent.apply_to(*value) - 0.01) * 10.0;
            return_value = *value;
 }


 return (total_invested, return_value, my_struct.cdi_value, cdi_value_translated, my_struct.use_online_selic_return_value, my_struct.use_cdi_value, my_struct.years_invested, my_struct.months_invested);
}





pub fn maths() -> (String, String, String, String, String, String)
{
    let (ron_file_total_invested, mut ron_file_return_value, _, ron_file_cdi_value_translated, _, use_cdi_value, ron_file_years_invested, ron_file_months_invested) = read_ron_file();

    let month_return_value: f64 = (ron_file_return_value / 12.0) / 100.0;
    ron_file_return_value = ron_file_return_value / 100.0;


    if use_cdi_value
    {
        ron_file_return_value = ron_file_cdi_value_translated / 100.0;
    }


    //formula = total_invested * (1 + return_value)^total_time_invested
    let formula: f64 = ron_file_total_invested * f64::powf(1.0 + ron_file_return_value, ron_file_years_invested as f64) - ron_file_total_invested;
    let formula_month: f64 = ron_file_total_invested * f64::powf(1.0 + month_return_value, ron_file_months_invested as f64) - ron_file_total_invested;


    let one_year: f64 = formula;
    let one_month: f64 = formula_month;
    let one_day: f64 =  formula_month / 31.0;
    let one_hour: f64 = one_day / 24.0;
    let one_min: f64 = one_hour / 60.0;
    let one_secs: f64 = one_min / 60.0;


    let one_year = round_float(one_year, 2);
    let one_month = round_float(one_month, 2);
    let one_day = round_float(one_day, 2);
    let one_hour = round_float(one_hour, 2);
    let one_min = round_float(one_min, 3);
    let one_secs = round_float(one_secs, 4);
    
    return (one_year.to_string(), one_month.to_string(), one_day.to_string(), one_hour.to_string(), one_min.to_string(), one_secs.to_string());
}




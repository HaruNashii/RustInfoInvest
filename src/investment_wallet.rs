use std::time::SystemTime;

pub static mut ALL_INVESTMENT: Vec<(SystemTime, f64, f64, String)> = Vec::new();
pub static mut YEAR_RETURN_VALUE_PER_INVESTMENT: f64 = 12.15;
pub static mut TOTAL_INVESTED_PER_INVESTMENT: f64 = 5000.0;
pub static mut INVESTMENT_NAME: String = String::new();

#[allow(static_mut_refs)]
pub fn add_investment()
{
   unsafe
   {
        if INVESTMENT_NAME.replace(" ", "").is_empty()
        {
            INVESTMENT_NAME = "Generic Investment".to_string();
        }

       ALL_INVESTMENT.push
       (
           (
               SystemTime::now(),
               YEAR_RETURN_VALUE_PER_INVESTMENT, 
               TOTAL_INVESTED_PER_INVESTMENT,
               INVESTMENT_NAME.clone()
            )
       );

       println!("{:?}", ALL_INVESTMENT);
       YEAR_RETURN_VALUE_PER_INVESTMENT = 12.15;
       TOTAL_INVESTED_PER_INVESTMENT = 5000.0;
       INVESTMENT_NAME = String::new();
   };
}

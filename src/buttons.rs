use crate::
{ 
    getonlineinfo::infos, input_handler::{BUTTON_CLICKED, IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_4, IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_5, IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_6, IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_7, IS_ON_WRITE_MODE_ON_BUTTON_8, IS_ON_WRITE_MODE_ON_BUTTON_9}, investment_wallet::{add_investment, ALL_INVESTMENTS, MUTABLE_ALL_INVESTMENTS}, math::ONLINE_HISTORIC_RETURN_VALUE
};





pub static mut PAGE_TO_RENDER: u8 = 0;
pub static mut ALLOW_QUERY: bool = true;





#[allow(static_mut_refs)]
pub fn button_action()
{
    unsafe 
    {
        match BUTTON_CLICKED
        {
            Some(1) =>
            {
                //CALCULATOR BUTTON (PERSISTENT PAGE)
                if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3 { PAGE_TO_RENDER = 0; };
                BUTTON_CLICKED = None;
            }

            Some(2) =>
            {
                //REALTIME CURRENCY BUTTON (PERSISTENT PAGE)
                if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3 { BUTTON_CLICKED = None; PAGE_TO_RENDER = 1; };
                BUTTON_CLICKED = None;
            }

            Some(3) =>
            {
                //SELIC PAGE BUTTON
                if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3 { BUTTON_CLICKED = None; PAGE_TO_RENDER = 2; };
                BUTTON_CLICKED = None;
            }

            Some(4) => 
            {
                //USER INPUT 1 (CALCULATOR PAGE) (YEAR RETURN VALUE)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_1 = true; }
                BUTTON_CLICKED = None;
            }

            Some(5) =>
            {
                //USER INPUT 2 (CALCULATOR PAGE) (TOTAL INVESTED)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_2 = true; }
                BUTTON_CLICKED = None;
            }

            Some(6) =>
            {
                //USER INPUT 3 (CALCULATOR PAGE) (YEAR)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(7) => 
            {
                //USER INPUT 4 (CALCULATOR PAGE) (MONTHS)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_4 = true; };
                BUTTON_CLICKED = None;
            }

            Some(8) => 
            {
                //USER INPUT 5 (CALCULATOR PAGE) (DAYS)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_5 = true; };
                BUTTON_CLICKED = None;
            }

            Some(9) => 
            {
                //USER INPUT 6 (CALCULATOR PAGE) (HOURS)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_6 = true; };
                BUTTON_CLICKED = None;
            }

            Some(10) => 
            {
                //USER INPUT 7 (CALCULATOR PAGE) (MINUTES)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_7 = true; };
                BUTTON_CLICKED = None;
            }

            Some(11) => 
            {
                //USER INPUT 8 (CALCULATOR PAGE) (SECONDS)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 && !IS_ON_WRITE_MODE_ON_BUTTON_9 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_8 = true; };
                BUTTON_CLICKED = None;
            }

            Some(12) =>
            {
                //USER INPUT 9 (CALCULATOR PAGE) (MONTLY CONTRIBUTION)
                if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2&& !IS_ON_WRITE_MODE_ON_BUTTON_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4 && !IS_ON_WRITE_MODE_ON_BUTTON_5 && !IS_ON_WRITE_MODE_ON_BUTTON_6 && !IS_ON_WRITE_MODE_ON_BUTTON_7 && !IS_ON_WRITE_MODE_ON_BUTTON_8 && !IS_ON_WRITE_MODE_ON_BUTTON_9 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_9 = true; };
                BUTTON_CLICKED = None;
            }

            Some(13) =>
            {
                //INVESTMENT WALLET BUTTON (REALTIME CURRENCY PAGE)
                if PAGE_TO_RENDER == 1 { BUTTON_CLICKED = None; PAGE_TO_RENDER = 3; }
                BUTTON_CLICKED = None;
            }

            Some(14) =>
            {
                //SYNC ONLINE BUTTON (SELIC PAGE)
                if PAGE_TO_RENDER == 2 { BUTTON_CLICKED = None; std::thread::spawn(move|| { if ALLOW_QUERY { ONLINE_HISTORIC_RETURN_VALUE = vec!["        Fetching Data, Please Wait...".to_string()]; ALLOW_QUERY = false; let historic = infos(); ALLOW_QUERY = true; ONLINE_HISTORIC_RETURN_VALUE = historic; } }); }
                BUTTON_CLICKED = None;
            }

            Some(15) =>
            {
                //BACK BUTTON (INVESTMENT WALLET PAGE)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; PAGE_TO_RENDER = 1; };
                BUTTON_CLICKED = None;
            }

            Some(16) =>
            {
                //USER INPUT 1 BUTTON (INVESTMENT WALLET PAGE) (YEAR RETURN VALUE)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(17) =>
            {
                //USER INPUT 2 BUTTON (INVESTMENT WALLET PAGE) (TOTAL INVESTED)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(18) =>
            {
                //USER INPUT 3 (INVESTMENT WALLET) (INVESTMENT NAME)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = true; }
                BUTTON_CLICKED = None;
            }

            Some(19) =>
            {
                //ADD INVESTMENT BUTTON (INVESTMENT WALLET)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; add_investment(); }
                BUTTON_CLICKED = None;
            }

            Some(20) =>
            {
                //REMOVE ALL INVESTMENT BUTTON (INVESTMENT WALLET)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { BUTTON_CLICKED = None; ALL_INVESTMENTS.clear(); MUTABLE_ALL_INVESTMENTS.clear(); }
                BUTTON_CLICKED = None;
            }

            Some(21) =>
            {
                //USER INPUT 4 (INVESTMENT WALLET) (YEARS)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(22) =>
            {
                //USER INPUT 5 (INVESTMENT WALLET) (MONTHS)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(23) =>
            {
                //USER INPUT 6 (INVESTMENT WALLET) (DAYS)
                if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { BUTTON_CLICKED = None; IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 = true; };
                BUTTON_CLICKED = None;
            }

            Some(x) =>
            {
                if x >= 1000
                {
                    //(INVESTMENT WALLET) INVESTMENT REMOVE BUTTON
                    ALL_INVESTMENTS.remove(x - 1000);
                    BUTTON_CLICKED = None;
                }
            }

            _=> {},
        }
    }
}

use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, BUTTON_CLICKED};
use crate::math::ONLINE_HISTORIC_RETURN_VALUE;
use crate::getonlineinfo::infos;
use crate::investment_wallet::add_investment;



pub static mut PAGE_TO_RENDER: u8 = 0;
pub static mut ALLOW_QUERY: bool = true;



#[allow(static_mut_refs)]
pub fn button_action()
{
    if let Some(result) = unsafe{BUTTON_CLICKED}
    {
        match result 
        {
            0 =>
            {
                unsafe 
                {
                    //CALCULATOR BUTTON (PERSISTENT PAGE)
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3
                    {
                        BUTTON_CLICKED = None;
                        PAGE_TO_RENDER = 0;
                    };

                    //BACK BUTTON (INVESTMENT WALLET PAGE)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        BUTTON_CLICKED = None;
                        PAGE_TO_RENDER = 1;
                    };
                }
            }

            1 =>
            {
                unsafe 
                {
                    //USER INPUT 1 BUTTON (INVESTMENT WALLET PAGE) (YEAR RETURN VALUE)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        BUTTON_CLICKED = None;
                        IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = true;
                    };

                    //REALTIME CURRENCY BUTTON (PERSISTENT PAGE)
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3
                    {
                        BUTTON_CLICKED = None;
                        PAGE_TO_RENDER = 1;
                    };
                }
            }

            2 =>
            {
                unsafe 
                {
                    //USER INPUT 2 BUTTON (INVESTMENT WALLET PAGE) (TOTAL INVESTED)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        BUTTON_CLICKED = None;
                        IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = true;
                    };

                    //SELIC PAGE BUTTON
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3
                    {
                        BUTTON_CLICKED = None;
                        PAGE_TO_RENDER = 2;
                    };
                }
            }

            3 => 
            {
                unsafe 
                {
                    //USER INPUT 3 (INVESTMENT WALLET) (INVESTMENT NAME)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        BUTTON_CLICKED = None;
                        IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = true;
                    }

                    //USER INPUT 1 (CALCULATOR PAGE) (YEAR RETURN VALUE)
                    if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 {
                        BUTTON_CLICKED = None;
                        IS_ON_WRITE_MODE_ON_BUTTON_1 = true;
                    }
                    
                    //INVESTMENT WALLET BUTTON (REALTIME CURRENCY PAGE)
                    if PAGE_TO_RENDER == 1 
                    {
                        BUTTON_CLICKED = None;
                        PAGE_TO_RENDER = 3;
                    }

                    //SYNC ONLINE BUTTON (SELIC PAGE)
                    if PAGE_TO_RENDER == 2
                    {
                        BUTTON_CLICKED = None;
                        std::thread::spawn(move||
                        {
                            if ALLOW_QUERY
                            {
                                println!("fetching...");
                                ONLINE_HISTORIC_RETURN_VALUE = vec!["        Fetching Data, Please Wait...".to_string()];
                                ALLOW_QUERY = false;
                                let historic = infos();
                                ALLOW_QUERY = true;
                                ONLINE_HISTORIC_RETURN_VALUE = historic;
                            }
                        });
                    }
                };
            }

            4 =>
            {
                unsafe
                {
                    //ADD INVESTMENT BUTTON (INVESTMENT WALLET)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        BUTTON_CLICKED = None;
                        add_investment();
                    }

                    //USER INPUT 2 (CALCULATOR PAGE) (TOTAL INVESTED)
                    if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 
                    {
                        BUTTON_CLICKED = None;
                        IS_ON_WRITE_MODE_ON_BUTTON_2 = true;
                    }
                };
            }

            _ => {}
        }
    }
}

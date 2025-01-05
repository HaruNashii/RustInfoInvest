use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, BUTTON_CLICKED};
use crate::math::ONLINE_HISTORIC_RETURN_VALUE;
use crate::getonlineinfo::infos;
use crate::investment_wallet::add_investment;



pub static mut PAGE_TO_RENDER: u8 = 0;



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
                        PAGE_TO_RENDER = 0;
                        BUTTON_CLICKED = None;
                    };

                    //BACK BUTTON (INVESTMENT WALLET PAGE)
                    if PAGE_TO_RENDER == 3 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
                    {
                        PAGE_TO_RENDER = 1;
                        BUTTON_CLICKED = None;
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
                        IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = true;
                        BUTTON_CLICKED = None;
                    };

                    //REALTIME CURRENCY BUTTON (PERSISTENT PAGE)
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3
                    {
                        PAGE_TO_RENDER = 1;
                        BUTTON_CLICKED = None;
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
                        IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = true;
                        BUTTON_CLICKED = None;
                    };

                    //SELIC PAGE BUTTON
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2 && PAGE_TO_RENDER != 3
                    {
                        PAGE_TO_RENDER = 2;
                        BUTTON_CLICKED = None;
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
                        IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = true;
                        BUTTON_CLICKED = None;
                    }

                    //USER INPUT 1 (CALCULATOR PAGE) (YEAR RETURN VALUE)
                    if PAGE_TO_RENDER == 0
                    {
                        if !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2
                        {
                            IS_ON_WRITE_MODE_ON_BUTTON_1 = true;
                            BUTTON_CLICKED = None;
                        }
                    }
                    
                    //INVESTMENT WALLET BUTTON (REALTIME CURRENCY PAGE)
                    if PAGE_TO_RENDER == 1 
                    {
                        PAGE_TO_RENDER = 3;
                        BUTTON_CLICKED = None;
                    }

                    //SYNC ONLINE BUTTON (SELIC PAGE)
                    if PAGE_TO_RENDER == 2
                    {
                        ONLINE_HISTORIC_RETURN_VALUE = infos();
                        BUTTON_CLICKED = None;
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
                        println!("add investment button in investment wallet clicked");
                        add_investment();
                        BUTTON_CLICKED = None;
                    }

                    //USER INPUT 2 (CALCULATOR PAGE) (TOTAL INVESTED)
                    if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 
                    {
                        IS_ON_WRITE_MODE_ON_BUTTON_2 = true;
                        BUTTON_CLICKED = None;
                    }
                };
            }

            _ => {}
        }
    }
}

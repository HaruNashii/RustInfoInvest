use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2, BUTTON_CLICKED};
use crate::math::{RETURN_VALUE, ONLINE_HISTORIC_RETURN_VALUE};
use crate::getonlineinfo::infos;



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
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2
                    {
                        PAGE_TO_RENDER = 0;
                    };
                    BUTTON_CLICKED = None;
                }
            }

            1 =>
            {
                unsafe 
                {
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2
                    {
                        PAGE_TO_RENDER = 1;
                    };
                    BUTTON_CLICKED = None;
                }
            }

            2 =>
            {
                unsafe 
                {
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1  && !IS_ON_WRITE_MODE_ON_BUTTON_2
                    {
                        PAGE_TO_RENDER = 2;
                    };
                    BUTTON_CLICKED = None;
                }
            }

            3 => 
            {
                unsafe 
                {
                    if PAGE_TO_RENDER == 0
                    {
                        //user input text button 1
                        if !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2
                        {
                            IS_ON_WRITE_MODE_ON_BUTTON_1 = true;
                        }
                    }

                    if PAGE_TO_RENDER == 1 
                    {
                        //user input text button 1
                        if !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
                        {
                            IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 = true;
                        }

                    }

                    if PAGE_TO_RENDER == 2
                    {
                        let (online_historic_return_value, online_return_value) = infos();
                        RETURN_VALUE = online_return_value;
                        ONLINE_HISTORIC_RETURN_VALUE = online_historic_return_value;
                    }
                    BUTTON_CLICKED = None;
                };
            }

            4 =>
            {
                //user input text button 2
                unsafe
                {
                    if PAGE_TO_RENDER == 0 && !IS_ON_WRITE_MODE_ON_BUTTON_1 && !IS_ON_WRITE_MODE_ON_BUTTON_2 {
                        IS_ON_WRITE_MODE_ON_BUTTON_2 = true;
                    }

                    if PAGE_TO_RENDER == 1 && !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 && !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2 {
                        IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2 = true;
                    }
                    BUTTON_CLICKED = None;
                };
            }

            _ => {}
        }
    }
}

use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_2, BUTTON_CLICKED};


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
                println!("button 0 pressed");
                unsafe 
                {
                    PAGE_TO_RENDER = 0;
                    BUTTON_CLICKED = None;
                }
            }

            1 =>
            {
                println!("button 1 pressed");
                unsafe 
                {
                    PAGE_TO_RENDER = 1;
                    BUTTON_CLICKED = None;
                }
            }

            2 =>
            {
                println!("button 2 pressed");
                unsafe 
                {
                    PAGE_TO_RENDER = 2;
                    BUTTON_CLICKED = None;
                }
            }

            3 => 
            {
                //user input text button 1
                unsafe 
                {
                    println!("button 2 pressed");
                    if !IS_ON_WRITE_MODE_ON_BUTTON_1
                    {
                        IS_ON_WRITE_MODE_ON_BUTTON_1 = true;
                        BUTTON_CLICKED = None;
                    }
                };
            }

            4 =>
            {
                //user input text button 2
                unsafe
                {
                    println!("button 3 pressed");
                    if !IS_ON_WRITE_MODE_ON_BUTTON_2
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

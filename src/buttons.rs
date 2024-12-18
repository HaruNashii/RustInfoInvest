use crate::input_handler::{IS_ON_WRITE_MODE, BUTTON_CLICKED};


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
                //user input text
                unsafe 
                {
                    println!("button 2 pressed");
                    if !IS_ON_WRITE_MODE
                    {
                        IS_ON_WRITE_MODE = true;
                        BUTTON_CLICKED = None;
                    }
                };
            }

            _ => {}
        }
    }
}

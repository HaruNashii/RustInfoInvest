use std::process::exit;

use sdl3::
{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect, EventPump
};

use crate::
{ 
    buttons::PAGE_TO_RENDER,
    getonlineinfo::PREVENT_KILL, 
    investment_wallet::{DAY, INVESTMENT_NAME, MONTH, RETURN_PER_INVESTMENT, TOTAL_INVESTED_PER_INVESTMENT, YEAR}, 
    math::{DAYS_INVESTED, HOURS_INVESTED, MINUTES_INVESTED, MONTHS_INVESTED, MONTHLY_CONTRIBUTION, RETURN_VALUE, SECS_INVESTED, TOTAL_INVESTED, YEARS_INVESTED}, 
};




pub static mut USER_INPUT: String = String::new();
pub static mut IS_ON_WRITE_MODE: (bool, Option<u16>) = (false, None);
pub static mut BUTTON_BEING_HOVERED: Option<usize> = None;
pub static mut BUTTON_CLICKED: Option<usize> = None;


#[allow(static_mut_refs)]
pub fn handle_input(buttons: &Vec<(bool, Color, Rect, u16)>, event_pump: &mut EventPump, window_scale: (u32, u32))
{   
    for event in event_pump.poll_iter() 
    {
        match event 
        {
            //================================================================================================#
            //==========================================# MOUSE #=============================================#
            //================================================================================================#
            Event::MouseMotion { x, y, .. } => 
            {

                unsafe 
                {
                    if !IS_ON_WRITE_MODE.0 
                    { 
                        BUTTON_BEING_HOVERED = None; 

                        let x_scaled = x * (1920.00 / window_scale.0 as f32);
                        let y_scaled = y * (1080.00 / window_scale.1 as f32);
                        
                        for object in buttons
                        {
                            if x_scaled >= object.2.x as f32 && x_scaled <= (object.2.x + object.2.w) as f32 && y_scaled >= object.2.y as f32 && y_scaled <= (object.2.y + object.2.h) as f32
                            {
                                BUTTON_BEING_HOVERED = Some(object.3 as usize); 
                            }
                        };
                    };
                };
            }



            Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
            {
                unsafe 
                {
                    if !IS_ON_WRITE_MODE.0
                    {
                        if let Some(result) = BUTTON_BEING_HOVERED {BUTTON_CLICKED = Some(result)};
                    }
                }
            }



            //================================================================================================#
            //=======================================# KEYBOARD #=============================================#
            //================================================================================================#
            Event::TextInput{text, .. } =>
            {
                unsafe 
                {
                    if text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9" || text == "."
                    {
                        match IS_ON_WRITE_MODE.1
                        {
                            Some(4) =>  { if USER_INPUT.replace(" ", "").len() < 6  { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue;}; USER_INPUT = USER_INPUT.replace(" ", ""); USER_INPUT.push_str(&text); } },
                            Some(5) =>  { if USER_INPUT.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue;}; USER_INPUT = USER_INPUT.replace(" ", ""); USER_INPUT.push_str(&text); } },
                            Some(12) => { if USER_INPUT.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue};  USER_INPUT.push_str(&text); }},
                            Some(16..18) => { if USER_INPUT.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue};  USER_INPUT.push_str(&text); }},
                            _=> {},
                        }
                    }

                    if text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9"
                    {
                        match IS_ON_WRITE_MODE.1
                        {
                            Some(3) =>      { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 3  { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); } },
                            Some(6..12) =>  { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 3  { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); } },
                            Some(21) =>     { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 4  { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); } },
                            Some(22) =>     { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 2  { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); if !USER_INPUT.replace(" ", "").is_empty() && USER_INPUT.replace(" ", "").parse::<i32>().unwrap() > 12 || USER_INPUT.replace(" ", "").len() < 2 && text == "0" { USER_INPUT.pop(); } } },
                            Some(23) =>     { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 2  { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); if !USER_INPUT.replace(" ", "").is_empty() && USER_INPUT.replace(" ", "").parse::<i32>().unwrap() > 31 || USER_INPUT.replace(" ", "").len() < 2 && text == "0" { USER_INPUT.pop(); } } },
                            _=> {},
                        }
                    }
                    
                    if IS_ON_WRITE_MODE.1 == Some(18) && USER_INPUT.len() < 19 { USER_INPUT.push_str(&text); };
                };
            }
            


            Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
            {
                unsafe 
                {
                    if IS_ON_WRITE_MODE.0 && !USER_INPUT.is_empty() 
                    { 
                        USER_INPUT.pop();
                    };
                }
            }



            Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
            {
                 unsafe 
                 {
                     match IS_ON_WRITE_MODE.1
                     {
                        Some(4) =>  { if !USER_INPUT.replace(" ", "").is_empty() { RETURN_VALUE =                  USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(5) =>  { if !USER_INPUT.replace(" ", "").is_empty() { TOTAL_INVESTED =                USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(6) =>  { if !USER_INPUT.replace(" ", "").is_empty() { YEARS_INVESTED =                USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(7) =>  { if !USER_INPUT.replace(" ", "").is_empty() { MONTHS_INVESTED =               USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(8) =>  { if !USER_INPUT.replace(" ", "").is_empty() { DAYS_INVESTED =                 USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(9) =>  { if !USER_INPUT.replace(" ", "").is_empty() { HOURS_INVESTED =                USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(10) => { if !USER_INPUT.replace(" ", "").is_empty() { MINUTES_INVESTED =              USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(11) => { if !USER_INPUT.replace(" ", "").is_empty() { SECS_INVESTED =                 USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(12) => { if !USER_INPUT.replace(" ", "").is_empty() { MONTHLY_CONTRIBUTION =          USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(16) => { if !USER_INPUT.replace(" ", "").is_empty() { RETURN_PER_INVESTMENT =         USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(17) => { if !USER_INPUT.replace(" ", "").is_empty() { TOTAL_INVESTED_PER_INVESTMENT = USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(18) => { if !USER_INPUT.replace(" ", "").is_empty() { INVESTMENT_NAME =               USER_INPUT.to_string();                       } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(21) => { if !USER_INPUT.replace(" ", "").is_empty() { YEAR =                          USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(22) => { if !USER_INPUT.replace(" ", "").is_empty() { MONTH =                         USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(23) => { if !USER_INPUT.replace(" ", "").is_empty() { DAY =                           USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        _=> {},
                     }
                 }
            }
            


            Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
            {
                 unsafe
                 {
                     if IS_ON_WRITE_MODE.0
                     {
                         USER_INPUT.clear();
                         IS_ON_WRITE_MODE = (false, None);
                         BUTTON_CLICKED = None;
                     } 
                     else 
                     {
                         if PAGE_TO_RENDER != 3 && !PREVENT_KILL { exit(0); }
                         if PAGE_TO_RENDER == 3 { PAGE_TO_RENDER = 1; }
                     }
                 }
            }



            Event::Quit { .. } => 
            { 
                print!("\x1B[2J\x1B[1;1H");
                println!("bye bye :3");
                exit(0); 
            },

            _ => {},
        }
    }
}


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




//PAGE 1
pub static mut USER_INPUT: String = String::new();
pub static mut IS_ON_WRITE_MODE: (bool, Option<u16>) = (false, None);


//PAGE 3
pub static mut USER_INPUT_BUTTON_1_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_2_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_3_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_4_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_5_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_6_PAGE_3: String = String::new();
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3: bool = false;

pub static mut BUTTON_BEING_HOVERED: usize = 0;
pub static mut IS_HOVERING_BUTTON: bool = false;
pub static mut BUTTON_CLICKED: Option<usize> = None;





#[allow(static_mut_refs)]
#[allow(clippy::suspicious_else_formatting)]
pub fn handle_input(buttons: &Vec<(bool, Color, Rect, u16)>, event_pump: &mut EventPump)
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
                    IS_HOVERING_BUTTON = false;
                    for object in buttons
                    {
                        if x >= object.2.x as f32 && x <= (object.2.x + object.2.w) as f32 && y >= object.2.y as f32 && y <= (object.2.y + object.2.h) as f32
                        {
                                BUTTON_BEING_HOVERED = object.3 as usize; 
                                IS_HOVERING_BUTTON = true;
                        }
                    };
                };
            }

            Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
            {
                unsafe 
                {
                    if IS_HOVERING_BUTTON 
                    {
                        BUTTON_CLICKED = Some(BUTTON_BEING_HOVERED);
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
                        // PAGE 1
                        match IS_ON_WRITE_MODE.1
                        {
                            Some(4) => { if USER_INPUT.replace(" ", "").len() < 6  { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue;}; USER_INPUT = USER_INPUT.replace(" ", ""); USER_INPUT.push_str(&text); } },
                            Some(5) => { if USER_INPUT.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue;}; USER_INPUT = USER_INPUT.replace(" ", ""); USER_INPUT.push_str(&text); } },
                            Some(9) => { if USER_INPUT.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT.replace(" ", "").is_empty() || USER_INPUT.contains('.')) {continue;}; USER_INPUT = USER_INPUT.replace(" ", ""); USER_INPUT.push_str(&text); } },
                            _=> {},
                        }
                        
                        // PAGE 3
                        if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 && USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").len() < 6  { if text == "." && (USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").is_empty() || USER_INPUT_BUTTON_1_PAGE_3.contains('.')) {continue}; USER_INPUT_BUTTON_1_PAGE_3.push_str(&text); };
                        if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 && USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").len() < 10 { if text == "." && (USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").is_empty() || USER_INPUT_BUTTON_2_PAGE_3.contains('.')) {continue}; USER_INPUT_BUTTON_2_PAGE_3.push_str(&text); };
                    }

                    if text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9"
                    {
                        // PAGE 1
                        match IS_ON_WRITE_MODE.1
                        {
                            Some(3) => { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 3 { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); } },
                            Some(6..8) => { if USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 3 { USER_INPUT = USER_INPUT.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT.push_str(&text); } },
                            _=> {},
                        }

                        // PAGE 3
                        if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 && USER_INPUT_BUTTON_4_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 4 { USER_INPUT_BUTTON_4_PAGE_3 = USER_INPUT_BUTTON_4_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT_BUTTON_4_PAGE_3.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 && USER_INPUT_BUTTON_6_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 2 { USER_INPUT_BUTTON_6_PAGE_3 = USER_INPUT_BUTTON_6_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT_BUTTON_6_PAGE_3.push_str(&text); if !USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").is_empty() && USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").parse::<i32>().unwrap() > 31 || USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").len() < 2 && text == "0" { USER_INPUT_BUTTON_6_PAGE_3.pop(); } }
                        if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 && USER_INPUT_BUTTON_5_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>().len() < 2 { USER_INPUT_BUTTON_5_PAGE_3 = USER_INPUT_BUTTON_5_PAGE_3.chars().filter(|c| !c.is_whitespace()).collect(); USER_INPUT_BUTTON_5_PAGE_3.push_str(&text); if !USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").is_empty() && USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").parse::<i32>().unwrap() > 12 || USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").len() < 2 && text == "0" { USER_INPUT_BUTTON_5_PAGE_3.pop(); } }
                    }
                    
                    //PAGE 3
                    if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 && USER_INPUT_BUTTON_3_PAGE_3.len() < 19 { USER_INPUT_BUTTON_3_PAGE_3.push_str(&text); }
                };
            }
            
            Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
            {
                unsafe 
                {
                    // PAGE 1
                    match IS_ON_WRITE_MODE.1
                    {
                        Some(4..11) =>  { if USER_INPUT.len() == 1 { USER_INPUT.insert(0, ' '); USER_INPUT.pop(); } if !USER_INPUT.is_empty() { USER_INPUT.pop(); } },
                        _=> {},
                    };

                    // PAGE 3
                    if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 { if USER_INPUT_BUTTON_1_PAGE_3.len() == 1 { USER_INPUT_BUTTON_1_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_1_PAGE_3.pop(); } if !USER_INPUT_BUTTON_1_PAGE_3.is_empty() { USER_INPUT_BUTTON_1_PAGE_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 { if USER_INPUT_BUTTON_2_PAGE_3.len() == 1 { USER_INPUT_BUTTON_2_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_2_PAGE_3.pop(); } if !USER_INPUT_BUTTON_2_PAGE_3.is_empty() { USER_INPUT_BUTTON_2_PAGE_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { if USER_INPUT_BUTTON_3_PAGE_3.len() == 1 { USER_INPUT_BUTTON_3_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_3_PAGE_3.pop(); } if !USER_INPUT_BUTTON_3_PAGE_3.is_empty() { USER_INPUT_BUTTON_3_PAGE_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 { if USER_INPUT_BUTTON_4_PAGE_3.len() == 1 { USER_INPUT_BUTTON_4_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_4_PAGE_3.pop(); } if !USER_INPUT_BUTTON_4_PAGE_3.is_empty() { USER_INPUT_BUTTON_4_PAGE_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 { if USER_INPUT_BUTTON_5_PAGE_3.len() == 1 { USER_INPUT_BUTTON_5_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_5_PAGE_3.pop(); } if !USER_INPUT_BUTTON_5_PAGE_3.is_empty() { USER_INPUT_BUTTON_5_PAGE_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { if USER_INPUT_BUTTON_6_PAGE_3.len() == 1 { USER_INPUT_BUTTON_6_PAGE_3.insert(0, ' '); USER_INPUT_BUTTON_6_PAGE_3.pop(); } if !USER_INPUT_BUTTON_6_PAGE_3.is_empty() { USER_INPUT_BUTTON_6_PAGE_3.pop(); } }
                }
            }

            Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
            {
                 unsafe 
                 {
                     // PAGE 1
                     match IS_ON_WRITE_MODE.1
                     {
                        Some(4) =>  { if !USER_INPUT.replace(" ", "").is_empty() { RETURN_VALUE =         USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(5) =>  { if !USER_INPUT.replace(" ", "").is_empty() { TOTAL_INVESTED =       USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(7) =>  { if !USER_INPUT.replace(" ", "").is_empty() { MONTHS_INVESTED =      USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(6) =>  { if !USER_INPUT.replace(" ", "").is_empty() { YEARS_INVESTED =       USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(8) =>  { if !USER_INPUT.replace(" ", "").is_empty() { DAYS_INVESTED =        USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(9) =>  { if !USER_INPUT.replace(" ", "").is_empty() { HOURS_INVESTED =       USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(10) => { if !USER_INPUT.replace(" ", "").is_empty() { MINUTES_INVESTED =     USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(11) => { if !USER_INPUT.replace(" ", "").is_empty() { SECS_INVESTED =        USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        Some(12) => { if !USER_INPUT.replace(" ", "").is_empty() { MONTHLY_CONTRIBUTION = USER_INPUT.replace(" ", "").parse().unwrap(); } USER_INPUT.clear(); IS_ON_WRITE_MODE = (false, None); },
                        _=> {},
                     }

                     // PAGE 3
                     if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 { if !USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").is_empty() { RETURN_PER_INVESTMENT =         USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_1_PAGE_3.clear();   IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 { if !USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").is_empty() { TOTAL_INVESTED_PER_INVESTMENT = USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_2_PAGE_3.clear();   IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { if !USER_INPUT_BUTTON_3_PAGE_3.replace(" ", "").is_empty() { INVESTMENT_NAME =               USER_INPUT_BUTTON_3_PAGE_3.clone(); }                           USER_INPUT_BUTTON_3_PAGE_3.clear();   IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 { if !USER_INPUT_BUTTON_4_PAGE_3.replace(" ", "").is_empty() { YEAR =                          USER_INPUT_BUTTON_4_PAGE_3.parse().unwrap();                    USER_INPUT_BUTTON_4_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 { if !USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").is_empty() { MONTH =                         USER_INPUT_BUTTON_5_PAGE_3.parse().unwrap();                    USER_INPUT_BUTTON_5_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { if !USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").is_empty() { DAY =                           USER_INPUT_BUTTON_6_PAGE_3.parse().unwrap();                    USER_INPUT_BUTTON_6_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 = false; }
                 }
            }
            
            Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
            {
                 unsafe
                 {
                     if IS_ON_WRITE_MODE.0 == true || IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3
                     {
                         //PAGE 1
                         USER_INPUT.clear();
                         IS_ON_WRITE_MODE = (false, None);
                         
                         //PAGE 3
                         USER_INPUT_BUTTON_1_PAGE_3.clear();
                         USER_INPUT_BUTTON_2_PAGE_3.clear();
                         USER_INPUT_BUTTON_3_PAGE_3.clear();
                         USER_INPUT_BUTTON_4_PAGE_3.clear();
                         USER_INPUT_BUTTON_5_PAGE_3.clear();
                         USER_INPUT_BUTTON_6_PAGE_3.clear();
                         USER_INPUT_BUTTON_1_PAGE_3.push(' ');
                         USER_INPUT_BUTTON_2_PAGE_3.push(' ');
                         USER_INPUT_BUTTON_3_PAGE_3.push(' ');
                         USER_INPUT_BUTTON_4_PAGE_3.push(' ');
                         USER_INPUT_BUTTON_5_PAGE_3.push(' ');
                         USER_INPUT_BUTTON_6_PAGE_3.push(' ');
                         IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = false;    
                         IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 = false;

                         BUTTON_CLICKED = None;
                     } 
                     else 
                     {
                         if PAGE_TO_RENDER != 3 && !PREVENT_KILL
                         {
                             print!("\x1B[2J\x1B[1;1H");
                             println!("bye bye :3");
                             exit(0);
                         }

                         if PAGE_TO_RENDER == 3
                         {
                             PAGE_TO_RENDER = 1;
                         }
                         
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


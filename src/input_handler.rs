use std::process::exit;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::
{ 
    getonlineinfo::PREVENT_KILL,
    investment_wallet::{DAY, INVESTMENT_NAME, MONTH, RETURN_PER_INVESTMENT, TOTAL_INVESTED_PER_INVESTMENT, YEAR},
    window::SDL2_EVENT_PUMP,
    buttons::PAGE_TO_RENDER,
    math::{DAYS_INVESTED, HOURS_INVESTED, MINUTES_INVESTED, MONTHS_INVESTED, RETURN_VALUE, SECS_INVESTED, TOTAL_INVESTED, YEARS_INVESTED},
};





pub static mut USER_INPUT_BUTTON_1: String = String::new();
pub static mut USER_INPUT_BUTTON_2: String = String::new();
pub static mut USER_INPUT_BUTTON_3: String = String::new();
pub static mut USER_INPUT_BUTTON_4: String = String::new();
pub static mut USER_INPUT_BUTTON_5: String = String::new();
pub static mut USER_INPUT_BUTTON_6: String = String::new();
pub static mut USER_INPUT_BUTTON_7: String = String::new();
pub static mut USER_INPUT_BUTTON_8: String = String::new();

pub static mut USER_INPUT_BUTTON_1_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_2_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_3_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_4_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_5_PAGE_3: String = String::new();
pub static mut USER_INPUT_BUTTON_6_PAGE_3: String = String::new();

pub static mut IS_ON_WRITE_MODE_ON_BUTTON_1: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_2: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_3: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_4: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_5: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_6: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_7: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_8: bool = false;

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
pub fn handle_input(buttons: Vec<(bool, Color, Rect)>)
{   
    let event_pump = unsafe{&mut SDL2_EVENT_PUMP[0]};
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
                    for (index, object) in buttons.iter().enumerate()
                    {
                        if x >= object.2.x && x <= object.2.x + object.2.w && y >= object.2.y && y <= object.2.y + object.2.h
                        {
                                BUTTON_BEING_HOVERED = index; 
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
                        if IS_ON_WRITE_MODE_ON_BUTTON_1 { if (USER_INPUT_BUTTON_1.replace(" ", "").is_empty() && text == ".") || (USER_INPUT_BUTTON_1.contains('.') && text == ".") {continue;}; USER_INPUT_BUTTON_1.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_2 { if (USER_INPUT_BUTTON_2.replace(" ", "").is_empty() && text == ".") || (USER_INPUT_BUTTON_2.contains('.') && text == ".") {continue;}; USER_INPUT_BUTTON_2.push_str(&text); }
                        
                        // PAGE 3
                        if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 { if (USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").is_empty() && text == ".") || (USER_INPUT_BUTTON_1_PAGE_3.contains('.') && text == ".") {continue}; USER_INPUT_BUTTON_1_PAGE_3.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 { if (USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").is_empty() && text == ".") || (USER_INPUT_BUTTON_2_PAGE_3.contains('.') && text == ".") {continue}; USER_INPUT_BUTTON_2_PAGE_3.push_str(&text); }
                    }

                    if text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9"
                    {
                        // PAGE 1
                        if IS_ON_WRITE_MODE_ON_BUTTON_3 && USER_INPUT_BUTTON_3.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_3 = USER_INPUT_BUTTON_3.replace(" ", ""); USER_INPUT_BUTTON_3.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_4 && USER_INPUT_BUTTON_4.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_4 = USER_INPUT_BUTTON_4.replace(" ", ""); USER_INPUT_BUTTON_4.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_5 && USER_INPUT_BUTTON_5.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_5 = USER_INPUT_BUTTON_5.replace(" ", ""); USER_INPUT_BUTTON_5.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_6 && USER_INPUT_BUTTON_6.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_6 = USER_INPUT_BUTTON_6.replace(" ", ""); USER_INPUT_BUTTON_6.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_7 && USER_INPUT_BUTTON_7.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_7 = USER_INPUT_BUTTON_7.replace(" ", ""); USER_INPUT_BUTTON_7.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_8 && USER_INPUT_BUTTON_8.replace(" ", "").len() < 3 { USER_INPUT_BUTTON_8 = USER_INPUT_BUTTON_8.replace(" ", ""); USER_INPUT_BUTTON_8.push_str(&text); }

                        // PAGE 3
                        if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 && USER_INPUT_BUTTON_4_PAGE_3.replace("", "").len() < 4 { USER_INPUT_BUTTON_4_PAGE_3 = USER_INPUT_BUTTON_4_PAGE_3.replace(" ", ""); USER_INPUT_BUTTON_4_PAGE_3.push_str(&text); }
                        if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 && USER_INPUT_BUTTON_5_PAGE_3.replace("", "").len() < 2 { USER_INPUT_BUTTON_5_PAGE_3 = USER_INPUT_BUTTON_5_PAGE_3.replace(" ", ""); USER_INPUT_BUTTON_5_PAGE_3.push_str(&text); if !USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").is_empty() && USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").parse::<i32>().unwrap() > 12 || USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").len() < 2 && text == "0" { USER_INPUT_BUTTON_5_PAGE_3.pop(); } }
                        if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 && USER_INPUT_BUTTON_6_PAGE_3.replace("", "").len() < 2 { USER_INPUT_BUTTON_6_PAGE_3 = USER_INPUT_BUTTON_6_PAGE_3.replace(" ", ""); USER_INPUT_BUTTON_6_PAGE_3.push_str(&text); if !USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").is_empty() && USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").parse::<i32>().unwrap() > 31 || USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").len() < 2 && text == "0" { USER_INPUT_BUTTON_6_PAGE_3.pop(); } }
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
                    if IS_ON_WRITE_MODE_ON_BUTTON_1 { if USER_INPUT_BUTTON_1.len() == 1  { USER_INPUT_BUTTON_1.insert(0, ' '); USER_INPUT_BUTTON_1.pop(); } if !USER_INPUT_BUTTON_1.is_empty() { USER_INPUT_BUTTON_1.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_2 { if USER_INPUT_BUTTON_2.len() == 1  { USER_INPUT_BUTTON_2.insert(0, ' '); USER_INPUT_BUTTON_2.pop(); } if !USER_INPUT_BUTTON_2.is_empty() { USER_INPUT_BUTTON_2.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_3 { if USER_INPUT_BUTTON_3.len() == 1  { USER_INPUT_BUTTON_3.insert(0, ' '); USER_INPUT_BUTTON_3.pop(); } if !USER_INPUT_BUTTON_3.is_empty() { USER_INPUT_BUTTON_3.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_4 { if USER_INPUT_BUTTON_4.len() == 1  { USER_INPUT_BUTTON_4.insert(0, ' '); USER_INPUT_BUTTON_4.pop(); } if !USER_INPUT_BUTTON_4.is_empty() { USER_INPUT_BUTTON_4.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_5 { if USER_INPUT_BUTTON_5.len() == 1  { USER_INPUT_BUTTON_5.insert(0, ' '); USER_INPUT_BUTTON_5.pop(); } if !USER_INPUT_BUTTON_5.is_empty() { USER_INPUT_BUTTON_5.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_6 { if USER_INPUT_BUTTON_6.len() == 1  { USER_INPUT_BUTTON_6.insert(0, ' '); USER_INPUT_BUTTON_6.pop(); } if !USER_INPUT_BUTTON_6.is_empty() { USER_INPUT_BUTTON_6.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_7 { if USER_INPUT_BUTTON_7.len() == 1  { USER_INPUT_BUTTON_7.insert(0, ' '); USER_INPUT_BUTTON_7.pop(); } if !USER_INPUT_BUTTON_7.is_empty() { USER_INPUT_BUTTON_7.pop(); } }
                    if IS_ON_WRITE_MODE_ON_BUTTON_8 { if USER_INPUT_BUTTON_8.len() == 1  { USER_INPUT_BUTTON_8.insert(0, ' '); USER_INPUT_BUTTON_8.pop(); } if !USER_INPUT_BUTTON_8.is_empty() { USER_INPUT_BUTTON_8.pop(); } }

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
                     if IS_ON_WRITE_MODE_ON_BUTTON_1 { if !USER_INPUT_BUTTON_1.replace(" ", "").is_empty() { RETURN_VALUE = USER_INPUT_BUTTON_1.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_1.clear(); IS_ON_WRITE_MODE_ON_BUTTON_1 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_2 { if !USER_INPUT_BUTTON_2.replace(" ", "").is_empty() { TOTAL_INVESTED = USER_INPUT_BUTTON_2.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_2.clear(); IS_ON_WRITE_MODE_ON_BUTTON_2 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_3 { if !USER_INPUT_BUTTON_3.replace(" ", "").is_empty() { YEARS_INVESTED = USER_INPUT_BUTTON_3.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_3.clear(); IS_ON_WRITE_MODE_ON_BUTTON_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_4 { if !USER_INPUT_BUTTON_4.replace(" ", "").is_empty() { MONTHS_INVESTED = USER_INPUT_BUTTON_4.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_4.clear(); IS_ON_WRITE_MODE_ON_BUTTON_4 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_5 { if !USER_INPUT_BUTTON_5.replace(" ", "").is_empty() { DAYS_INVESTED = USER_INPUT_BUTTON_5.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_5.clear(); IS_ON_WRITE_MODE_ON_BUTTON_5 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_6 { if !USER_INPUT_BUTTON_6.replace(" ", "").is_empty() { HOURS_INVESTED = USER_INPUT_BUTTON_6.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_6.clear(); IS_ON_WRITE_MODE_ON_BUTTON_6 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_7 { if !USER_INPUT_BUTTON_7.replace(" ", "").is_empty() { MINUTES_INVESTED = USER_INPUT_BUTTON_7.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_7.clear(); IS_ON_WRITE_MODE_ON_BUTTON_7 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_8 { if !USER_INPUT_BUTTON_8.replace(" ", "").is_empty() { SECS_INVESTED = USER_INPUT_BUTTON_8.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_8.clear(); IS_ON_WRITE_MODE_ON_BUTTON_8 = false; }

                     // PAGE 3
                     if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 { if !USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").is_empty() { RETURN_PER_INVESTMENT = USER_INPUT_BUTTON_1_PAGE_3.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_1_PAGE_3.clear(); IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 { if !USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").is_empty() { TOTAL_INVESTED_PER_INVESTMENT = USER_INPUT_BUTTON_2_PAGE_3.replace(" ", "").parse().unwrap(); } USER_INPUT_BUTTON_2_PAGE_3.clear(); IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { if !USER_INPUT_BUTTON_3_PAGE_3.replace(" ", "").is_empty() { INVESTMENT_NAME = USER_INPUT_BUTTON_3_PAGE_3.clone(); } USER_INPUT_BUTTON_3_PAGE_3.clear(); IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 { if !USER_INPUT_BUTTON_4_PAGE_3.replace(" ", "").is_empty() { YEAR = USER_INPUT_BUTTON_4_PAGE_3.parse().unwrap(); USER_INPUT_BUTTON_4_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 { if !USER_INPUT_BUTTON_5_PAGE_3.replace(" ", "").is_empty() { MONTH = USER_INPUT_BUTTON_5_PAGE_3.parse().unwrap(); USER_INPUT_BUTTON_5_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 = false; }
                     if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { if !USER_INPUT_BUTTON_6_PAGE_3.replace(" ", "").is_empty() { DAY = USER_INPUT_BUTTON_6_PAGE_3.parse().unwrap(); USER_INPUT_BUTTON_6_PAGE_3.clear(); } IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 = false; }
                 }
            }
            
            Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
            {
                 unsafe
                 {
                     if IS_ON_WRITE_MODE_ON_BUTTON_1 || IS_ON_WRITE_MODE_ON_BUTTON_2 || IS_ON_WRITE_MODE_ON_BUTTON_3 || IS_ON_WRITE_MODE_ON_BUTTON_4 || IS_ON_WRITE_MODE_ON_BUTTON_5 || IS_ON_WRITE_MODE_ON_BUTTON_6 || IS_ON_WRITE_MODE_ON_BUTTON_7 || IS_ON_WRITE_MODE_ON_BUTTON_8 || IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 || IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3
                     {
                         //PAGE 1
                         USER_INPUT_BUTTON_1.clear();
                         USER_INPUT_BUTTON_2.clear();
                         USER_INPUT_BUTTON_3.clear();
                         USER_INPUT_BUTTON_4.clear();
                         USER_INPUT_BUTTON_5.clear();
                         USER_INPUT_BUTTON_6.clear();
                         USER_INPUT_BUTTON_7.clear();
                         USER_INPUT_BUTTON_8.clear();
                         USER_INPUT_BUTTON_1.push(' ');
                         USER_INPUT_BUTTON_2.push(' ');
                         USER_INPUT_BUTTON_3.push(' ');
                         USER_INPUT_BUTTON_4.push(' ');
                         USER_INPUT_BUTTON_5.push(' ');
                         USER_INPUT_BUTTON_6.push(' ');
                         USER_INPUT_BUTTON_7.push(' ');
                         USER_INPUT_BUTTON_8.push(' ');

                         IS_ON_WRITE_MODE_ON_BUTTON_1 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_2 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_3 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_4 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_5 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_6 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_7 = false;
                         IS_ON_WRITE_MODE_ON_BUTTON_8 = false;
                         
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

            Event::Quit { .. } => {},
            _ => {},
        }
    }
}


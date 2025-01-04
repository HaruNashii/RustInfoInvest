use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::process::exit;
use std::time::SystemTime;
use crate::window::SDL2_EVENT_PUMP;
use crate::math::{RETURN_VALUE, TOTAL_INVESTED};
use crate::pages::{RETURN_VALUE_REALTIME_PAGE, TOTAL_INVESTED_REALTIME_PAGE, CURRENT_TIME};


pub static mut USER_INPUT_BUTTON_1: String = String::new();
pub static mut USER_INPUT_BUTTON_2: String = String::new();
pub static mut USER_INPUT_BUTTON_1_PAGE_2: String = String::new();
pub static mut USER_INPUT_BUTTON_2_PAGE_2: String = String::new();

pub static mut IS_ON_WRITE_MODE_ON_BUTTON_1: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_2: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2: bool = false;
pub static mut IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2: bool = false;

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
                            unsafe { IS_HOVERING_BUTTON = false; };
                                        for (index, object) in buttons.iter().enumerate()
                                        {
                                            if x >= object.2.x && x <= object.2.x + object.2.w
                                                && y >= object.2.y && y <= object.2.y + object.2.h
                                            {
                                                unsafe 
                                                {
                                                    BUTTON_BEING_HOVERED = index; 
                                                    IS_HOVERING_BUTTON = true;
                                                };
                                            }
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
                                if IS_ON_WRITE_MODE_ON_BUTTON_1 && (text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9") || (!USER_INPUT_BUTTON_1.contains('.') && USER_INPUT_BUTTON_1.len() != 1 && text == ".")
                                {
                                    USER_INPUT_BUTTON_1.push_str(&text);
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2 && (text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9") || (!USER_INPUT_BUTTON_2.contains('.') && USER_INPUT_BUTTON_2.len() != 1 && text == ".")
                                {
                                    USER_INPUT_BUTTON_2.push_str(&text);
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 && (text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9") || (!USER_INPUT_BUTTON_1_PAGE_2.contains('.') && USER_INPUT_BUTTON_1_PAGE_2.len() != 1 && text == ".")
                                {
                                    USER_INPUT_BUTTON_1_PAGE_2.push_str(&text);
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2 && (text == "0" || text == "1" || text == "2" || text == "3" || text == "4" || text == "5" || text == "6" || text == "7" || text == "8" || text == "9") || (!USER_INPUT_BUTTON_2_PAGE_2.contains('.') && USER_INPUT_BUTTON_2_PAGE_2.len() != 1 && text == ".")
                                {
                                    USER_INPUT_BUTTON_2_PAGE_2.push_str(&text);
                                }
                            };
                        }

                        
                        Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                        {
                            unsafe 
                            {
                                if IS_ON_WRITE_MODE_ON_BUTTON_1
                                {
                                    if USER_INPUT_BUTTON_1.len() == 1 
                                    {
                                        USER_INPUT_BUTTON_1.insert(0, ' ');
                                        USER_INPUT_BUTTON_1.pop();
                                    }

                                    if !USER_INPUT_BUTTON_1.is_empty() 
                                    {
                                        USER_INPUT_BUTTON_1.pop();
                                    }
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2
                                {
                                    if USER_INPUT_BUTTON_2.len() == 1 
                                    {
                                        USER_INPUT_BUTTON_2.insert(0, ' ');
                                        USER_INPUT_BUTTON_2.pop();
                                    }

                                    if !USER_INPUT_BUTTON_2.is_empty() 
                                    {
                                        USER_INPUT_BUTTON_2.pop();
                                    }
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2
                                {
                                    if USER_INPUT_BUTTON_1_PAGE_2.len() == 1 
                                    {
                                        USER_INPUT_BUTTON_1_PAGE_2.insert(0, ' ');
                                        USER_INPUT_BUTTON_1_PAGE_2.pop();
                                    }

                                    if !USER_INPUT_BUTTON_1_PAGE_2.is_empty() 
                                    {
                                        USER_INPUT_BUTTON_1_PAGE_2.pop();
                                    }
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
                                {
                                    if USER_INPUT_BUTTON_2_PAGE_2.len() == 1 
                                    {
                                        USER_INPUT_BUTTON_2_PAGE_2.insert(0, ' ');
                                        USER_INPUT_BUTTON_2_PAGE_2.pop();
                                    }

                                    if !USER_INPUT_BUTTON_2_PAGE_2.is_empty() 
                                    {
                                        USER_INPUT_BUTTON_2_PAGE_2.pop();
                                    }
                                }
                            }
                        }


                       Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
                       {
                            unsafe 
                            {
                                if IS_ON_WRITE_MODE_ON_BUTTON_1
                                {
                                    if !USER_INPUT_BUTTON_1.replace(" ", "").is_empty()
                                    {
                                        RETURN_VALUE = USER_INPUT_BUTTON_1.replace(" ", "").parse().unwrap();
                                    }
                                    USER_INPUT_BUTTON_1.clear();
                                    IS_ON_WRITE_MODE_ON_BUTTON_1 = false;
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2
                                {
                                    if !USER_INPUT_BUTTON_2.replace(" ", "").is_empty()
                                    {
                                        TOTAL_INVESTED = USER_INPUT_BUTTON_2.replace(" ", "").parse().unwrap();
                                    }
                                    USER_INPUT_BUTTON_2.clear();
                                    IS_ON_WRITE_MODE_ON_BUTTON_2 = false;
                                }



                                if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2
                                {
                                    if !USER_INPUT_BUTTON_1_PAGE_2.replace(" ", "").is_empty()
                                    {
                                        RETURN_VALUE_REALTIME_PAGE = USER_INPUT_BUTTON_1_PAGE_2.replace(" ", "").parse().unwrap();
                                    }
                                    USER_INPUT_BUTTON_1_PAGE_2.clear();
                                    IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 = false;
                                }

                                if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
                                {
                                    if !USER_INPUT_BUTTON_2_PAGE_2.replace(" ", "").is_empty()
                                    {
                                        
                                        CURRENT_TIME = Some(SystemTime::now());
                                        TOTAL_INVESTED_REALTIME_PAGE = USER_INPUT_BUTTON_2_PAGE_2.replace(" ", "").parse().unwrap();
                                    }
                                    USER_INPUT_BUTTON_2_PAGE_2.clear();
                                    IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2 = false;
                                }
                            }
                       }
                       
                       Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
                       {
                            unsafe
                            {
                                if IS_ON_WRITE_MODE_ON_BUTTON_1 || IS_ON_WRITE_MODE_ON_BUTTON_2 || IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 || IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
                                {
                                    USER_INPUT_BUTTON_1.clear();
                                    USER_INPUT_BUTTON_2.clear();
                                    USER_INPUT_BUTTON_1.push(' ');
                                    USER_INPUT_BUTTON_2.push(' ');
                                    USER_INPUT_BUTTON_1_PAGE_2.clear();
                                    USER_INPUT_BUTTON_2_PAGE_2.clear();
                                    USER_INPUT_BUTTON_1_PAGE_2.push(' ');
                                    USER_INPUT_BUTTON_2_PAGE_2.push(' ');
                                    BUTTON_CLICKED = None;
                                    IS_ON_WRITE_MODE_ON_BUTTON_1 = false;
                                    IS_ON_WRITE_MODE_ON_BUTTON_2 = false;
                                    IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2 = false;
                                    IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2 = false;
                                } 
                                else 
                                {
                                    print!("\x1B[2J\x1B[1;1H");
                                    println!("bye bye :3");
                                    exit(0);
                                }
                            }
                       }

                        Event::Quit { .. } => {},

                       _ => {},
                }
            }
}


use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use crate::window::SDL2_EVENT_PUMP;
use std::process::exit;
use sdl2::rect::Rect;
use sdl2::pixels::Color;


pub static mut USER_INPUT: String = String::new();
pub static mut IS_ON_WRITE_MODE: bool = false;

pub static mut BUTTON_BEING_HOVERED: usize = 0;
pub static mut IS_HOVERING_BUTTON: bool = false;
pub static mut BUTTON_CLICKED: Option<usize> = None;


#[allow(static_mut_refs)]
pub fn handle_input(buttons: &Vec<(bool, Option<Color>, Rect)>)
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
                            if unsafe{IS_ON_WRITE_MODE}
                            {
                                unsafe{USER_INPUT.push_str(&text)};
                            }
                        }

                        
                        Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                        {
                            unsafe 
                            {
                                if IS_ON_WRITE_MODE
                                {
                                    if USER_INPUT.len() == 1 
                                    {
                                        USER_INPUT.insert(0, ' ');
                                        USER_INPUT.pop();
                                    }

                                    if !USER_INPUT.is_empty() 
                                    {
                                        USER_INPUT.pop();
                                    }
                                }
                            }
                        }


                       Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
                       {
                            unsafe 
                            {
                                if IS_ON_WRITE_MODE
                                {
                                    USER_INPUT.clear();
                                    IS_ON_WRITE_MODE = false;
                                }
                            }
                       }
                       
                       Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
                       {
                            unsafe
                            {
                                if IS_ON_WRITE_MODE
                                {
                                    USER_INPUT.clear();
                                    USER_INPUT.push(' ');
                                    IS_ON_WRITE_MODE = false;
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


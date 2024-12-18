use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::process::exit;
use crate::window::SDL2_EVENT_PUMP;
use crate::input_handler::IS_ON_WRITE_MODE;





pub fn handle_quit() 
{
    if unsafe{!IS_ON_WRITE_MODE}
    {
        let event_pump = unsafe{&mut SDL2_EVENT_PUMP[0]};
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
                {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("bye bye :3");
                    exit(0);
                }

                _ => {}
            }
        }
    }
}

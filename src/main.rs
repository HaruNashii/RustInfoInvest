use std::time::Duration;
use crate::
{
    window::{create_window, render_scene},
    quit::handle_quit,
};



mod window;
mod quit;
mod ui;
mod math;
mod getonlineinfo;



fn main()
{
    let (sdl_started, mut canvas, texture_creator) = create_window();


    let mut event_pump = sdl_started.event_pump().unwrap(); 
    loop 
    {
        std::thread::sleep(Duration::from_millis(32));

        render_scene(&mut canvas, &texture_creator);
        handle_quit(&mut event_pump);
    }
}

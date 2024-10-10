use std::process::exit;
use sdl2::event::Event;
use sdl2::EventPump;



pub fn handle_quit(event_pump: &mut EventPump)
{
    for event in event_pump.poll_iter()
    {
        match event
        {
            Event::Quit { .. } => exit(0),
            _=> ()
        }
    }
}

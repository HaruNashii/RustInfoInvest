use std::time::Duration;
use crate::pages::{main_page, persistent_page, second_page, selic_page};
use crate::window::{create_window, render_page};
use crate::input_handler::handle_input;
use crate::buttons::{PAGE_TO_RENDER, button_action};

pub mod math;
pub mod getonlineinfo;
pub mod pages;
pub mod window;
pub mod input_handler;
pub mod sdl2_generators;
pub mod buttons;





fn main() 
{
    create_window();


    loop
    {
        std::thread::sleep(Duration::from_millis(32));


        let main_page = main_page();
        let second_page = second_page();
        let selic_page = selic_page();
        let persistent_page = persistent_page();




        match unsafe{PAGE_TO_RENDER}
        {
            0 => 
            {
                let mut all_objects = Vec::new();
                all_objects.append(&mut persistent_page.buttons.clone().unwrap());
                all_objects.append(&mut main_page.buttons.clone().unwrap());
                handle_input(&mut all_objects);

                button_action();
                render_page(main_page, persistent_page);
            },
            1 =>
            {
                handle_input(&mut persistent_page.buttons.clone().unwrap());
                button_action();
                render_page(second_page, persistent_page);
            },
            2 =>
            {
                handle_input(&mut persistent_page.buttons.clone().unwrap());
                button_action();
                render_page(selic_page, persistent_page);
            },
            _=>{},
        }
    }

}

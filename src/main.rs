use std::time::Duration;
use crate::pages::{main_page, persistent_page, realtime_currency_page, selic_page, config_page};
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
        let realtime_currency_page = realtime_currency_page();
        let selic_page = selic_page();
        let persistent_page = persistent_page();
        let config_page = config_page();

        match unsafe{PAGE_TO_RENDER}
        {
            0 => 
            {
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut main_page.buttons.clone().unwrap());
                handle_input(all_buttons);

                button_action();
                render_page(main_page, Some(persistent_page));
            },
            1 =>
            {
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut realtime_currency_page.buttons.clone().unwrap());
                handle_input(all_buttons);
                button_action();
                render_page(realtime_currency_page, Some(persistent_page));
            },
            2 =>
            {
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut selic_page.buttons.clone().unwrap());
                handle_input(all_buttons);
                button_action();
                render_page(selic_page, Some(persistent_page));
            },
            3 =>
            {
                render_page(config_page, None);
            }
            _=>{},
        }
    }

}

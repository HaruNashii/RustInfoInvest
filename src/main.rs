use std::time::Duration;
use crate::pages::{calculator_page, persistent_page, realtime_currency_page, selic_page, investment_wallet_page};
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
pub mod investment_wallet;





fn main() 
{
    create_window();

    loop
    {
        std::thread::sleep(Duration::from_millis(33));
        

        let persistent_page = persistent_page();
        match unsafe{PAGE_TO_RENDER}
        {
            0 => 
            {
                let main_page = calculator_page();
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut main_page.buttons.clone().unwrap());
                handle_input(all_buttons);

                button_action();
                render_page(main_page, Some(persistent_page));
            },
            1 =>
            {
                let realtime_currency_page = realtime_currency_page();
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut realtime_currency_page.buttons.clone().unwrap());
                handle_input(all_buttons);
                button_action();
                render_page(realtime_currency_page, Some(persistent_page));
            },
            2 =>
            {
                let selic_page = selic_page();
                let mut all_buttons = Vec::new();
                all_buttons.append(&mut persistent_page.buttons.clone().unwrap());
                all_buttons.append(&mut selic_page.buttons.clone().unwrap());
                handle_input(all_buttons);
                button_action();
                render_page(selic_page, Some(persistent_page));
            },
            3 =>
            {
                let investment_wallet_page = investment_wallet_page();
                handle_input(investment_wallet_page.buttons.clone().unwrap());
                button_action();
                render_page(investment_wallet_page, None);
            }
            _=>{},
        }
    }

}

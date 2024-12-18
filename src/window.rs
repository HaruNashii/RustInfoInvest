use sdl2::video::{WindowContext, Window};
use sdl2::render::{TextureCreator, Canvas};
use crate::pages::Page;
use std::time::Duration;





pub static mut SDL2_CANVAS: Vec<Canvas<Window>> = Vec::new();
pub static mut SDL2_TEXTURE_CREATOR: Vec<TextureCreator<WindowContext>> = Vec::new();
pub static mut SDL2_EVENT_PUMP: Vec<sdl2::EventPump> = Vec::new();





#[allow(static_mut_refs)]
pub fn create_window()
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("RustInfoInvest", 800, 600).position_centered().build().unwrap();
    let canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();

    unsafe 
    { 
        SDL2_CANVAS.push(canvas);
        SDL2_TEXTURE_CREATOR.push(texture_creator);
        SDL2_EVENT_PUMP.push(event_pump);
    };

    while unsafe{SDL2_EVENT_PUMP.is_empty()} { println!("waiting for event pump"); std::thread::sleep(Duration::from_millis(250)) };
}





#[allow(static_mut_refs)]
pub fn render_page(page: Page, persistent_elements: Page)
{
    let canvas = unsafe {&mut SDL2_CANVAS[0]};
    canvas.set_draw_color(persistent_elements.background_color.unwrap());
    canvas.clear();

        if let Some(rect_vector_of_tuple) = &page.rects { for tuple in rect_vector_of_tuple { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }

        if let Some(buttons_vector_of_tuple) = &page.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }

        if let Some(texts_vector_of_tuple) = &page.texts { for tuple in texts_vector_of_tuple { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }


        //================================
        

        if let Some(rect_vector_of_tuple) = &persistent_elements.rects { for tuple in rect_vector_of_tuple { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }

        if let Some(buttons_vector_of_tuple) = &persistent_elements.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }

        if let Some(texts_vector_of_tuple) = &persistent_elements.texts { for tuple in texts_vector_of_tuple { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }

        canvas.present();
}

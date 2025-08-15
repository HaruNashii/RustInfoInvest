use sdl3::render::{TextureCreator, Canvas};
use sdl3::video::{WindowContext, Window};
use sdl3::EventPump;
use crate::pages::Page;





pub static mut SDL3_TEXTURE_CREATOR: Vec<TextureCreator<WindowContext>> = Vec::new();





#[allow(static_mut_refs)]
pub fn create_window() -> (Canvas<Window>, EventPump)
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("RustInfoInvest", 800, 600).position_centered().build().unwrap();
    video_system.text_input().start(&window);
    let canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();
    

    unsafe 
    { 
        SDL3_TEXTURE_CREATOR.push(texture_creator);
    };

    (canvas, event_pump)
}



#[allow(static_mut_refs)]
pub fn render_page(page: Page, persistent_page: Option<Page>, canvas: &mut Canvas<Window>)
{
    canvas.set_draw_color(page.background_color.unwrap());
    canvas.clear();

    if let Some(rect_vector_of_tuple) =    &page.rects   { for tuple in rect_vector_of_tuple    { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }
    if let Some(buttons_vector_of_tuple) = &page.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }
    if let Some(texts_vector_of_tuple) =   &page.texts   { for tuple in texts_vector_of_tuple   { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }

    if let Some(persistent_elements) = persistent_page 
    {
        if let Some(rect_vector_of_tuple) =    &persistent_elements.rects   { for tuple in rect_vector_of_tuple    { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }
        if let Some(buttons_vector_of_tuple) = &persistent_elements.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }
        if let Some(texts_vector_of_tuple) =   &persistent_elements.texts   { for tuple in texts_vector_of_tuple   { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    }

    canvas.present();
}

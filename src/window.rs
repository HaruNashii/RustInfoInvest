use crate::ui::{objects, fonts};
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};



const WINDOW_WIDTH: u32 = 1360;
const WINDOW_HEIGHT: u32 = 768;



pub fn create_window() -> (sdl2::Sdl, Canvas<Window>, TextureCreator<WindowContext>)
{
    let sdl_started = sdl2::init().unwrap();
    let sdl_videosystem = sdl_started.video().unwrap();
    let window = sdl_videosystem.window("ruinvest", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    canvas.set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let texture_creator = canvas.texture_creator();


                (
                    sdl_started,
                    canvas,
                    texture_creator
                )
}



pub fn render_scene(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<sdl2::video::WindowContext>)
{
    //pick the objects
    let 
    (  
        invested_value_rect,
        return_percentage_value_rect,
        one_year_rect,
        one_month_rect,
        one_day_rect,
        one_hour_rect, 
        one_min_rect,
        one_secs_rect
    ) = objects();


        //pick the textures/texts/images
        let 
        (
            invested_value_text_image,
            return_percentage_value_text_image,
            one_year_text_image,
            one_month_text_image,
            one_day_text_image,        
            one_hour_text_image, 
            one_min_text_image,
            one_secs_text_image,
        ) = fonts(texture_creator);


            canvas.set_draw_color(Color::RGB(0,0,0));
            canvas.clear();


                canvas.copy(&invested_value_text_image, None, invested_value_rect).unwrap();
                canvas.copy(&return_percentage_value_text_image, None, return_percentage_value_rect).unwrap();
                canvas.copy(&one_year_text_image, None, one_year_rect).unwrap();
                canvas.copy(&one_month_text_image, None, one_month_rect).unwrap();
                canvas.copy(&one_day_text_image, None, one_day_rect).unwrap();
                canvas.copy(&one_hour_text_image, None, one_hour_rect).unwrap();
                canvas.copy(&one_min_text_image, None, one_min_rect).unwrap();
                canvas.copy(&one_secs_text_image, None, one_secs_rect).unwrap();


                    canvas.present();
}

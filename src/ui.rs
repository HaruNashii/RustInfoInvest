//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES FRONT-END------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Texture, TextureCreator};
use sdl2::{rect::Rect, render::Canvas};
use std::time::Duration;

use crate::math::maths;
use crate::math::read_ron_file;


//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------FRONT-END-------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
const WINDOW_WIDTH: u32 = 1360;
const WINDOW_HEIGHT: u32 = 768;

const DEFAULT_FONT_SIZE: u16 = 20;
const DEFAULT_RECT_FONT_SIZE: [u32;2] = [650, 75];
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);

const VALUE_INVESTED_TEXT: &str = "VALUE INVESTED =";
const VALUE_INVEST_TEXT_POS: [i32;2] = [364, 50];
const RETURN_PERCENTAGE_TEXT: &str = "RETURN PERCENTAGE =";
const RETURN_PERCENTAGE_POS: [i32;2] = [364, 125];

const VALUE_RETURN_TEXT: [&str;6] = ["RETURN IN", "RETURN IN", "RETURN PER DAY =", "RETURN PER HOUR =", "RETURN PER MINUTE =", "RETURN PER SECOND ="];
const VALUE_RETURN_TEXT_POS_X: i32 = 364;
const VALUE_RETURN_TEXT_POS_Y: [i32;6] = [200, 250, 300, 350, 400, 450];





pub fn exec_fn()
{
    let (sdl_started, canvas, texture_creator                ) = window();

    fnloop(sdl_started, canvas, texture_creator);
}





fn window() -> (sdl2::Sdl, Canvas<Window>, TextureCreator<WindowContext>)
{
    let sdl_started = sdl2::init().unwrap();
    let sdl_videosystem = sdl_started.video().unwrap();
    let window = sdl_videosystem
    .window("ruinvest", WINDOW_WIDTH, WINDOW_HEIGHT)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())
    .unwrap();

    let mut canvas = window.into_canvas()
    .accelerated()
    .build()
    .map_err(|e| e.to_string())
    .unwrap();
    canvas.set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let texture_creator = canvas.texture_creator();

    return(sdl_started, canvas, texture_creator)
}





pub fn fnloop(sdl_started: sdl2::Sdl, mut canvas: Canvas<Window>, texture_creator: TextureCreator<WindowContext>)
{

    let mut event_pump = sdl_started.event_pump().unwrap(); 
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(32));

        render_scene(&mut canvas, &texture_creator);

        for event in event_pump.poll_iter()
        {
            match event
            {
                sdl2::event::Event::Quit { .. } => { break 'running; }
                _=> ()
            }
        }
    }
}





fn render_scene(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<sdl2::video::WindowContext>)
{
    //pick the objects
    let (  
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
    let (
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





fn text_image_generator<'a>(additional_text: &str, main_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>) -> Texture<'a>
{
    let ttf_started = sdl2::ttf::init().unwrap();
    let font = ttf_started.load_font(DEFAULT_FONT_PATH, DEFAULT_FONT_SIZE).unwrap();
    let surface = font
    .render(&format! ("{} {}", additional_text, main_text))
    .blended(DEFAULT_FONT_COLOR)
    .unwrap();

    let texture = texture_creator
    .create_texture_from_surface(&surface).unwrap();

    return texture;
}





fn fonts<'a>(texture_creator: &'a TextureCreator<sdl2::video::WindowContext>) -> (Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>)
{
    let (one_year, one_month, one_day, one_hour, one_min, one_secs) = maths();
    let (ron_file_total_invested, ron_file_return_value, ron_file_cdi_value, ron_file_cdi_value_translated, _, use_cdi_value, ron_file_years_invested, ron_file_months_invested) = read_ron_file();

    let mut return_value_as_string: String = format!("{}% p.a", ron_file_return_value.to_string());

    if use_cdi_value
    {
        return_value_as_string = format!("{}% / {}% p.a", ron_file_cdi_value, ron_file_cdi_value_translated);
    };


    let invested_value_text_image = text_image_generator(VALUE_INVESTED_TEXT, &ron_file_total_invested.to_string(), texture_creator);
    let return_percentage_value_text_image = text_image_generator(RETURN_PERCENTAGE_TEXT, &return_value_as_string, texture_creator);

    let mut year_string = String::new();
    if ron_file_years_invested <= 1
    {
        year_string = format!("{} {} YEAR =", VALUE_RETURN_TEXT[0], ron_file_years_invested);
    }
    if ron_file_years_invested >= 2
    {
        year_string = format!("{} {} YEARS =", VALUE_RETURN_TEXT[0], ron_file_years_invested);
    }

    let mut month_string = String::new();
    if ron_file_months_invested <= 1
    {
        month_string = format!("{} {} MONTH =", VALUE_RETURN_TEXT[1], ron_file_months_invested);
    }
    if ron_file_months_invested >= 2
    {
        month_string = format!("{} {} MONTHS =", VALUE_RETURN_TEXT[1], ron_file_months_invested);
    }
    let one_year_text_image = text_image_generator(&year_string, &one_year, texture_creator);
    let one_month_text_image = text_image_generator(&month_string, &one_month, texture_creator);
    let one_day_text_image = text_image_generator(VALUE_RETURN_TEXT[2], &one_day, texture_creator);
    let one_hour_text_image = text_image_generator(VALUE_RETURN_TEXT[3], &one_hour, texture_creator);
    let one_min_text_image = text_image_generator(VALUE_RETURN_TEXT[4], &one_min, texture_creator);
    let one_secs_text_image = text_image_generator(VALUE_RETURN_TEXT[5], &one_secs, texture_creator);

    return (
            invested_value_text_image,
            return_percentage_value_text_image,
            one_year_text_image,
            one_month_text_image,
            one_day_text_image,        
            one_hour_text_image, 
            one_min_text_image,
            one_secs_text_image,
            );
}





fn objects() -> (Rect, Rect, Rect, Rect, Rect, Rect, Rect, Rect)
{
    let invested_value_rect = Rect::new(VALUE_INVEST_TEXT_POS[0], VALUE_INVEST_TEXT_POS[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let return_percentage_value_rect = Rect::new(RETURN_PERCENTAGE_POS[0], RETURN_PERCENTAGE_POS[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);

    let one_year_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[0], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let one_month_rect = Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let one_day_rect =   Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[2], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let one_hour_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[3], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let one_min_rect =   Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[4], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let one_secs_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[5], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);


    return (
            invested_value_rect,
            return_percentage_value_rect,
            one_year_rect,
            one_month_rect,
            one_day_rect,
            one_hour_rect, 
            one_min_rect,
            one_secs_rect,
            );
}
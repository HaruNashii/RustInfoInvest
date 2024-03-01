//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use percentage::Percentage;
use std::fs::File;
use std::io::Read;
use ron::de::from_str;
use serde_derive::Deserialize;




//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------BACK-END--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
#[derive(Deserialize)]
struct RonFloatValues 
{
    return_value: f32,
    total_invested: f32,
}





fn round_float(value: f32, precision: usize) -> f32 
{
    let factor = 10.0_f32.powi(precision as i32);
    (value * factor).round() / factor
}

fn read_ron_file() -> (f32, f32)
{
 // Open the file
 let mut file = File::open("config/data.ron").unwrap();
    
 // Read the contents of the file into a String
 let mut content = String::new();
 file.read_to_string(&mut content).unwrap();

 // Parse the RON content into a MyStruct
 let my_struct: RonFloatValues = from_str(&content).expect("Failed to parse RON");

 // Round the float to remove unecessary decimal numbers
 let total_invested = round_float(my_struct.total_invested, 2);
 let return_value = round_float(my_struct.return_value, 2);

 return (total_invested, return_value);
}

fn maths() -> (String, String, String, String, String, String)
{
    let (ron_file_total_invested, ron_file_return_value) = read_ron_file();

    let ron_file_return_value = ron_file_return_value / 100.0;
    let percent = Percentage::from_decimal(ron_file_return_value.into());

    let one_year = percent.apply_to(ron_file_total_invested.into());
    let one_month = one_year / 12.0;
    let one_day = one_month / 31.0;
    let one_hour = one_day / 24.0;
    let one_min = one_day / 60.0;
    let one_secs = one_min / 60.0;

    let one_year = round_float(one_year as f32, 2);
    let one_month = round_float(one_month as f32, 2);
    let one_day = round_float(one_day as f32, 2);
    let one_hour = round_float(one_hour as f32, 2);
    let one_min = round_float(one_min as f32, 2);
    let one_secs = round_float(one_secs as f32, 2);
    
    return (one_year.to_string(), one_month.to_string(), one_day.to_string(), one_hour.to_string(), one_min.to_string(), one_secs.to_string());
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES BACK-END-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Texture, TextureCreator};
use sdl2::{rect::Rect, render::Canvas};
use std::time::Duration;





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------FRONT-END-------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
const WINDOW_WIDTH: u32 = 1360;
const WINDOW_HEIGHT: u32 = 768;

const DEFAULT_FONT_SIZE: u16 = 20;
const DEFAULT_RECT_FONT_SIZE: [u32;2] = [550, 75];
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);

const VALUE_INVESTED_TEXT: &str = "VALUE INVESTED =";
const VALUE_INVEST_TEXT_POS: [i32;2] = [364, 50];
const RETURN_PERCENTAGE_TEXT: &str = "RETURN PERCENTAGE =";
const RETURN_PERCENTAGE_POS: [i32;2] = [364, 125];

const VALUE_RETURN_TEXT: [&str;6] = ["RETURN PER YEAR =", "RETURN PER MONTH =", "RETURN PER DAY =", "RETURN PER HOUR =", "RETURN PER MINUTE =", "RETURN PER SECOND ="];
const VALUE_RETURN_TEXT_POS_X: i32 = 364;
const VALUE_RETURN_TEXT_POS_Y: [i32;6] = [200, 250, 300, 350, 400, 450];






fn main()
{
    fnloop();
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

fn fnloop()
{
    let (sdl_started, mut canvas, texture_creator) = window();

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
    let (ron_file_total_invested, ron_file_return_value) = read_ron_file();

    let invested_value_text_image = text_image_generator(VALUE_INVESTED_TEXT, &ron_file_total_invested.to_string(), texture_creator);
    let return_percentage_value_text_image = text_image_generator(RETURN_PERCENTAGE_TEXT, &ron_file_return_value.to_string(), texture_creator);

    let one_year_text_image = text_image_generator(VALUE_RETURN_TEXT[0], &one_year, texture_creator);
    let one_month_text_image = text_image_generator(VALUE_RETURN_TEXT[1], &one_month, texture_creator);
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
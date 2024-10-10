//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES FRONT-END------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::Rect;
use format_num::NumberFormat;
use crate::math::maths;
use crate::math::read_ron_file;



//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------FRONT-END-------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
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



fn text_generator<'a>(additional_text: &str, main_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>) -> Texture<'a>
{
    let ttf_started = sdl2::ttf::init().unwrap();
    let font = ttf_started.load_font(DEFAULT_FONT_PATH, DEFAULT_FONT_SIZE).unwrap();
    let surface = font.render(&format! ("{} {}", additional_text, main_text)).blended(DEFAULT_FONT_COLOR).unwrap();
    texture_creator.create_texture_from_surface(&surface).unwrap()
}



fn format_string() -> (String, String, String, String, String, String)
{
    let (one_year, one_month, one_day, one_hour, one_min, one_secs) = maths();


        let num = NumberFormat::new();
        let format_spec = ",.2";
        let format_spec_for_secs_and_min = ",.4";

        let one_year  = num.format(format_spec, one_year);
        let one_month = num.format(format_spec, one_month);
        let one_day   = num.format(format_spec, one_day);
        let one_hour  = num.format(format_spec, one_hour);
        let one_min   = num.format(format_spec_for_secs_and_min, one_min);
        let one_secs  = num.format(format_spec_for_secs_and_min, one_secs);


                (
                    one_year,
                    one_month,
                    one_day,
                    one_hour,
                    one_min,
                    one_secs
                )
}



pub fn fonts(texture_creator: &TextureCreator<sdl2::video::WindowContext>) -> (Texture<'_>, Texture<'_>, Texture<'_>, Texture<'_>, Texture<'_>, Texture<'_>, Texture<'_>, Texture<'_>)
{   

    let (one_year, one_month, one_day, one_hour, one_min, one_secs) = format_string();
    let ( ron_file_total_invested, ron_file_return_value, ron_file_cdi_value, ron_file_cdi_value_translated, _, use_cdi_value, ron_file_years_invested, ron_file_months_invested) = read_ron_file();


        let num = NumberFormat::new();
        let ron_file_total_invested = num.format(",.2", ron_file_total_invested);
        let ron_file_return_value =   num.format(",.2", ron_file_return_value);
        let ron_file_cdi_value =      num.format(",.2", ron_file_cdi_value);
        let ron_file_cdi_value_translated = num.format(",.2", ron_file_cdi_value_translated);


        let mut return_value_as_string: String = format!("{}% p.a", ron_file_return_value);
        if use_cdi_value { return_value_as_string = format!("{}% / {}% p.a", ron_file_cdi_value, ron_file_cdi_value_translated); };


            let invested_value_text_image = text_generator(VALUE_INVESTED_TEXT, &ron_file_total_invested.to_string(), texture_creator);
            let return_percentage_value_text_image = text_generator(RETURN_PERCENTAGE_TEXT, &return_value_as_string, texture_creator);


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


                    let one_year_text_image = text_generator(&year_string, &one_year, texture_creator);
                    let one_month_text_image = text_generator(&month_string, &one_month, texture_creator);
                    let one_day_text_image = text_generator(VALUE_RETURN_TEXT[2], &one_day, texture_creator);
                    let one_hour_text_image = text_generator(VALUE_RETURN_TEXT[3], &one_hour, texture_creator);
                    let one_min_text_image = text_generator(VALUE_RETURN_TEXT[4], &one_min, texture_creator);
                    let one_secs_text_image = text_generator(VALUE_RETURN_TEXT[5], &one_secs, texture_creator);


                        (
                            invested_value_text_image,
                            return_percentage_value_text_image,
                            one_year_text_image,
                            one_month_text_image,
                            one_day_text_image,        
                            one_hour_text_image, 
                            one_min_text_image,
                            one_secs_text_image,
                        )
}



pub fn objects() -> (Rect, Rect, Rect, Rect, Rect, Rect, Rect, Rect)
{
    let invested_value_rect = Rect::new(VALUE_INVEST_TEXT_POS[0], VALUE_INVEST_TEXT_POS[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
    let return_percentage_value_rect = Rect::new(RETURN_PERCENTAGE_POS[0], RETURN_PERCENTAGE_POS[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);


        let one_year_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[0], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
        let one_month_rect = Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[1], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
        let one_day_rect =   Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[2], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
        let one_hour_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[3], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
        let one_min_rect =   Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[4], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);
        let one_secs_rect =  Rect::new(VALUE_RETURN_TEXT_POS_X, VALUE_RETURN_TEXT_POS_Y[5], DEFAULT_RECT_FONT_SIZE[0], DEFAULT_RECT_FONT_SIZE[1]);


            (
                invested_value_rect,
                return_percentage_value_rect,
                one_year_rect,
                one_month_rect,
                one_day_rect,
                one_hour_rect, 
                one_min_rect,
                one_secs_rect,
            )
}

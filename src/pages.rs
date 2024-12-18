use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use crate::sdl2_generators::gen_text;
use crate::math::{basic_data, maths, ONLINE_HISTORIC_RETURN_VALUE};
use crate::input_handler::USER_INPUT;





pub struct Page<'a>
{
    pub background_color: Option<Color>,
    pub rects:   Option<Vec< (Color, Rect) >>,
    pub buttons: Option<Vec< (bool, Option<Color>, Rect) >>,
    pub texts:   Option<Vec< (Texture<'a>, Rect) >>,
    pub images:  Option<Vec< (Texture<'a>, Rect) >>,
}





pub fn persistent_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(24, 24, 37);
    let default_text_color = Color::RGB(205, 214, 244);



    //===================== rects =========================
    let all_rects = vec!
    [
        //header background
        (Color::RGB(250, 179, 135), Rect::new(0, 0, 800, 100)),
    ];
    


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //main page button
        (true, Some(Color::RGB(166, 227, 161)),   Rect::new(75, 10, 200, 75)),
        //secound page button
        (true, Some(Color::RGB(166, 227, 161)),   Rect::new(300, 10, 200, 75)),
        //selic page button
        (true, Some(Color::RGB(166, 227, 161)),   Rect::new(525, 10, 200, 75)),
    ];



    //===================== texts =========================
    let all_text = vec!
    [
        //main page button text
        gen_text(20, (all_buttons[0].2.x + 25, all_buttons[0].2.y + 15), " Main Page ".to_string(), default_text_color),
        //second page button text
        gen_text(20, (all_buttons[1].2.x + 25, all_buttons[1].2.y + 15), "Second Page".to_string(), default_text_color),
        //selic page button text
        gen_text(20, (all_buttons[2].2.x + 25, all_buttons[2].2.y + 15), " Selic Page".to_string(), default_text_color),
    ];



    //===================== page creation =========================
    Page 
    {
        background_color: Some(bg_color),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}




#[allow(static_mut_refs)]
pub fn main_page() -> Page<'static>
{
    //===================== variables =========================
    let default_text_color = Color::RGB(205, 214, 244);
    let ( one_year, one_month, one_day, one_hour, one_min, one_secs,) = maths();
    let ( total_invested, year_return_value, _, _, years_invested, months_invested, days_invested, hours_invested, minutes_invested) = basic_data();
    unsafe { if USER_INPUT.is_empty() { USER_INPUT.push(' ') }; };



    //===================== rects =========================
    let all_rects = vec!
    [
        //header info background
        (Color::RGB(203, 166, 247), Rect::new(15, 120, 760, 100)),
    ];



    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button
        (true, Some(Color::RGB(0,   255, 0)),   Rect::new(500, 250, 250, 75)),
    ];
    





    //===================== texts =========================
    let year_string: String   = format!("Year:   R$ {:.2}", one_year);
    let month_string: String  = format!("Month:  R$ {:.2}", one_month);
    let day_string: String    = format!("Day:    R$ {:.3}", one_day);
    let hour_string: String   = format!("Hour:   R$ {:.3}", one_hour);
    let minute_string: String = format!("Minute: R$ {:.4}", one_min);
    let second_string: String = format!("Second: R$ {:.4}", one_secs);
    
    let all_text = vec!
    [
        //one year text
        gen_text(20, (30, 340), year_string, default_text_color),
        //one month text
        gen_text(20, (30, 360), month_string, default_text_color),
        //one day text
        gen_text(20, (30, 380), day_string, default_text_color),
        //one hour text
        gen_text(20, (30, 400), hour_string, default_text_color),
        //one min text
        gen_text(20, (30, 420), minute_string, default_text_color),
        //one sec text
        gen_text(20, (30, 440), second_string, default_text_color),
        //total invested text
        gen_text(20, (30, 140), format!("Total Invested : R$ {}", total_invested    ), default_text_color),
        //year return value text
        gen_text(20, (30, 160), format!("Year Return Value : {}%", year_return_value), default_text_color),
        //time invested text     
        gen_text(20, (30, 180), format!("Time Invested : {} Years, {} Months, {} Days, {} Hours, {} Minutes", years_invested, months_invested, days_invested, hours_invested, minutes_invested), default_text_color),
        //receive input button text
        gen_text(20, (all_buttons[0].2.x + 15, all_buttons[0].2.y + 15), "Receive Input".to_string(), default_text_color),
        //user input text
        gen_text(20, (10, 450), unsafe{USER_INPUT.clone()}, default_text_color),
    ];

    

    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





pub fn second_page() -> Page<'static>
{
    //===================== variables =========================
    let default_text_color = Color::RGB(205, 214, 244);



    //===================== texts =========================
    let all_text = vec!
    [
        //example text
        gen_text(20, (30, 340),  "Example".to_string(), default_text_color),
    ];



    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   None,
        buttons: None,
        texts:   Some( all_text ),
        images:  None,
    }
}





#[allow(static_mut_refs)]
pub fn selic_page() -> Page<'static>
{
    //===================== variables =========================
    let default_text_color = Color::RGB(205, 214, 244);



    //===================== texts =========================
    let mut all_text = vec![gen_text(25, (150, 115), " Number       Date    Percentage %".to_string(), default_text_color)];
    for (index, string) in unsafe{ONLINE_HISTORIC_RETURN_VALUE.iter().enumerate()}
    {
        let text = gen_text(20, (200, 180 + (40 * index as i32)),  string.to_string(), default_text_color);
        all_text.push(text);
    }



    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   None,
        buttons: None,
        texts:   Some( all_text ),
        images:  None,
    }
}

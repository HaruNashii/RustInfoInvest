use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use crate::sdl2_generators::gen_text;
use crate::math::{maths, ONLINE_HISTORIC_RETURN_VALUE, RETURN_VALUE, TOTAL_INVESTED};
use crate::input_handler::{USER_INPUT_BUTTON_1, USER_INPUT_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_2};
use std::time::SystemTime;
use std::sync::Once;

static START: Once = Once::new();
static mut CURRENT_TIME: Option<SystemTime> = None; 
static mut REALTIME_SECS: f64 = 0.0;
static mut REALTIME_CURRENCY: f64 = 0.0;

pub struct Page<'a>
{
    pub background_color: Option<Color>,
    pub rects:   Option<Vec< (Color, Rect) >>,
    pub buttons: Option<Vec< (bool, Color, Rect) >>,
    pub texts:   Option<Vec< (Texture<'a>, Rect) >>,
    pub images:  Option<Vec< (Texture<'a>, Rect) >>,
}





pub fn persistent_page() -> Page<'static>
{
    //===================== variables =========================
    let default_text_color = Color::RGB(255, 255, 255);



    //===================== rects =========================
    let all_rects = vec!
    [
        //header background
        (Color::RGB(17, 17, 27), Rect::new(0, 0, 800, 100)),
    ];
    


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //main page button
        (true, Color::RGB(243, 139, 168), Rect::new(75, 10, 200, 75)),
        //secound page button
        (true, Color::RGB(243, 139, 168), Rect::new(300, 10, 200, 75)),
        //selic page button
        (true, Color::RGB(243, 139, 168), Rect::new(525, 10, 200, 75)),
    ];



    //===================== texts =========================
    let all_text = vec!
    [
        //main page button text
        gen_text(20, (all_buttons[0].2.x + 30, all_buttons[0].2.y + 20), " Main Page ".to_string(), default_text_color),
        //second page button text
        gen_text(19, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 20), "Realtime Currency".to_string(), default_text_color),
        //selic page button text
        gen_text(20, (all_buttons[2].2.x + 30, all_buttons[2].2.y + 20), " Selic Page".to_string(), default_text_color),
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




#[allow(static_mut_refs)]
pub fn main_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color      = Color::RGB(186, 194, 222);
    let ( one_year, one_month, one_day, one_hour, one_min, one_secs) = maths();
    
    unsafe { if USER_INPUT_BUTTON_1.is_empty() { USER_INPUT_BUTTON_1.push(' ') }; };
    unsafe { if USER_INPUT_BUTTON_2.is_empty() { USER_INPUT_BUTTON_2.push(' ') }; };



    //===================== rects =========================
    let all_rects = vec!
    [
        //header info background
        (Color::RGB(203, 166, 247), Rect::new(0, 105, 800, 50)),
    ];



    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(166,   227, 161),   Rect::new(20, 105, 365, 50)),
        //receive input button 2
        (true, Color::RGB(166,   227, 161),   Rect::new(415, 105, 355, 50)),
    ];
    


    //===================== texts =========================
    let year_string: String   = format!("Return Per Year:   R$ {:.2}", one_year);
    let month_string: String  = format!("Return Per Month:  R$ {:.2}", one_month);
    let day_string: String    = format!("Return Per Day:    R$ {:.3}", one_day);
    let hour_string: String   = format!("Return Per Hour:   R$ {:.3}", one_hour);
    let minute_string: String = format!("Return Per Minute: R$ {:.4}", one_min);
    let second_string: String = format!("Return Per Second: R$ {:.4}", one_secs);
    
    let mut all_text = vec!
    [
        //one year text
        gen_text(20, (225, 245), year_string, subtext_color),
        //one month text
        gen_text(20, (225, 295), month_string, subtext_color),
        //one day text
        gen_text(20, (225, 345), day_string, subtext_color),
        //one hour text
        gen_text(20, (225, 395), hour_string, subtext_color),
        //one min text
        gen_text(20, (225, 445), minute_string, subtext_color),
        //one sec text
        gen_text(20, (225, 495), second_string, subtext_color),
        //user input text
        gen_text(24, (284, 113), unsafe{USER_INPUT_BUTTON_1.clone()}, default_text_color),
        gen_text(24, (663, 113), unsafe{USER_INPUT_BUTTON_2.clone()}, default_text_color),
    ];
    unsafe
    {
        //year return value text
        if IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(24, (30, 113), "Year Return Value: ".to_string(), default_text_color));
        }

        //year return value text
        if !IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(24, (30, 113), format!("Year Return Value: {}%", RETURN_VALUE), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(24, (425, 113), "Total Invested: R$".to_string(), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(24, (425, 113), format!("Total Invested: R${}", TOTAL_INVESTED), default_text_color));
        }
    };
    

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
pub fn realtime_currency_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color      = Color::RGB(186, 194, 222);


    unsafe
    {
        // Move numbers one case to the right to fit the formula math (example = 1.0 -> 0.1)
        let mut year_return_value: f64 = RETURN_VALUE;
        year_return_value /= 100.0;

        let month_return_value  = f64::powf(1.0 + year_return_value,   1.00 / 12.00) - 1.0;
        let day_return_value    = f64::powf(1.0 + month_return_value,  1.00 / 30.00) - 1.0;
        let hour_return_value   = f64::powf(1.0 + day_return_value,    1.00 / 24.00) - 1.0;
        let minute_return_value = f64::powf(1.0 + hour_return_value,   1.00 / 60.00) - 1.0;
        let secs_return_value   = f64::powf(1.0 + minute_return_value, 1.00 / 60.00) - 1.0;

        // Formulas
        // formula = total_invested * (1 + return_value)^total_time_invested
        REALTIME_SECS = REALTIME_CURRENCY * f64::powf(1.0 + secs_return_value,  1.0) - REALTIME_CURRENCY;


        START.call_once
        (|| {
            CURRENT_TIME = Some(SystemTime::now())
        });

        let secs_since_checked_current_time = CURRENT_TIME.unwrap().elapsed().unwrap().as_secs();
        REALTIME_CURRENCY = TOTAL_INVESTED + (REALTIME_SECS * secs_since_checked_current_time as f64);
        
        if USER_INPUT_BUTTON_1.is_empty() { USER_INPUT_BUTTON_1.push(' ') }; 
        if USER_INPUT_BUTTON_2.is_empty() { USER_INPUT_BUTTON_2.push(' ') }; 
    };
    


    //===================== rects =========================
    let all_rects = vec!
    [
        //header info background
        (Color::RGB(203, 166, 247), Rect::new(0, 105, 800, 50)),
    ];



    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(166,   227, 161),   Rect::new(20, 105, 365, 50)),
        //receive input button 2
        (true, Color::RGB(166,   227, 161),   Rect::new(415, 105, 355, 50)),
    ];
    


    //===================== texts =========================
    let realtime_currency_string: String = format!("Realtime Currency: R$ {:.4}", unsafe{REALTIME_CURRENCY});
    let second_string: String = format!("Return Per Second: R$ {:.6}", unsafe{REALTIME_SECS});
    
    let mut all_text = vec!
    [
        //realtime curreny text
        gen_text(30, (100, 350), realtime_currency_string, subtext_color),
        //one sec text
        gen_text(20, (225, 495), second_string, subtext_color),
        //user input text
        gen_text(24, (284, 113), unsafe{USER_INPUT_BUTTON_1.clone()}, default_text_color),
        gen_text(24, (663, 113), unsafe{USER_INPUT_BUTTON_2.clone()}, default_text_color),
    ];
    unsafe
    {
        //year return value text
        if IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(24, (30, 113), "Year Return Value: ".to_string(), default_text_color));
        }

        //year return value text
        if !IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(24, (30, 113), format!("Year Return Value: {}%", RETURN_VALUE), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(24, (425, 113), "Total Invested: R$".to_string(), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(24, (425, 113), format!("Total Invested: R${}", TOTAL_INVESTED), default_text_color));
        }
    };
    

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
pub fn selic_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color = Color::RGB(186, 194, 222);


    //===================== rects =========================
    let all_rects = vec!
    [
        //header info background
        (Color::RGB(203, 166, 247), Rect::new(0, 105, 800, 50)),
    ];


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //sync online info button
        (true, Color::RGB(166, 227, 161), Rect::new(300, 105, 200, 50)),
    ];



    //===================== texts =========================
    let mut all_text = vec!
    [
        //sync online info button text
        gen_text(20, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "Sync With Online".to_string(), default_text_color),
        gen_text(25, (125, 175), "Number          Date         Selic Tax".to_string(), default_text_color),
    ];
    for (index, string) in unsafe{ONLINE_HISTORIC_RETURN_VALUE.iter().enumerate()}
    {
        let text = gen_text(20, (145, 300 + (40 * index as i32)),  string.to_string(), subtext_color);
        all_text.push(text);
    }



    //===================== page creation =========================
    Page 
    {
        background_color: Some(bg_color),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}



pub fn config_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);



    //===================== buttons =========================
    let all_buttons = vec!
    [
        //get online info
        (true, Color::RGB(166,   227, 161),   Rect::new(20, 275, 200, 50)),
    ];



    //===================== texts =========================
    let all_text = vec!
    [
        //example text
        gen_text(20, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "Sync With Online".to_string(), default_text_color),
    ];



    //===================== page creation =========================
    Page 
    {
        background_color: Some(bg_color),
        rects:   None,
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use crate::sdl2_generators::gen_text;
use crate::math::{basic_data, maths, ONLINE_HISTORIC_RETURN_VALUE};
use crate::input_handler::{USER_INPUT_BUTTON_1, USER_INPUT_BUTTON_2};





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
    let bg_color = Color::RGB(30, 30, 46);
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
        gen_text(20, (all_buttons[1].2.x + 30, all_buttons[1].2.y + 20), "Second Page".to_string(), default_text_color),
        //selic page button text
        gen_text(20, (all_buttons[2].2.x + 30, all_buttons[2].2.y + 20), " Selic Page".to_string(), default_text_color),
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
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color      = Color::RGB(186, 194, 222);
    let ( one_year, one_month, one_day, one_hour, one_min, one_secs) = maths();
    let ( total_invested, year_return_value, _, _, _, _, _, _, _) = basic_data();
    
    //let ( total_invested, year_return_value, _, _, years_invested, months_invested, days_invested, hours_invested, minutes_invested) = basic_data();
    ////time invested text     
    //gen_text(25, (15, 225), format!("Time Invested: {} Yr, {} Mo, {} D, {} Hr, {} Min", years_invested, months_invested, days_invested, hours_invested, minutes_invested), default_text_color),

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
        (true, Color::RGB(0,   255, 0),   Rect::new(20, 105, 365, 50)),
        //receive input button 2
        (true, Color::RGB(0,   255, 0),   Rect::new(415, 105, 355, 50)),
    ];
    


    //===================== texts =========================
    let year_string: String   = format!("Return Per Year:   R$ {:.2}", one_year);
    let month_string: String  = format!("Return Per Month:  R$ {:.2}", one_month);
    let day_string: String    = format!("Return Per Day:    R$ {:.3}", one_day);
    let hour_string: String   = format!("Return Per Hour:   R$ {:.3}", one_hour);
    let minute_string: String = format!("Return Per Minute: R$ {:.4}", one_min);
    let second_string: String = format!("Return Per Second: R$ {:.4}", one_secs);
    
    let all_text = vec!
    [
        //total invested text
        gen_text(24, (425, 113), format!("Total Invested: R${}", total_invested    ), default_text_color),
        //year return value text
        gen_text(24, (30, 113), format!("Year Return Value: {}%", year_return_value), default_text_color),
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
        gen_text(24, (20, 100), unsafe{USER_INPUT_BUTTON_1.clone()}, default_text_color),
        gen_text(24, (350, 100), unsafe{USER_INPUT_BUTTON_2.clone()}, default_text_color),
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
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color = Color::RGB(186, 194, 222);


    //===================== rects =========================
    let all_rects = vec!
    [
        //header info background
        (Color::RGB(203, 166, 247), Rect::new(0, 105, 800, 50)),
    ];



    //===================== texts =========================
    let mut all_text = vec![gen_text(25, (125, 113), "Number          Date         Selic Tax".to_string(), default_text_color)];
    for (index, string) in unsafe{ONLINE_HISTORIC_RETURN_VALUE.iter().enumerate()}
    {
        let text = gen_text(20, (145, 180 + (40 * index as i32)),  string.to_string(), subtext_color);
        all_text.push(text);
    }



    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   Some( all_rects ),
        buttons: None,
        texts:   Some( all_text ),
        images:  None,
    }
}

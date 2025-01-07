use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use crate::investment_wallet::{ALL_INVESTMENTS, INVESTMENT_NAME, REALTIME_CURRENCY, REALTIME_RETURN_PER_SECOND, REALTIME_TOTAL_INVESTED, RETURN_PER_INVESTMENT, TOTAL_INVESTED_PER_INVESTMENT};
use crate::sdl2_generators::gen_text;
use crate::math::{calculator_maths, realtime_currency_maths, DAYS_INVESTED, HOURS_INVESTED, MINUTES_INVESTED, MONTHS_INVESTED, ONLINE_HISTORIC_RETURN_VALUE, RETURN_VALUE, SECS_INVESTED, TOTAL_INVESTED, YEARS_INVESTED};
use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_4, IS_ON_WRITE_MODE_ON_BUTTON_5, IS_ON_WRITE_MODE_ON_BUTTON_6, IS_ON_WRITE_MODE_ON_BUTTON_7, IS_ON_WRITE_MODE_ON_BUTTON_8, USER_INPUT_BUTTON_1, USER_INPUT_BUTTON_1_PAGE_3, USER_INPUT_BUTTON_2, USER_INPUT_BUTTON_2_PAGE_3, USER_INPUT_BUTTON_3, USER_INPUT_BUTTON_3_PAGE_3, USER_INPUT_BUTTON_4, USER_INPUT_BUTTON_5, USER_INPUT_BUTTON_6, USER_INPUT_BUTTON_7, USER_INPUT_BUTTON_8};


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
        gen_text(17, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 24), "    Calculator   ".to_string(), default_text_color),
        //second page button text
        gen_text(17, (all_buttons[1].2.x + 9, all_buttons[1].2.y + 24), "Real-time Currency".to_string(), default_text_color),
        //selic page button text
        gen_text(17, (all_buttons[2].2.x + 31, all_buttons[2].2.y + 24), "Selic Historic  ".to_string(), default_text_color),
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
pub fn calculator_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color =           Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color =      Color::RGB(186, 194, 222);
    let (one_year, one_month, one_day, one_hour, one_min, one_secs) = calculator_maths();
    
    unsafe 
    { 
        if USER_INPUT_BUTTON_1.is_empty() { USER_INPUT_BUTTON_1.push(' ') }; 
        if USER_INPUT_BUTTON_2.is_empty() { USER_INPUT_BUTTON_2.push(' ') }; 
        if USER_INPUT_BUTTON_3.is_empty() { USER_INPUT_BUTTON_3.push(' ') }; 
        if USER_INPUT_BUTTON_4.is_empty() { USER_INPUT_BUTTON_4.push(' ') }; 
        if USER_INPUT_BUTTON_5.is_empty() { USER_INPUT_BUTTON_5.push(' ') }; 
        if USER_INPUT_BUTTON_6.is_empty() { USER_INPUT_BUTTON_6.push(' ') }; 
        if USER_INPUT_BUTTON_7.is_empty() { USER_INPUT_BUTTON_7.push(' ') }; 
        if USER_INPUT_BUTTON_8.is_empty() { USER_INPUT_BUTTON_8.push(' ') }; 
    };


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 125, 385, 50)),
        //receive input button 2
        (true, Color::RGB(203,   166, 247),   Rect::new(405, 125, 385, 50)),

        //year button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 245, 40, 25)),
        //month button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 295, 40, 25)),
        //day button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 345, 40, 25)),
        //hour button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 395, 40, 25)),
        //minute button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 445, 40, 25)),
        //secs button
        (true, Color::RGB(250,   179, 135),   Rect::new(345, 495, 40, 25)),
    ];
    


    //===================== texts =========================
    let year_string =   format!("Years:   R${:.2}", one_year);
    let month_string =  format!("Months:  R${:.2}", one_month);
    let day_string =    format!("Days:    R${:.2}", one_day);
    let hour_string =   format!("Hours:   R${:.2}", one_hour);
    let minute_string = format!("Minutes: R${:.2}", one_min);
    let second_string = format!("Seconds: R${:.2}", one_secs);

    let return_year_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_3} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_3.clone()}) }
    else
    { format!("Return in {}", unsafe{YEARS_INVESTED.to_string()}) };
    
    let return_month_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_4} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_4.clone()}) }
    else
    { format!("Return in {}", unsafe{MONTHS_INVESTED.to_string()}) };

    let return_day_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_5} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_5.clone()}) }
    else
    { format!("Return in {}", unsafe{DAYS_INVESTED.to_string()}) };

    let return_hour_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_6} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_6.clone()}) }
    else
    { format!("Return in {}", unsafe{HOURS_INVESTED.to_string()}) };

    let return_minute_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_7} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_7.clone()}) }
    else
    { format!("Return in {}", unsafe{MINUTES_INVESTED.to_string()}) };

    let return_second_string = if unsafe{IS_ON_WRITE_MODE_ON_BUTTON_8} 
    { format!("Return in {}", unsafe{USER_INPUT_BUTTON_8.clone()}) }
    else
    { format!("Return in {}", unsafe{SECS_INVESTED.to_string()}) };

    let mut all_text = vec!
    [
        gen_text(20, (226, 245), return_year_string, subtext_color),
        gen_text(20, (226, 295), return_month_string, subtext_color),
        gen_text(20, (226, 345), return_day_string, subtext_color),
        gen_text(20, (226, 395), return_hour_string, subtext_color),
        gen_text(20, (226, 445), return_minute_string, subtext_color),
        gen_text(20, (226, 495), return_second_string, subtext_color),
        
        gen_text(20, (400, 245), year_string, subtext_color),
        gen_text(20, (400, 295), month_string, subtext_color),
        gen_text(20, (400, 345), day_string, subtext_color),
        gen_text(20, (400, 395), hour_string, subtext_color),
        gen_text(20, (400, 445), minute_string, subtext_color),
        gen_text(20, (400, 495), second_string, subtext_color),
        //user input text
        gen_text(23, (274, 133), unsafe{USER_INPUT_BUTTON_1.clone()}, default_text_color),
        gen_text(23, (653, 133), unsafe{USER_INPUT_BUTTON_2.clone()}, default_text_color),
    ];
    unsafe
    {
        //year return value text
        if IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(23, (20, 133), "Year Return Value: ".to_string(), default_text_color));
        }

        //year return value text
        if !IS_ON_WRITE_MODE_ON_BUTTON_1
        {
            all_text.push(gen_text(23, (20, 133), format!("Year Return Value: {}%", RETURN_VALUE), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(23, (415, 133), "Total Invested: R$".to_string(), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2
        {
            //total invested text
            all_text.push(gen_text(23, (415, 133), format!("Total Invested: R${}", TOTAL_INVESTED), default_text_color));
        }
    };
    

    //===================== page creation =========================
    Page 
    {
        background_color: Some(bg_color),
        rects:   None,
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





#[allow(static_mut_refs)]
pub fn realtime_currency_page() -> Page<'static>
{
    realtime_currency_maths();


    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color      = Color::RGB(186, 194, 222);
    
    //===================== rects =========================
    let all_rects = vec! 
    [
        //total invested background
        (Color::RGB(203, 166, 247), Rect::new(405, 125, 385, 50)),
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        //investment wallet
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 125, 385, 50)),
    ];


    //===================== texts =========================
    let realtime_currency_string: String = format!("Realtime Currency: R$ {:.7}", unsafe{REALTIME_CURRENCY});
    let second_string: String = format!("Return Per Second: R$ {:.13}", unsafe{REALTIME_RETURN_PER_SECOND.to_string()});
    
    let all_text = vec!
    [
        //realtime curreny text
        gen_text(30, (100, 300), realtime_currency_string, subtext_color),
        //one sec text
        gen_text(18, (225, 350), second_string, subtext_color),
        //total invested text
        gen_text(23, (415, 133), format!("Total Invested: R${}", unsafe{REALTIME_TOTAL_INVESTED}), default_text_color),
        //investment wallet button text
        gen_text(23, (75, 133), "Investment Wallet".to_string(), default_text_color),
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
pub fn selic_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color = Color::RGB(186, 194, 222);


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //sync online info button
        (true, Color::RGB(203, 166, 247), Rect::new(300, 125, 213, 50)),
    ];



    //===================== texts =========================
    let mut all_text = vec!
    [
        //sync online info button text
        gen_text(20, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "Sync With Online".to_string(), default_text_color),
        gen_text(25, (125, 205), "Number          Date         Selic Tax".to_string(), subtext_color),
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
        rects:   None,
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}



#[allow(static_mut_refs)]
pub fn investment_wallet_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    unsafe 
    {
        if USER_INPUT_BUTTON_1_PAGE_3.is_empty() { USER_INPUT_BUTTON_1_PAGE_3.push(' ') };
        if USER_INPUT_BUTTON_2_PAGE_3.is_empty() { USER_INPUT_BUTTON_2_PAGE_3.push(' ') };
        if USER_INPUT_BUTTON_3_PAGE_3.is_empty() { USER_INPUT_BUTTON_3_PAGE_3.push(' ') }; 
    };



    //===================== rects =========================
    let mut all_rects = Vec::new();
    for index in 0..(unsafe{ALL_INVESTMENTS.len()})
    {
        //investment background
        all_rects.push((Color::RGB(24, 24, 37), Rect::new(20, 325 + (index * 45) as i32, 730, 40)));
    }



    //===================== buttons =========================
    let all_buttons = vec!
    [
        //back button
        (true, Color::RGB(243, 139, 168), Rect::new(20, 20, 50, 50)),
        //investment year return value button
        (true, Color::RGB(203, 166, 247), Rect::new(20, 175, 350, 50)),
        //total invested return value button
        (true, Color::RGB(203, 166, 247), Rect::new(400, 175, 350, 50)),
        //investment name button
        (true, Color::RGB(203, 166, 247), Rect::new(20, 100, 730, 50)),
        //add investment button
        (true, Color::RGB(250, 179, 135), Rect::new(100, 20, 650, 50)),
        //delete all investment button 
        (true, Color::RGB(243, 139, 168), Rect::new(20, 250, 730, 50)),
    ];



    //===================== texts =========================
    let mut all_text = vec!
    [
        //example text
        gen_text(20, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "<-".to_string(), default_text_color),
        gen_text(20, (all_buttons[4].2.x + 245, all_buttons[4].2.y + 10),  "Add Investment".to_string(), default_text_color),
        gen_text(20, (all_buttons[4].2.x + 200, all_buttons[5].2.y + 10),  "Remove All Investments".to_string(), default_text_color),
    ];
    unsafe 
    {
        if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 10),  format!("Return Value: {}", USER_INPUT_BUTTON_1_PAGE_3.clone()), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 10),  format!("Total Invested: {}", USER_INPUT_BUTTON_2_PAGE_3.clone()), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 10),  format!("Investment Name: {}", USER_INPUT_BUTTON_3_PAGE_3.clone()), default_text_color));
        }




        if !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 10),  format!("Return Value: {}%", RETURN_PER_INVESTMENT), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 10),  format!("Total Invested: R${}", TOTAL_INVESTED_PER_INVESTMENT), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 10),  format!("Investment Name: {}", INVESTMENT_NAME.clone()), default_text_color));
        }

        for (index, investment) in ALL_INVESTMENTS.clone().into_iter().enumerate()
        {
            let year_return_value = investment.1;
            let total_invested = investment.2;
            let name = investment.3;
    
            all_text.push(gen_text(18, (30, 332 + (index * 45) as i32),  format!("Name:{}   Rate:{}%   Total Invested: R${}", name, year_return_value, total_invested), default_text_color));
        };
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

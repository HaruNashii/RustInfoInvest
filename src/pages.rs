use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use crate::investment_wallet::{INVESTMENT_NAME, TOTAL_INVESTED_PER_INVESTMENT, YEAR_RETURN_VALUE_PER_INVESTMENT};
use crate::sdl2_generators::gen_text;
use crate::math::{calculator_maths, realtime_currency_maths, ONLINE_HISTORIC_RETURN_VALUE, REALTIME_CURRENCY, REALTIME_SECS, RETURN_VALUE, RETURN_VALUE_REALTIME_PAGE, TOTAL_INVESTED, TOTAL_INVESTED_REALTIME_PAGE};
use crate::input_handler::{IS_ON_WRITE_MODE_ON_BUTTON_1, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, USER_INPUT_BUTTON_1, USER_INPUT_BUTTON_1_PAGE_2, USER_INPUT_BUTTON_1_PAGE_3, USER_INPUT_BUTTON_2, USER_INPUT_BUTTON_2_PAGE_2, USER_INPUT_BUTTON_2_PAGE_3, USER_INPUT_BUTTON_3_PAGE_3};


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
pub fn main_page() -> Page<'static>
{
    //===================== variables =========================
    let bg_color =           Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color =      Color::RGB(186, 194, 222);
    let ( one_year, one_month, one_day, one_hour, one_min, one_secs) = calculator_maths();
    
    unsafe 
    { 
        if USER_INPUT_BUTTON_1.is_empty() { USER_INPUT_BUTTON_1.push(' ') }; 
        if USER_INPUT_BUTTON_2.is_empty() { USER_INPUT_BUTTON_2.push(' ') }; 
    };


    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 125, 385, 50)),
        //receive input button 2
        (true, Color::RGB(203,   166, 247),   Rect::new(405, 125, 385, 50)),
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

    unsafe
    {
        if USER_INPUT_BUTTON_1_PAGE_2.is_empty() { USER_INPUT_BUTTON_1_PAGE_2.push(' ') };
        if USER_INPUT_BUTTON_2_PAGE_2.is_empty() { USER_INPUT_BUTTON_2_PAGE_2.push(' ') };
    };
    

    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 125, 385, 50)),
        //receive input button 2
        (true, Color::RGB(203,   166, 247),   Rect::new(405, 125, 385, 50)),
        //investment wallet
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 200, 385, 50)),
    ];


    //===================== texts =========================
    let realtime_currency_string: String = format!("Realtime Currency: R$ {:.7}", unsafe{REALTIME_CURRENCY});
    let second_string: String = format!("Return Per Second: R$ {:.10}", unsafe{REALTIME_SECS});
    
    let mut all_text = vec!
    [
        //realtime curreny text
        gen_text(30, (100, 300), realtime_currency_string, subtext_color),
        //one sec text
        gen_text(18, (225, 350), second_string, subtext_color),
        //user input text
        gen_text(23, (274, 133), unsafe{USER_INPUT_BUTTON_1_PAGE_2.clone()}, default_text_color),
        gen_text(23, (653, 133), unsafe{USER_INPUT_BUTTON_2_PAGE_2.clone()}, default_text_color), 
    ];
    unsafe
    {
        //year return value text
        if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2
        {
            all_text.push(gen_text(23, (20, 133), "Year Return Value: ".to_string(), default_text_color));
        }

        //year return value text
        if !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_2
        {
            all_text.push(gen_text(23, (20, 133), format!("Year Return Value: {}%", RETURN_VALUE_REALTIME_PAGE), default_text_color));
        }

        if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
        {
            //total invested text
            all_text.push(gen_text(23, (415, 133), "Total Invested: R$".to_string(), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_2
        {
            //total invested text
            all_text.push(gen_text(23, (415, 133), format!("Total Invested: R${}", TOTAL_INVESTED_REALTIME_PAGE), default_text_color));
        }
        all_text.push(gen_text(23, (20, 200), "Investment Wallet".to_string(), default_text_color));
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
    ];



    //===================== texts =========================
    let mut all_text = vec!
    [
        //example text
        gen_text(20, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "<-".to_string(), default_text_color),
        gen_text(20, (all_buttons[4].2.x + 10, all_buttons[4].2.y + 10),  "Add Investment".to_string(), default_text_color),
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
            all_text.push(gen_text(20, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 10),  format!("Return Value: {}", YEAR_RETURN_VALUE_PER_INVESTMENT), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 10),  format!("Total Invested: {}", TOTAL_INVESTED_PER_INVESTMENT), default_text_color));
        }

        if !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3
        {
            all_text.push(gen_text(20, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 10),  format!("Investment Name: {}", INVESTMENT_NAME.clone()), default_text_color));
        }
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

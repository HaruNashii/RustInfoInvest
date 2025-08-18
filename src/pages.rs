use std::process::Command;
use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::render::Texture;
use crate::
{ 
    input_handler::{IS_ON_WRITE_MODE, IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3, IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3, USER_INPUT, USER_INPUT_BUTTON_1_PAGE_3, USER_INPUT_BUTTON_2_PAGE_3, USER_INPUT_BUTTON_3_PAGE_3, USER_INPUT_BUTTON_4_PAGE_3, USER_INPUT_BUTTON_5_PAGE_3, USER_INPUT_BUTTON_6_PAGE_3}, 
    investment_wallet::{ALL_INVESTMENTS, DAY, INVESTMENT_NAME, MONTH, REALTIME_CURRENCY, REALTIME_RETURN_PER_SECOND, REALTIME_TOTAL_INVESTED, RETURN_PER_INVESTMENT, TOTAL_INVESTED_PER_INVESTMENT, YEAR}, 
    math::{calculator_maths, realtime_currency_maths, DAYS_INVESTED, HOURS_INVESTED, MINUTES_INVESTED, MONTHLY_CONTRIBUTION, MONTHS_INVESTED, ONLINE_HISTORIC_RETURN_VALUE, RETURN_VALUE, SECS_INVESTED, TOTAL_INVESTED, YEARS_INVESTED}, 
    sdl3_generators::gen_text
};





pub struct Page<'a>
{
    pub background_color: Option<Color>,
    pub rects:   Option<Vec< (Color, Rect) >>,
    pub buttons: Option<Vec< (bool, Color, Rect, u16) >>,
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
        (true, Color::RGB(243, 139, 168), Rect::new(75, 10, 200, 75), 1),
        //secound page button
        (true, Color::RGB(243, 139, 168), Rect::new(300, 10, 200, 75), 2),
        //selic page button
        (true, Color::RGB(243, 139, 168), Rect::new(525, 10, 200, 75), 3),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //main page button text
        gen_text(17.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 24), "    Calculator   ".to_string(), default_text_color),
        //second page button text
        gen_text(17.0, (all_buttons[1].2.x + 9, all_buttons[1].2.y + 24), "Real-time Currency".to_string(), default_text_color),
        //selic page button text
        gen_text(17.0, (all_buttons[2].2.x + 31, all_buttons[2].2.y + 24), "Selic Historic  ".to_string(), default_text_color),
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
    
    //===================== rects =========================
    let all_rects = vec! 
    [
        //total invested + income background
        (Color::RGB(250, 179, 135), Rect::new(10, 215, 385, 40)),
        (Color::RGB(250, 179, 135), Rect::new(405, 215, 385, 40)),
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        //receive input button 1
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 105, 385, 40), 4),
        //receive input button 2
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 160, 780, 40), 5),
        //year button
        (true, Color::RGB(17,   17, 27),   Rect::new(345, 275, 40, 25), 6),
        //month button
        (true, Color::RGB(17, 17, 27),   Rect::new(345, 325, 40, 25), 7),
        //day button
        (true, Color::RGB(17, 17, 27),   Rect::new(345, 375, 40, 25), 8),
        //hour button
        (true, Color::RGB(17, 17, 27),   Rect::new(345, 425, 40, 25), 9),
        //minute button
        (true, Color::RGB(17, 17, 27),   Rect::new(345, 475, 40, 25), 10),
        //secs button
        (true, Color::RGB(17, 17, 27),   Rect::new(345, 525, 40, 25), 11),
        //receive input button 3
        (true, Color::RGB(203,   166, 247),   Rect::new(405, 105, 385, 40), 12),
    ];
    
    //===================== texts =========================
    let year_string =   format!("Years:   R${:.2}", one_year);
    let month_string =  format!("Months:  R${:.2}", one_month);
    let day_string =    format!("Days:    R${:.2}", one_day);
    let hour_string =   format!("Hours:   R${:.4}", one_hour);
    let minute_string = format!("Minutes: R${:.5}", one_min);
    let second_string = format!("Seconds: R${:.6}", one_secs);

    
    
    
    
    
    


    let total_with_year_income_string = format!("Total in years Income: R${:.2}", unsafe{TOTAL_INVESTED + one_year});
    let total_with_month_income_string = format!("Total in months Income: R${:.2}", unsafe{TOTAL_INVESTED + one_month});

    let mut all_text = vec!
    [
        gen_text(16.0, (20, 224), total_with_year_income_string, default_text_color),
        gen_text(16.0, (415, 224), total_with_month_income_string, default_text_color),
        gen_text(20.0, (400, 275), year_string, subtext_color),
        gen_text(20.0, (400, 325), month_string, subtext_color),
        gen_text(20.0, (400, 375), day_string, subtext_color),
        gen_text(20.0, (400, 425), hour_string, subtext_color),
        gen_text(20.0, (400, 475), minute_string, subtext_color),
        gen_text(20.0, (400, 525), second_string, subtext_color),
    ];
    unsafe
    {
        //year return value text
        if IS_ON_WRITE_MODE.0 
        {
            match IS_ON_WRITE_MODE.1
            {
                Some(4) =>  { all_text.push(gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), format!("Year Return Value: {}%", USER_INPUT), default_text_color)); },
                Some(5) =>  { all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Total Invested: R${}",   USER_INPUT), default_text_color)); },
                Some(6) =>  { all_text.push(gen_text(20.0, (226, 275), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(7) =>  { all_text.push(gen_text(20.0, (226, 325), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(8) =>  { all_text.push(gen_text(20.0, (226, 375), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(9) =>  { all_text.push(gen_text(20.0, (226, 425), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(10) => { all_text.push(gen_text(20.0, (226, 475), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(11) => { all_text.push(gen_text(20.0, (226, 525), format!("Return in {}", USER_INPUT), subtext_color)); },
                Some(12) => { all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Monthly Contribution: R${}", USER_INPUT), default_text_color)); },
                _=> {},
            };
        }
        else 
        { 
            all_text.push(gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), format!("Year Return Value: {}%", RETURN_VALUE), default_text_color)); 
            all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Total Invested: R${}", TOTAL_INVESTED), default_text_color));
            all_text.push(gen_text(20.0, (226, 275), format!("Return in {}", MONTHS_INVESTED.to_string()  ), subtext_color) );
            all_text.push(gen_text(20.0, (226, 325), format!("Return in {}", YEARS_INVESTED.to_string()   ), subtext_color) );
            all_text.push(gen_text(20.0, (226, 375), format!("Return in {}", DAYS_INVESTED.to_string()    ), subtext_color) );
            all_text.push(gen_text(20.0, (226, 425), format!("Return in {}", HOURS_INVESTED.to_string()   ), subtext_color) );
            all_text.push(gen_text(20.0, (226, 475), format!("Return in {}", MINUTES_INVESTED.to_string() ), subtext_color) );
            all_text.push(gen_text(20.0, (226, 525), format!("Return in {}", SECS_INVESTED.to_string()    ), subtext_color) );
            all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Monthly Contribution: R${}", MONTHLY_CONTRIBUTION), default_text_color));
        };

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
    realtime_currency_maths();

    //===================== variables =========================
    let bg_color = Color::RGB(30, 30, 46);
    let default_text_color = Color::RGB(255, 255, 255);
    let subtext_color      = Color::RGB(186, 194, 222);
    
    //===================== rects =========================
    let all_rects = vec! 
    [
        //total invested background
        (Color::RGB(203, 166, 247), Rect::new(405, 105, 385, 40)),
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        //investment wallet
        (true, Color::RGB(203,   166, 247),   Rect::new(10, 105, 385, 40), 13),
    ];

    //===================== texts =========================
    let realtime_currency_string: String = format!("Realtime Currency: R$ {:.7}", unsafe{REALTIME_CURRENCY});
    let second_string: String = format!("Return Per Days: R$ {:.5}", unsafe{REALTIME_RETURN_PER_SECOND.to_string()});
    let all_text = vec!
    [
        //realtime curreny text
        gen_text(30.0, (100, 300), realtime_currency_string, subtext_color),
        //one sec text
        gen_text(18.0, (225, 350), second_string, subtext_color),
        //total invested text
        gen_text(18.0, (all_rects[0].1.x + 10, all_rects[0].1.y + 7), format!("Total Invested: R${}", unsafe{REALTIME_TOTAL_INVESTED}), default_text_color),
        //investment wallet button text
        gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), "Investment Wallet".to_string(), default_text_color),
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

    //===================== check if geckodriver is available ========================= 
    let geckodriver_available: bool = Command::new("which").arg("geckodriver").output().map(|o| o.status.success()).unwrap_or(false);
    let firefox_available: bool = Command::new("which").arg("firefox").output().map(|o| o.status.success()).unwrap_or(false);


    //===================== buttons =========================
    let mut all_buttons = Vec::new();
    if geckodriver_available && firefox_available
    {
        all_buttons.push((true, Color::RGB(203, 166, 247), Rect::new(300, 125, 213, 50), 14));
    };


    //===================== texts =========================
    let mut all_text = Vec::new();
    if geckodriver_available && firefox_available
    {
        //sync online info button text
        all_text.push(gen_text(20.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "Sync With Online".to_string(), default_text_color));
        all_text.push(gen_text(25.0, (125, 205), "Number          Date         Selic Tax".to_string(), subtext_color));
    };
    
    if !geckodriver_available
    {
        all_text.push(gen_text(26.0, (40, 275), "Geckodriver Not Available, Please Install It.".to_string(), default_text_color));
    };

    if !firefox_available
    {
        all_text.push(gen_text(26.0, (75, 325), "Firefox Not Available, Please Install It.".to_string(), default_text_color));
    };


    for (index, string) in unsafe{ONLINE_HISTORIC_RETURN_VALUE.iter().enumerate()}
    {
        let text = gen_text(20.0, (145, 300 + (40 * index as i32)),  string.to_string(), subtext_color);
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

    //===================== rects =========================
    let mut all_rects = Vec::new();
    for index in 0..(unsafe{ALL_INVESTMENTS.len()})
    {
        //investment background
        all_rects.push((Color::RGB(24, 24, 37), Rect::new(20, 320 + (index * 40) as i32, 730, 30)));
    }
    
    //===================== buttons =========================
    let mut all_buttons = vec!
    [
        //back button
        (true, Color::RGB(243, 139, 168), Rect::new(20, 20, 50, 40), 15),
        //investment year return value button
        (true, Color::RGB(203, 166, 247), Rect::new(20, 200, 350, 40), 16),
        //total invested return value button
        (true, Color::RGB(203, 166, 247), Rect::new(400, 200, 350, 40), 17),
        //investment name button
        (true, Color::RGB(203, 166, 247), Rect::new(20, 140, 730, 40), 18),
        //add investment button
        (true, Color::RGB(250, 179, 135), Rect::new(100, 20, 650, 40), 19),
        //delete all investment button 
        (true, Color::RGB(243, 139, 168), Rect::new(20, 80, 730, 40), 20),

        //year button 
        (true, Color::RGB(243, 139, 168), Rect::new(20, 260, 230, 40), 21),
        //month button 
        (true, Color::RGB(243, 139, 168), Rect::new(270, 260, 230, 40), 22),
        //day button 
        (true, Color::RGB(243, 139, 168), Rect::new(520, 260, 230, 40), 23),
    ];

    for (index, rect) in all_rects.iter().enumerate()
    {
        all_buttons.push( (true, Color::RGB(243, 139, 168), Rect::new(680, rect.1.y, 70, rect.1.h as u32), (index + 1000) as u16) );
    }

    //===================== texts =========================
    let mut all_text = vec!
    [
        //example text
        gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7),  "<-".to_string(), default_text_color),
        gen_text(18.0, (all_buttons[4].2.x + 245, all_buttons[4].2.y + 7),  "Add Investment".to_string(), default_text_color),
        gen_text(18.0, (all_buttons[4].2.x + 200, all_buttons[5].2.y + 7),  "Remove All Investments".to_string(), default_text_color),
    ];
    unsafe 
    {
        if IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7),  format!("Return Value: {}", USER_INPUT_BUTTON_1_PAGE_3.clone()), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_1_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7),  format!("Return Value: {}%", RETURN_PER_INVESTMENT), default_text_color)); }
        if IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 7),  format!("Total Invested: {}", USER_INPUT_BUTTON_2_PAGE_3.clone()), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_2_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 7),  format!("Total Invested: R${}", TOTAL_INVESTED_PER_INVESTMENT), default_text_color)); }
        if IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 7),  format!("Investment Name: {}", USER_INPUT_BUTTON_3_PAGE_3.clone()), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_3_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 7),  format!("Investment Name: {}", INVESTMENT_NAME.clone()), default_text_color)); }
        if IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[6].2.x + 10, all_buttons[6].2.y + 7),  format!("Year: {}", USER_INPUT_BUTTON_4_PAGE_3), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_4_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[6].2.x + 10, all_buttons[6].2.y + 7),  format!("Year: {}", YEAR), default_text_color)); }
        if IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[7].2.x + 10, all_buttons[7].2.y + 7),  format!("Month: {}", USER_INPUT_BUTTON_5_PAGE_3), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_5_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[7].2.x + 10, all_buttons[7].2.y + 7),  format!("Month: {}", MONTH), default_text_color)); }
        if IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3  { all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7),  format!("Day: {}", USER_INPUT_BUTTON_6_PAGE_3), default_text_color)); }
        if !IS_ON_WRITE_MODE_ON_BUTTON_6_PAGE_3 { all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7),  format!("Day {}", DAY), default_text_color)); }
        //investment summary text
        for (index, investment) in ALL_INVESTMENTS.clone().into_iter().enumerate()
        {
            let date = investment.0.format("%Y-%m-%d");
            let year_return_value = investment.1;
            let total_invested = investment.2;
            let name = investment.3;
            all_text.push(gen_text(10.0, (30, 327 + (index * 40) as i32),  format!("Name:{}   Rate:{}%   Total Invested: R${}   Date: {}", name, year_return_value, total_invested, date), default_text_color));
        };

        for rect in &all_rects
        {
            all_text.push( gen_text(14.0, (690, rect.1.y + 5), "Remove".to_string(), default_text_color) );
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

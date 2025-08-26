use std::process::Command;
use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::render::Texture;
use crate::
{ 
    input_handler::{IS_ON_WRITE_MODE, USER_INPUT}, 
    investment_wallet::{ALL_INVESTMENTS, DAY, INVESTMENT_NAME, MONTH, REALTIME_CURRENCY, REALTIME_RETURN_PER_SECOND, REALTIME_TOTAL_INVESTED, RETURN_PER_INVESTMENT, TOTAL_INVESTED_PER_INVESTMENT, YEAR}, 
    math::{calculator_maths, realtime_currency_maths, DAYS_INVESTED, HOURS_INVESTED, MINUTES_INVESTED, MONTHLY_CONTRIBUTION, MONTHS_INVESTED, ONLINE_HISTORIC_RETURN_VALUE, RETURN_VALUE, SECS_INVESTED, TOTAL_INVESTED, YEARS_INVESTED}, 
    sdl3_generators::gen_text,
    buttons::button_change_color_when_hovered,
};





pub struct Page<'a>
{
    pub background_color: Option<Color>,
    pub rects:   Option<Vec< (Color, Rect) >>,
    pub buttons: Option<Vec< (bool, Color, Rect, u16) >>,
    pub texts:   Option<Vec< (Texture<'a>, Rect) >>,
    pub images:  Option<Vec< (Texture<'a>, Rect) >>,
}




pub const COLOR_CHANGE_WHEN_SELECTED: (u8, u8, u8) = (25, 25, 25);
const BACKGROUND_COLOR: Color = Color::RGB(30,  30,  46);
const TEXT_COLOR:       Color = Color::RGB(255, 255, 255);
const SUBTEXT_COLOR:    Color = Color::RGB(186, 194, 222);
const PURPLE_COLOR:     Color = Color::RGB(203, 166, 247);
const PINK_COLOR:       Color = Color::RGB(243, 139, 168);
const ORANGE_COLOR:     Color = Color::RGB(250, 179, 135);
const BLACK_COLOR:      Color = Color::RGB(17,  17,  27);





pub fn persistent_page() -> Page<'static>
{
    //===================== rects =========================
    let all_rects = vec!
    [
        //header background
        (BLACK_COLOR, Rect::new(0, 0, 1920, 100)),
    ];



    //===================== buttons =========================
    let mut all_buttons = vec!
    [
        //main page button
        (true, PINK_COLOR, Rect::new(550, 10, 200, 75), 1),
        //secound page button
        (true, PINK_COLOR, Rect::new(850, 10, 200, 75), 2),
        //selic page button
        (true, PINK_COLOR, Rect::new(1149, 10, 200, 75), 3),
    ];



    //===================== make buttons change color when selected =========================
    button_change_color_when_hovered(&mut all_buttons);



    //===================== texts =========================
    let all_text = vec!
    [
        //main page button text
        gen_text(17.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 24), "    Calculator   ".to_string(), TEXT_COLOR),
        //second page button text
        gen_text(17.0, (all_buttons[1].2.x + 9, all_buttons[1].2.y + 24), "Real-time Currency".to_string(), TEXT_COLOR),
        //selic page button text
        gen_text(17.0, (all_buttons[2].2.x + 31, all_buttons[2].2.y + 24), "Selic Historic  ".to_string(), TEXT_COLOR),
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
    let (one_year, one_month, one_day, one_hour, one_min, one_secs) = calculator_maths();



    //===================== rects =========================
    let all_rects = vec! 
    [
        //total invested + income background
        (ORANGE_COLOR, Rect::new(10, 215, 940, 40)),
        (ORANGE_COLOR, Rect::new(965, 215, 940, 40)),
    ];



    //===================== buttons =========================
    let mut all_buttons: Vec<(bool, Color, Rect, u16)> = vec!
    [
        //receive input button 1
        (true, PURPLE_COLOR, Rect::new(10, 105, 940, 40), 4),
        //receive input button 2
        (true, PURPLE_COLOR, Rect::new(10, 160, 1895, 40), 5),
        //year button
        (true, BLACK_COLOR,  Rect::new(935, 405, 45, 25), 6),
        //month button
        (true, BLACK_COLOR,  Rect::new(935, 505, 45, 25), 7),
        //day button
        (true, BLACK_COLOR,  Rect::new(935, 605, 45, 25), 8),
        //hour button
        (true, BLACK_COLOR,  Rect::new(935, 705, 45, 25), 9),
        //minute button
        (true, BLACK_COLOR,  Rect::new(935, 805, 45, 25), 10),
        //secs button
        (true, BLACK_COLOR,  Rect::new(935, 905, 45, 25), 11),
        //receive input button 3
        (true, PURPLE_COLOR, Rect::new(965, 105, 940, 40), 12),
    ];



   //===================== make buttons change color when selected =========================
   button_change_color_when_hovered(&mut all_buttons);



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
        gen_text(16.0, (20, 224),  total_with_year_income_string, TEXT_COLOR),
        gen_text(16.0, (415, 224), total_with_month_income_string, TEXT_COLOR),
        gen_text(25.0, (all_buttons[2].2.x + 55, 400), year_string, SUBTEXT_COLOR),
        gen_text(25.0, (all_buttons[3].2.x + 55, 500), month_string, SUBTEXT_COLOR),
        gen_text(25.0, (all_buttons[4].2.x + 55, 600), day_string, SUBTEXT_COLOR),
        gen_text(25.0, (all_buttons[5].2.x + 55, 700), hour_string, SUBTEXT_COLOR),
        gen_text(25.0, (all_buttons[6].2.x + 55, 800), minute_string, SUBTEXT_COLOR),
        gen_text(25.0, (all_buttons[7].2.x + 55, 900), second_string, SUBTEXT_COLOR),
    ];

    unsafe
    {
        all_text.push(gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), format!("Year Return Value: {}%", RETURN_VALUE), TEXT_COLOR)); 
        all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Total Invested: R${}", TOTAL_INVESTED), TEXT_COLOR));
        all_text.push(gen_text(25.0, (all_buttons[2].2.x - 150, 400), format!("Return in {}", YEARS_INVESTED   ), SUBTEXT_COLOR) );
        all_text.push(gen_text(25.0, (all_buttons[3].2.x - 150, 500), format!("Return in {}", MONTHS_INVESTED  ), SUBTEXT_COLOR) );
        all_text.push(gen_text(25.0, (all_buttons[4].2.x - 150, 600), format!("Return in {}", DAYS_INVESTED    ), SUBTEXT_COLOR) );
        all_text.push(gen_text(25.0, (all_buttons[5].2.x - 150, 700), format!("Return in {}", HOURS_INVESTED   ), SUBTEXT_COLOR) );
        all_text.push(gen_text(25.0, (all_buttons[6].2.x - 150, 800), format!("Return in {}", MINUTES_INVESTED ), SUBTEXT_COLOR) );
        all_text.push(gen_text(25.0, (all_buttons[7].2.x - 150, 900), format!("Return in {}", SECS_INVESTED    ), SUBTEXT_COLOR) );
        all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Monthly Contribution: R${}", MONTHLY_CONTRIBUTION), TEXT_COLOR));

        match IS_ON_WRITE_MODE.1
        {
            Some(4) =>  { all_text.remove(4 + 4);  all_text.push(gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), format!("Year Return Value: {}%", USER_INPUT), TEXT_COLOR)); },
            Some(5) =>  { all_text.remove(5 + 4);  all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Total Invested: R${}",   USER_INPUT), TEXT_COLOR)); },
            Some(6) =>  { all_text.remove(6 + 4);  all_text.push(gen_text(25.0, (all_buttons[2].2.x - 150, 400), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(7) =>  { all_text.remove(7 + 4);  all_text.push(gen_text(25.0, (all_buttons[3].2.x - 150, 500), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(8) =>  { all_text.remove(8 + 4);  all_text.push(gen_text(25.0, (all_buttons[4].2.x - 150, 600), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(9) =>  { all_text.remove(9 + 4);  all_text.push(gen_text(25.0, (all_buttons[5].2.x - 150, 700), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(11) => { all_text.remove(11 + 4); all_text.push(gen_text(25.0, (all_buttons[6].2.x - 150, 800), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(10) => { all_text.remove(10 + 4); all_text.push(gen_text(25.0, (all_buttons[7].2.x - 150, 900), format!("Return in {}", USER_INPUT), SUBTEXT_COLOR)); },
            Some(12) => { all_text.remove(12 + 4); all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Monthly Contribution: R${}", USER_INPUT), TEXT_COLOR)); },
            _=> { },
        };
    };



    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





#[allow(static_mut_refs)]
pub fn realtime_currency_page() -> Page<'static>
{
    //===================== update variables =========================
    realtime_currency_maths();



    //===================== rects =========================
    let all_rects = vec! 
    [
        //total invested background
        (PURPLE_COLOR, Rect::new(965, 105, 940, 40)),
    ];



    //===================== buttons =========================
    let mut all_buttons = vec!
    [
        //investment wallet
        (true, PURPLE_COLOR,   Rect::new(10, 105, 940, 40), 13),
    ];



    //===================== make buttons change color when selected =========================
    button_change_color_when_hovered(&mut all_buttons);



    //===================== texts =========================
    let realtime_currency_string: String = format!("Realtime Currency: R$ {:.7}", unsafe{REALTIME_CURRENCY});
    let second_string: String = format!("Return Per Days: R$ {:.5}", unsafe{REALTIME_RETURN_PER_SECOND.to_string()});
    let all_text = vec!
    [
        //realtime curreny text
        gen_text(30.0, (650, 500), realtime_currency_string, SUBTEXT_COLOR),
        //one sec text
        gen_text(18.0, (825, 550), second_string, SUBTEXT_COLOR),
        //total invested text
        gen_text(18.0, (all_rects[0].1.x + 10, all_rects[0].1.y + 7), format!("Total Invested: R${}", unsafe{REALTIME_TOTAL_INVESTED}), TEXT_COLOR),
        //investment wallet button text
        gen_text(18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), "Investment Wallet".to_string(), TEXT_COLOR),
    ];



    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





#[allow(static_mut_refs)]
pub fn selic_page() -> Page<'static>
{
    //===================== check if geckodriver is available ========================= 
    let geckodriver_available: bool = Command::new("which").arg("geckodriver").output().map(|o| o.status.success()).unwrap_or(false);
    let firefox_available: bool = Command::new("which").arg("firefox").output().map(|o| o.status.success()).unwrap_or(false);



    //===================== buttons =========================
    let mut all_buttons = Vec::new();
    if geckodriver_available && firefox_available
    {
        // sync button
        all_buttons.push((true, PURPLE_COLOR, Rect::new(843, 125, 213, 50), 14));
    };



    //===================== make buttons change color when selected =========================
    button_change_color_when_hovered(&mut all_buttons);



    //===================== texts =========================
    let mut all_text = Vec::new();
    if geckodriver_available && firefox_available
    {
        //sync online info button text
        all_text.push(gen_text(20.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 10),  "Sync With Online".to_string(), TEXT_COLOR));
        all_text.push(gen_text(27.0, (650, 205), "Number          Date         Selic Tax".to_string(), SUBTEXT_COLOR));
    };
    
    if !geckodriver_available
    {
        all_text.push(gen_text(26.0, (40, 275), "Geckodriver Not Available, Please Install It.".to_string(), TEXT_COLOR));
    };

    if !firefox_available
    {
        all_text.push(gen_text(26.0, (75, 325), "Firefox Not Available, Please Install It.".to_string(), TEXT_COLOR));
    };

    for (index, string) in unsafe{ONLINE_HISTORIC_RETURN_VALUE.iter().enumerate()}
    {
        let text = gen_text(25.0, (645, 350 + (50 * index as i32)),  string.to_string(), SUBTEXT_COLOR);
        all_text.push(text);
    }



    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   None,
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}





#[allow(static_mut_refs)]
pub fn investment_wallet_page() -> Page<'static>
{
    //===================== rects =========================
    let mut all_rects = Vec::new();
    for index in 0..(unsafe{ALL_INVESTMENTS.len()})
    {
        // investment background
        all_rects.push((BLACK_COLOR, Rect::new(20, 320 + (index * 40) as i32, 1875, 30)));
    }


        //(true, PURPLE_COLOR, Rect::new(10, 105, 940, 40), 4),

    //===================== buttons =========================
    let mut all_buttons = vec!
    [
        //back button
        (true, PINK_COLOR, Rect::new(20, 20, 50, 40), 15),
        //investment year return value button
        (true, PURPLE_COLOR, Rect::new(20, 200, 930, 40), 16),
        //total invested return value button
        (true, PURPLE_COLOR, Rect::new(965, 200, 930, 40), 17),
        //investment name button
        (true, PURPLE_COLOR, Rect::new(20, 140, 1875, 40), 18),
        //add investment button
        (true, ORANGE_COLOR, Rect::new(100, 20, 1795, 40), 19),
        //delete all investment button 
        (true, PINK_COLOR, Rect::new(20, 80, 1875, 40), 20),
        //year button 
        (true, PINK_COLOR, Rect::new(20, 260, 610, 40), 21),
        //month button 
        (true, PINK_COLOR, Rect::new(652, 260, 610, 40), 22),
        //day button 
        (true, PINK_COLOR, Rect::new(1285, 260, 610, 40), 23),
    ];

    for (index, rect) in all_rects.iter().enumerate()
    {
        // remove button
        all_buttons.push( (true, PINK_COLOR, Rect::new(1790, rect.1.y, 105, rect.1.h as u32), (index + 1000) as u16) );
    }
    


    //===================== make buttons change color when selected =========================
    button_change_color_when_hovered(&mut all_buttons);



    //===================== texts =========================
    let mut all_text = vec!
    [
        gen_text(18.0, (all_buttons[0].2.x + 10,  all_buttons[0].2.y + 7),  "<-".to_string(), TEXT_COLOR),
        gen_text(18.0, (all_buttons[4].2.x + 800, all_buttons[4].2.y + 7),  "Add Investment".to_string(), TEXT_COLOR),
        gen_text(18.0, (all_buttons[4].2.x + 745, all_buttons[5].2.y + 7),  "Remove All Investments".to_string(), TEXT_COLOR),
    ];

    unsafe 
    {
        all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Return Value: {}%",    RETURN_PER_INVESTMENT),         TEXT_COLOR));
        all_text.push(gen_text(18.0, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 7), format!("Total Invested: R${}", TOTAL_INVESTED_PER_INVESTMENT), TEXT_COLOR));
        all_text.push(gen_text(18.0, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 7), format!("Investment Name: {}",  INVESTMENT_NAME),               TEXT_COLOR));
        all_text.push(gen_text(18.0, (all_buttons[6].2.x + 10, all_buttons[6].2.y + 7), format!("Year: {}",             YEAR),                          TEXT_COLOR));
        all_text.push(gen_text(18.0, (all_buttons[7].2.x + 10, all_buttons[7].2.y + 7), format!("Month: {}",            MONTH),                         TEXT_COLOR));
        all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Day {}",               DAY),                           TEXT_COLOR));

        match IS_ON_WRITE_MODE.1
        {
            Some(16) => { all_text.remove(3); all_text.push(gen_text(18.0, (all_buttons[1].2.x + 10, all_buttons[1].2.y + 7), format!("Return Value: {}",    USER_INPUT), TEXT_COLOR)); },
            Some(17) => { all_text.remove(4); all_text.push(gen_text(18.0, (all_buttons[2].2.x + 10, all_buttons[2].2.y + 7), format!("Total Invested: {}",  USER_INPUT), TEXT_COLOR)); },
            Some(18) => { all_text.remove(5); all_text.push(gen_text(18.0, (all_buttons[3].2.x + 10, all_buttons[3].2.y + 7), format!("Investment Name: {}", USER_INPUT), TEXT_COLOR)); },
            Some(21) => { all_text.remove(6); all_text.push(gen_text(18.0, (all_buttons[6].2.x + 10, all_buttons[6].2.y + 7), format!("Year: {}",            USER_INPUT), TEXT_COLOR)); },
            Some(22) => { all_text.remove(7); all_text.push(gen_text(18.0, (all_buttons[7].2.x + 10, all_buttons[7].2.y + 7), format!("Month: {}",           USER_INPUT), TEXT_COLOR)); },
            Some(23) => { all_text.remove(8); all_text.push(gen_text(18.0, (all_buttons[8].2.x + 10, all_buttons[8].2.y + 7), format!("Day: {}",             USER_INPUT), TEXT_COLOR)); },
            _=> {},
        };

        //investment summary text
        for (index, investment) in ALL_INVESTMENTS.clone().into_iter().enumerate()
        {
            all_text.push(gen_text(10.0, (30, 327 + (index * 40) as i32),  format!("Name:{}                                                              Rate:{}%                                                              Total Invested: R${}                                                              Date: {}", investment.3, investment.1, investment.2, investment.0.format("%Y-%m-%d")), TEXT_COLOR));
        };
    }

    for rect in &all_rects
    {
        all_text.push( gen_text(14.0, (1820, rect.1.y + 5), "Remove".to_string(), TEXT_COLOR) );
    };



    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}

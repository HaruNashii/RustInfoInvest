use std::env;
use std::fs;

fn main() 
{
    let home_path = env::home_dir().unwrap().display().to_string();

    //Move The Icon To The User Icon Folder
    let icon_path = "assets/icon/RustInfoInvest_LOGO_WITHOUT_BG.bmp";
    let target_icon_dir = format!("{}/.local/share/icons/", home_path).replace(" ", "");
    let icon_dir = format!("{}RustInfoInvest_LOGO_WITHOUT_BG.bmp", target_icon_dir).replace(" ", "");

    if !fs::exists(&target_icon_dir).unwrap() 
    {
        fs::create_dir_all(&target_icon_dir).expect("Failed to create icon directory");
    };

    if !fs::exists(&icon_dir).unwrap() 
    {
        fs::copy(icon_path, icon_dir).expect("Failed to copy icon");
    };
}

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() 
{
    //Move The Icon To The User Icon Folder
    let home_path = env::home_dir().unwrap().display().to_string();
    let icon_path = "assets/icon/RustInfoInvest_LOGO_WITHOUT_BG.bmp";
    let target_icon_dir = format!("{}/.local/share/icons/", home_path).replace(" ", "");
    let icon_dir = format!("{}RustInfoInvest_LOGO_WITHOUT_BG.bmp", target_icon_dir).replace(" ", "");
    if !fs::exists(&target_icon_dir).unwrap() 
    {
        fs::create_dir_all(&target_icon_dir).expect("Failed to create icon directory");
        println!("Directory Created Sucessfuly");
    };
    if !fs::exists(&icon_dir).unwrap() 
    {
        fs::copy(icon_path, icon_dir).expect("Failed to copy icon");
    };


    //Move The SDL_TTF Library To The Project Folder
    let local_lib_path = "assets/local_libs/libSDL3_ttf.so";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.ancestors().nth(3).unwrap().to_path_buf();
    let lib_destination = target_dir.join(PathBuf::from(local_lib_path).file_name().expect("Failed to get filename"),);
    if !fs::exists(&lib_destination).unwrap()
    {
        fs::copy(local_lib_path, lib_destination).expect("Failed to copy SDL_ttf library");
    }

    println!("cargo:rerun-if-changed={}", local_lib_path);
    println!("cargo:rustc-cfg=SDL_UNIX_CONSOLE_BUILD=ON");
    println!("cargo:env=SDL_UNIX_CONSOLE_BUILD=ON");
}

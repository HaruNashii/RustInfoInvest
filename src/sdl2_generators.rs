use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use crate::window::SDL2_TEXTURE_CREATOR;





pub fn gen_text<'a>(font_size: u16, pos: (i32, i32), content: String, font_color: Color) -> (Texture<'a>, Rect)
{
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("fonts/JetBrainsMono-Bold.ttf", font_size).unwrap();
    let surface = font.render(&content).blended(font_color).unwrap();
    let texture = unsafe{ SDL2_TEXTURE_CREATOR[0].create_texture_from_surface(&surface).unwrap() };
    let rect = Rect::new(pos.0, pos.1, surface.width(), surface.height());

    (texture, rect)
}

pub fn gen_image(pos: (i32, i32), size: (u32, u32), path_to_image: &str) -> (Texture, Rect)
{
    let texture = unsafe{SDL2_TEXTURE_CREATOR[0].load_texture(path_to_image).unwrap()};
    let rect = Rect::new(pos.0, pos.1, size.0, size.1);
    (texture, rect)
}   

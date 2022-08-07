use sdl2::render::WindowCanvas;
use sdl2::pixels;
use sdl2::rect::Rect;
use crate::chip8::PIXEL_SIZE;
pub struct Display;

impl Display {
    pub fn clear(canvas: &mut WindowCanvas){
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }
    pub fn render_gfx(canvas: &mut WindowCanvas){
        let x_pos: u16 = 1;
        let y_pos: u16 = 1;
    }
    pub fn get_rects(mut num_pixels: usize, set_vec: &mut Vec<Rect>, mut unset_vec: &mut Vec<Rect>){

    }
}

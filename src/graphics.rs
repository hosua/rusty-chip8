use sdl2::render::WindowCanvas;
use sdl2::pixels;
use sdl2::rect::Rect;
use std::vec::Vec;
use crate::chip8::Chip8;
use crate::chip8::PIXEL_SIZE;
use crate::chip8::DISP_X;
use crate::chip8::DISP_Y;
pub struct Display;

impl Display {
    pub fn clear(canvas: &mut WindowCanvas){
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }
    pub fn render_gfx(chip8: &mut Chip8, canvas: &mut WindowCanvas){
        let mut set_vec: Vec<Rect> = Vec::new();
        let mut unset_vec: Vec<Rect> = Vec::new();
        {
            let mut x_pos = 1;
            let mut y_pos = 1;
            if chip8.draw_flag {
                chip8.draw_flag = false;
                for i in 0..DISP_X*DISP_Y {
                    if chip8.gfx[i] != 0 {
                        set_vec.push(Rect::new(x_pos * PIXEL_SIZE as i32, 
                                               y_pos * PIXEL_SIZE as i32, 
                                               PIXEL_SIZE as u32, 
                                               PIXEL_SIZE as u32));
                    } else {
                        unset_vec.push(Rect::new(x_pos * PIXEL_SIZE as i32, 
                                                 y_pos * PIXEL_SIZE as i32, 
                                                 PIXEL_SIZE as u32, 
                                                 PIXEL_SIZE as u32));
                    }
                    if ((i+1) % DISP_X) == 0 {
                        y_pos += 1;
                        x_pos = 0;
                    }
                    x_pos += 1;
                }
            }
        }
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.fill_rects(&set_vec).ok();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.fill_rects(&unset_vec).ok();
        canvas.present();
    }
}

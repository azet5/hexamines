use std::collections::HashMap;

use sdl2::{EventPump, rect::Rect, render::{Canvas, Texture}, video::Window};

use crate::field::Field;

pub struct SdlData<'a> {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    // pub textures: HashMap<&'a str, Texture<'a>>,
    pub textures: Vec<Texture<'a>>
}

impl Field {
    pub fn handle_mouse(&mut self, x: i32, y: i32) {
        let (w, h) = self.size();
        let (x, y) = (
            x / 24,
            if (x / 24) % 2 == 0 {
                if y < 16 { 0 } else { (y -  16) / 32 }
            } else { y / 32 }
        );
        
        if x < w as i32 && y < h as i32 {
            self.reveal(x as usize, y as usize, true);
        }
    }

    pub fn render(&self, sdl: &mut SdlData) {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                let cell = self.get_cell(x, y);
                let dy = if x % 2 == 0 { 16 } else { 0 };
                if cell.1 {
                    sdl.canvas.copy(
                        &sdl.textures[cell.0 as usize],
                        None,
                        Some(Rect::new(x as i32 * 24, y as i32 * 32 + dy, 32, 32))
                    ).unwrap();
                } else {
                    sdl.canvas.copy(
                        &sdl.textures[9],
                        None,
                        Some(Rect::new(x as i32 * 24, y as i32 * 32 + dy, 32, 32))
                    ).unwrap();
                }
            }
        }
    }
}
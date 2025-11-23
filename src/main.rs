mod field;
mod state;

use std::time::Duration;

use sdl2::{event::Event, image::LoadTexture, mouse::MouseButton, pixels::Color};

use crate::{field::{Field, State}, state::SdlData};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    sdl2::hint::set("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");

    let window = video.window("hexamines", 320, 272)
        .position_centered()
        .build()
        .unwrap();
    
    let mut field = Field::new(13, 8, 16).unwrap();
    
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl.event_pump().unwrap();
    let creator = canvas.texture_creator();
    let textures = vec![
        creator.load_texture("data/cell0.png").unwrap(),
        creator.load_texture("data/cell1.png").unwrap(),
        creator.load_texture("data/cell2.png").unwrap(),
        creator.load_texture("data/cell3.png").unwrap(),
        creator.load_texture("data/cell4.png").unwrap(),
        creator.load_texture("data/cell5.png").unwrap(),
        creator.load_texture("data/cell6.png").unwrap(),
        creator.load_texture("data/cell7.png").unwrap(),
        creator.load_texture("data/cell8.png").unwrap(),
        // creator.load_texture("data/cell9.png").unwrap(),
        creator.load_texture("data/cell.png").unwrap(),
    ];
    let mut sdl = SdlData {
        canvas,
        event_pump,
        textures,
    };

    sdl.canvas.set_draw_color(Color::RGB(31, 37, 47));
    sdl.canvas.clear();
    sdl.canvas.present();

    'event: loop {
        if field.state() == State::Won {
            sdl.canvas.set_draw_color(Color::RGB(42, 157, 73));
        } else {
            sdl.canvas.set_draw_color(Color::RGB(31, 37, 47));
        }
        sdl.canvas.clear();
        field.render(&mut sdl);

        for e in sdl.event_pump.poll_iter() {
            match e {
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        if field.state() == State::Playing {
                            // let (w, h) = field.size();
                            // if x <= w as i32 * 32 && y <= h as i32 * 24 {
                                field.handle_mouse(x, y);
                            // }
                        }
                    }
                },
                Event::Quit { .. } => break 'event,
                _ => {},
            }
        }

        sdl.canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000 / 60));
    }
}

mod field;

use std::time::Duration;

use sdl2::{event::Event, pixels::Color};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("hexamines", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();
    'event: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'event,
                _ => {},
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000 / 60));
    }
}

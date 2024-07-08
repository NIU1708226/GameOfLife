extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod life;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let size : i32 = 64;
    let mut board: Vec<u8> = life::gen_board(size);

    let mut i: u32 = 0;
    'running: loop {
        i = (i + 12) % 255;
        canvas.set_draw_color(Color::RGB(i as u8, 64, (255 - i) as u8));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..size {
            for y in 0..size {
                if board[(y*size + x) as usize] == 0 {
                    let x = x as i32;
                    let y = y as i32;
                    let _ = canvas.fill_rect(sdl2::rect::Rect::new(x*16,y*16,16,16));
                }
            }
        }
        // Bad, allocating for every frame. But whatever.
        board = life::life(&board, size);

        let _ = canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

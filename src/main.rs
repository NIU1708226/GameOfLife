extern crate sdl2;
extern crate rand;


use rand::distributions::{Uniform, Distribution};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn life(grid: &Vec<u8>, size: i32) -> Vec<u8>
{
    let mut next = grid.clone();

    for _x in 0..size {
        for _y in 0..size {
            let mut sum = 0;

            for y in _y-1.._y+2 {
                for x in _x-1.._x+2 {
                    if !(_x == x && _y == y) {
                        if x >= 0 && x < size && y >= 0 && y < size {
                            sum += grid[(size*y + x) as usize];
                        }
                    }
                }
            }
            let neigbours = sum;
            if grid[(size*_y + _x) as usize] == 1 {
                if neigbours < 2 || neigbours > 3 {
                    next[(size*_y + _x) as usize] = 0;
                }
            }
            else
            {
                if neigbours == 3 {
                    next[(size*_y + _x) as usize] = 1;
                }
            }
        }
    }
    next
}


// viva:
// < 2: muerte
// 2,3: vive
// > 3, muerte

// muerta:
// 3: revive

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..2);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: u32 = 0;
    let size : i32 = 64;
    let mut board: Vec<u8> = (0..size*size).map(|_| die.sample(&mut rng)).collect();

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
        board = life(&board, size);
        // life(&board, size);
        let _ = canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

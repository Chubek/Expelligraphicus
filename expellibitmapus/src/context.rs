use std::time::Duration;

use crate::bitmap::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect;


pub fn render_to_screen(image: Canvas) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let (w, h) = image.get_size();

    let window = video_subsys
        .window("rust-sdl2_gfx: draw line & FPSManager", w as u32, h as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();

    let map = image.get_map();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }

                _ => {}
            }
        }

        for (i, v) in map.clone().into_iter().enumerate() {
            for (j, c) in v.into_iter().enumerate() {
                let color = match c.color_type {
                    crate::color::ColorType::RGB => {
                        let (r, g, b) = c.unravel_rgb();

                        pixels::Color::RGB(r, g, b)
                    }
                    crate::color::ColorType::RGBA => {
                        let (r, g, b, a) = c.unravel_rgba();

                        pixels::Color::RGBA(r, g, b, a)
                    }
                };

                canvas.set_draw_color(color);
                canvas
                    .draw_point(rect::Point::new(w as i32 - i as i32, h as i32 - j as i32))
                    .unwrap();
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

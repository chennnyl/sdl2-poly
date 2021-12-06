use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;

mod util;
mod draw;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2", 512, 384)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut secs_elapsed = 0.0;

    'mainLoop: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::GREEN);

        let (lissajous, _) = util::math::lissajous(
            canvas.viewport().width() as f64,
            canvas.viewport().height() as f64,
            11, 17
        );
        let domain = util::math::FunctionDomain::build_inclusive(secs_elapsed, secs_elapsed + 3.14);

        draw::primitive::draw_parametric_function(lissajous, domain, 0.01, &mut canvas).ok();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'mainLoop },
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        secs_elapsed += 1.0/60.0;
    }
}
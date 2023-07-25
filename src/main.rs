use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;

mod astro;
use astro::Astronaut;

type Percentage = u8;
type Celsius = f32;

pub struct Renderer { canvas: WindowCanvas }

impl Renderer {
    pub fn new(window: sdl2::video::Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }
    
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw_wireframe(&mut self, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_lines([
            Point::new(136,88),
            Point::new(112,88),
            Point::new(106,64),
            Point::new(112,40),
            Point::new(136,40),
            Point::new(142,64),
            Point::new(136,88),
            // Head ^^
            Point::new(142,100),
            Point::new(172,112),
            Point::new(184,136),
            Point::new(196,184),
            Point::new(196,244),
            Point::new(184,244),
            Point::new(172,184),
            Point::new(160,148),
            Point::new(157,232),
            // Right Leg vv
            Point::new(163,328),
            Point::new(163,424),//
            Point::new(145,424),
            Point::new(133,328),
            Point::new(124,268),
            // Left Leg vv
            Point::new(115,328),
            Point::new(103,424),
            Point::new(85, 424),
            Point::new(85, 328),
            Point::new(91, 232),
            // Left Side
            Point::new(85, 148),
            Point::new(73, 184),
            Point::new(61, 244),
            Point::new(49, 244),
            Point::new(49, 184),
            Point::new(61, 136),
            Point::new(73, 112),
            Point::new(106,100),
            Point::new(112,88),
        ].as_slice())?; // *0.2 *3 y+40 x+40

        self.canvas.present();
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut astro = Astronaut::default();

    astro.rad_exposure = 1000;
    dbg!(&astro);
    println!("{}%", astro.calculate_overall());

    //SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("MIST", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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
        renderer.draw_wireframe(Color::RGB(255,0,0))?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


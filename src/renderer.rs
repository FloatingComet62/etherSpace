pub trait Renderer {
    /// Just draw a rectangle to the screen
    /// x -> abscissa of top left corner
    /// y -> ordinate of top left corner
    /// width -> width of rectangle
    /// height -> height of rectangle
    fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: (u8, u8, u8));

    /// Just draw a circle to the screen
    /// x -> abscissa of center
    /// y -> ordinate of center
    /// radius -> radii of the circle
    fn draw_circle(&mut self, x: u32, y: u32, radius: u32, color: (u8, u8, u8));
}

extern crate sdl2;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};
use std::{thread::sleep, time::Duration};
const DIMENSIONS: [u32; 2] = [800, 600];
const FPS: u32 = 60;

pub struct SDLRenderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}
impl SDLRenderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL context");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize Video subsystem");
        let window = video_subsystem
            .window("demo", DIMENSIONS[0], DIMENSIONS[1])
            .position_centered()
            .build()
            .expect("Failed to initialize Window");
        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Failed to initialize Canvas");
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context
            .event_pump()
            .expect("Failed to initialize Event pump");

        Self { canvas, event_pump }
    }
    pub fn start_loop(&mut self) {
        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        self.canvas.clear();
    }
    // if true is returned, exit
    pub fn end_loop(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            return match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => true,
                _ => false,
            };
        }
        self.canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        false
    }
}
impl Renderer for SDLRenderer {
    fn draw_circle(&mut self, x: u32, y: u32, radius: u32, color: (u8, u8, u8)) {
        self.canvas
            .set_draw_color(Color::RGB(color.0, color.1, color.2));
        let radius = radius as i32;
        for i in x as i32 - radius..x as i32 + radius {
            for j in y as i32 - radius..y as i32 + radius {
                let delta = (x as i32 - i as i32, y as i32 - j as i32);
                if delta.0 * delta.0 + delta.1 * delta.1 <= radius * radius {
                    let _ = self
                        .canvas
                        .draw_point(Point::new(x as i32 + delta.0, y as i32 + delta.1));
                }
            }
        }
    }
    fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: (u8, u8, u8)) {
        self.canvas
            .set_draw_color(Color::RGB(color.0, color.1, color.2));
        let rect = Rect::new(x as i32, y as i32, width, height);
        self.canvas
            .fill_rect(rect)
            .expect("Failed to fill rectangle");
    }
}

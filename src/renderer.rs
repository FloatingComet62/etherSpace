use downcast_rs::{impl_downcast, Downcast};
pub trait Renderer: Downcast {
    /// Just draw a rectangle to the screen
    /// x -> abscissa of top left corner
    /// y -> ordinate of top left corner
    /// width -> width of rectangle
    /// height -> height of rectangle
    fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32);

    /// Just draw a circle to the screen
    /// x -> abscissa of center
    /// y -> ordinate of center
    /// radius -> radii of the circle
    fn draw_circle(&mut self, x: u32, y: u32, radius: u32);
}
impl_downcast!(Renderer);

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
    fn draw_circle(&mut self, x: u32, y: u32, radius: u32) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in x - radius..x + radius {
            for j in y - radius..y + radius {
                let delta = (x - i, y - j);
                if delta.0 * delta.0 + delta.1 * delta.1 <= radius * radius {
                    let _ = self
                        .canvas
                        .draw_point(Point::new((x + delta.0) as i32, (y + delta.1) as i32));
                }
            }
        }
    }
    fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(x as i32, y as i32, width, height);
        self.canvas
            .fill_rect(rect)
            .expect("Failed to fill rectangle");
    }
}

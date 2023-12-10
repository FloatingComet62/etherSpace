/*

use ether_space::log::Log;
use ggez::event::{self, EventHandler, EventLoop};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    Log::info("Initializing");

    let binding = ContextBuilder::new("etherSpace", "FloatingComet62").build();
    let mut context: Context;
    let event_loop: EventLoop<()>;
    match binding {
        Ok(val) => (context, event_loop) = val,
        Err(_) => {
            Log::critical("Failed to initialize the window context");
            return;
        }
    }
    let window = EtherSpaceEngine::new(&mut context);
    event::run(context, event_loop, window);
}

struct EtherSpaceEngine {
    //pos_x: f32,
    // circle: graphics::Mesh,
}
impl EtherSpaceEngine {
    pub fn new(_context: &mut Context) -> Self {
        /*let circle = graphics::Mesh::new_circle(
            context,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::BLUE,
        )
        .unwrap();
        Self { pos_x: 0.0, circle }*/
        Self {}
    }
}

impl EventHandler for EtherSpaceEngine {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        // canvas.draw(&self.circle, vec2(self.pos_x, 380.0));
        canvas.finish(context)
    }
}

*/

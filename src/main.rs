extern crate find_folder;
extern crate graphics;
extern crate mold_world;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use mold_world::report_struct_size;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::PistonWindow as Window;

pub struct Game {
    gl: GlGraphics,
    rotation: f64,
}

impl Game {
    fn new(gl: GlGraphics) -> Game {
        Game {
            gl: gl,
            rotation: 0.0,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FG_COLOR: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

        const SQUARE_SZ: f64 = 25.0;

        let rotation = self.rotation;
        let (x, y) = (args.width as f64 / 2.0, args.height as f64 / 2.0);
        let square = rectangle::centered_square(0.0, 0.0, SQUARE_SZ);
        let rect = rectangle::rectangle_by_corners(5.0, 5.0, args.width as f64 - 5.0, args.height as f64 - 5.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG_COLOR, gl);
            let transform = c.transform.trans(x, y).rot_rad(rotation);
            rectangle([0.0, 0.0, 0.5, 1.0], rect, c.transform, gl);
            rectangle(FG_COLOR, square, transform, gl);
            // top left is red
            rectangle([1.0, 0.0, 0.0, 1.0], rectangle::rectangle_by_corners(1.0, 1.0, 3.0, 3.0), c.transform, gl);
            // top right is yellow
            rectangle([1.0, 1.0, 0.0, 1.0], rectangle::rectangle_by_corners(args.width as f64 - 1.0, 1.0, args.width as f64 - 3.0, 3.0), c.transform, gl);
            // bottom right is purple
            rectangle([1.0, 0.0, 1.0, 1.0], rectangle::rectangle_by_corners(args.width as f64 - 1.0, args.height as f64 - 1.0, args.width as f64 - 3.0, args.height as f64 - 3.0), c.transform, gl);
            // bottom left is green
            rectangle([0.0, 1.0, 0.0, 1.0], rectangle::rectangle_by_corners(1.0, args.height as f64 - 1.0, 3.0, args.height as f64 - 3.0), c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 1.5 * args.dt;
    }
}

fn main() {
    report_struct_size();
    let opengl = OpenGL::V3_3;

//    let mut window: Window = WindowSettings::new("bla", [400, 400])
//        .opengl(opengl)
//        .exit_on_esc(true)
//        .vsync(true)
//        .build()
//        .unwrap();

    // fixme: it's a workaround until mesa 18.0 is released, https://github.com/PistonDevelopers/piston/issues/1202
    let mut window: Window = Window::new(
        opengl,
        0,
        WindowSettings::new("bla", [400, 400])
            .opengl(opengl)
            .exit_on_esc(true)
            .vsync(true)
            .srgb(false)
            .build()
            .unwrap(),
    );

    let mut game = Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(u) = e.update_args() {
            game.update(&u);
        }
//        if let Some(u) = e.() {
//            game.update(&u);
//        }
    }
}

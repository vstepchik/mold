extern crate amethyst;
extern crate mold_world;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput,
                         Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};
use amethyst::Result;
use mold_world::report_struct_size;

struct Mold;

impl State for Mold {
    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn run() -> Result<()> {
    let path = "./resources/display_config.ron";

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.2, 0.15, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let mut game = Application::build("./", Mold)?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .build()?;
    game.run();

    Ok(())
}

fn main() {
    report_struct_size();
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}

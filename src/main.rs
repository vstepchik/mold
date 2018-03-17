extern crate amethyst;
extern crate world;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use amethyst::Result;
use mold::Mold;
use systems::PlayerSystem;
use world::report_struct_size;

mod mold;
mod systems;
mod display;

fn run() -> Result<()> {
    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./resources/bindings.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.1, 0.07, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let mut game = Application::build("./", Mold)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(input_bundle)?
        .with(PlayerSystem, "player_system", &[])
        .build()?;
    game.run();

    Ok(())
}

fn main() {
    report_struct_size();
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    } else {
        println!("\n~See ya!")
    }
}

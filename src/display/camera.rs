use amethyst::prelude::*;
use amethyst::renderer::Camera;

pub fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d())
        .build();
}

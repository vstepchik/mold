use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Event, KeyboardInput, Material, MaterialDefaults,
                         MeshHandle, PosTex, VirtualKeyCode, WindowEvent};

pub struct Mold;
pub const SIZE: f32 = 0.05;

impl State for Mold {
    fn on_start(&mut self, world: &mut World) {
        world.register::<Player>();
        initialise_player(world);
        initialise_camera(world);
    }

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

fn initialise_player(world: &mut World) {
    let mut transform = Transform::default();

    transform.translation = Vector3::new(0.0, 0.0, 0.0);

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(-SIZE / 2.0, -SIZE / 2.0, SIZE / 2.0, SIZE / 2.0),
    );

    let material = create_colour_material(world, [0.5, 0.0, 0.0, 1.0]);

    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Player::new())
        .with(GlobalTransform::default())
        .with(transform)
        .build();
}

fn generate_rectangle_vertices(left: f32,
                               bottom: f32,
                               right: f32,
                               top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: [left, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [right, top, 0.],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
    ]
}

fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d())
        .build();
}

#[derive(Debug)]
pub struct Player;

impl Player {
    fn new() -> Self {
        Player
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

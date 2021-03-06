use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{Event, KeyboardInput, Material, MaterialDefaults,
                         MeshHandle, PosTex, VirtualKeyCode, WindowEvent, Texture, PngFormat};
use amethyst::ui::{MouseReactive, UiImage, UiTransform, Anchor};
use display::camera;

pub struct Mold;

pub const SIZE: f32 = 0.05;

impl State<()> for Mold {
    fn on_start(&mut self, world: &mut World) {
        let logo = Mold::load_texture(world, "assets/human.png");
        world
            .create_entity()
            .with(UiTransform::new(
                "logo".to_string(),
                Anchor::TopLeft,
                100.,
                100.,
                0.,
                32.,
                32.,
                0,
            ))
            .with(UiImage {
                texture: logo.clone(),
            })
            .with(MouseReactive)
            .build();

        world.register::<Player>();
        initialise_player(world);
        camera::initialise_camera(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans<()> {
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

fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
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

impl Mold {
    fn load_texture(world: &World, path: &str) -> Handle<Texture> {
        let loader = world.read_resource::<Loader>();
        loader.load(
            path,
            PngFormat,
            Default::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    }
}

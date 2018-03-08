use amethyst::core::transform::Transform;
use amethyst::ecs::{Fetch, Join, System};
use amethyst::ecs::{ReadStorage, WriteStorage};
use amethyst::input::InputHandler;
use mold::Player;

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Fetch<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        let scale = | mv_amount: f32 | (1.0 / 60.0) * mv_amount;

        for (_player, transform) in (&players, &mut transforms).join() {
            if let Some(amount) = input.axis_value("player_v") {
                transform.translation[1] = (transform.translation[1] + scale(amount as f32))
                    .min(1.0)
                    .max(-1.0);
            }

            if let Some(amount) = input.axis_value("player_h") {
                transform.translation[0] = (transform.translation[0] + scale(amount as f32))
                    .min(1.0)
                    .max(-1.0);
            }
        }
    }
}


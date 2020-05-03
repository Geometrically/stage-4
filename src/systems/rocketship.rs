use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::slt::{Rocket, ARENA_WIDTH, ROCKET_WIDTH};

#[derive(SystemDesc)]
pub struct RocketSystem;

impl<'s> System<'s> for RocketSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rocket>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, rocketships, input): Self::SystemData) {
        let x_move = input.axis_value("rocketship").unwrap();
        let scaled_movement = 10.0 * x_move as f32;

        for (_, transform) in (&rocketships, &mut transforms).join() {
            transform.set_translation_x((transform.translation().x + scaled_movement)
                .min(ARENA_WIDTH - ROCKET_WIDTH * 0.5)
                .max(ROCKET_WIDTH * 0.5),
            );
        }
    }
}


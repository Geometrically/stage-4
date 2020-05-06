use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::slt::{Rocket, ARENA_WIDTH, ROCKET_WIDTH, ROCKET_X_SPEED};

#[derive(SystemDesc)]
pub struct RocketSystem;

impl<'s> System<'s> for RocketSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rocket>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, rockets, input, time): Self::SystemData) {
        let x_move = input.axis_value("rocketship").unwrap();
        let scaled_movement = ROCKET_X_SPEED * time.delta_seconds() * x_move as f32;

        for (rocket, transform) in (&rockets, &mut transforms).join() {
            let rocket_x = transform.translation().x;
            let rocket_y = transform.translation().y;

            transform.set_translation_x((rocket_x + scaled_movement)
                .min(ARENA_WIDTH - ROCKET_WIDTH * 0.5)
                .max(ROCKET_WIDTH * 0.5),
            );

            transform.set_translation_y(rocket_y + rocket.y_speed * time.delta_seconds());
        }
    }
}

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{Camera},
};

use crate::slt::ROCKET_Y_SPEED;

#[derive(SystemDesc)]
pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut locals, time): Self::SystemData) {
        for (_, local) in (&cameras, &mut locals).join() {
            local.prepend_translation_y(ROCKET_Y_SPEED * time.delta_seconds());
        }
    }
}

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{Camera},
};

use crate::slt::{Rocket};

#[derive(SystemDesc)]
pub struct MoveCameraSystem;

impl<'s> System<'s> for MoveCameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rocket>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut locals, rockets, time): Self::SystemData) {
        for (_, local) in (&cameras, &mut locals).join() {
            for rocket in rockets.join() {
                local.prepend_translation_y(rocket.y_speed * time.delta_seconds());
            }
        }
    }
}

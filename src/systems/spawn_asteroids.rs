use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::{Join, ReadExpect, System, SystemData, Write, WriteStorage, ReadStorage, Read};
use amethyst::ui::UiText;
use amethyst::utils::fps_counter::FpsCounter;

use crate::slt::{Rocket, Asteroid, ARENA_HEIGHT, ROCKET_WIDTH, ROCKET_HEIGHT, ScoreBoard, ScoreText};
use rand::Rng;

#[derive(SystemDesc)]
pub struct SpawnAsteroidSystem;

impl<'s> System<'s> for SpawnAsteroidSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rocket>,
        ReadStorage<'s, Asteroid>,
        Read<'s, FpsCounter>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (mut transforms, rockets, asteroids, counter, mut ui_text, mut scores, score_text): Self::SystemData) {
        let mut rocket_x = 0.0;
        let mut rocket_y = 0.0;

        for (_rocket, transform) in (&rockets, &transforms).join() {
            rocket_x = transform.translation().x;
            rocket_y = transform.translation().y;
        }

        for (asteroid, asteroid_transform) in (&asteroids, &mut transforms).join() {
            let asteroid_x = asteroid_transform.translation().x;
            let asteroid_y = asteroid_transform.translation().y;

            if point_in_rect(asteroid_x, asteroid_y, &rocket_x - asteroid.radius, &rocket_y - asteroid.radius, &rocket_x + ROCKET_WIDTH + asteroid.radius, &rocket_y + ROCKET_HEIGHT + asteroid.radius) {
                println!("Hit Asteroid");
            }

            if asteroid_y < (&rocket_y - ARENA_HEIGHT / 4.0) {
                let mut rng = rand::thread_rng();
                let roll = rng.gen_range(1, 9);

                asteroid_transform.set_translation_y(&rocket_y + (3.1 * ARENA_HEIGHT) / 4.0);
                asteroid_transform.set_translation_x(50.0 + (100 * roll) as f32);

                scores.score = scores.score + 1;

                if let Some(text) = ui_text.get_mut(score_text.score) {
                    text.text = scores.score.to_string();
                }
            }
        }

        println!("{}", counter.frame_fps())
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
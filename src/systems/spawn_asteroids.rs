use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::{Join, System, SystemData, Write, WriteStorage, ReadStorage, ReadExpect};
use amethyst::ui::UiText;

use crate::slt::{Rocket, Asteroid, ARENA_HEIGHT, ROCKET_WIDTH, ROCKET_HEIGHT, ScoreBoard, ScoreText};
use rand::Rng;

#[derive(SystemDesc)]
pub struct SpawnAsteroidSystem;

impl<'s> System<'s> for SpawnAsteroidSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Rocket>,
        ReadStorage<'s, Asteroid>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (mut transforms, mut rockets, asteroids, mut ui_text, mut scores, score_text): Self::SystemData) {
        let mut rocket_x = 0.0;
        let mut rocket_y = 0.0;

        let mut rocket = &mut Rocket {
            width: 0.0,
            height: 0.0,
            y_speed: 0.0
        };

        for (rocketship, transform) in (&mut rockets, &transforms).join() {
            rocket_x = transform.translation().x;
            rocket_y = transform.translation().y;

            rocket = rocketship;
        }

        for (asteroid, asteroid_transform) in (&asteroids, &mut transforms).join() {
            let asteroid_x = asteroid_transform.translation().x;
            let asteroid_y = asteroid_transform.translation().y;

            if point_in_rect(asteroid_x, asteroid_y, &rocket_x - asteroid.radius, &rocket_y - asteroid.radius, &rocket_x + ROCKET_WIDTH + asteroid.radius, &rocket_y + ROCKET_HEIGHT + asteroid.radius) {
                scores.status = "Game Over".to_string();

                if let Some(text) = ui_text.get_mut(score_text.status) {
                    text.text = scores.status.to_string();
                }
            }

            if scores.status != "Game Over".to_string() {
                let s = "-.-. --- -.. . .. ... ..--- ----. .---- ----. ----- END";
                let current_morse = s.chars().nth(((scores.score / 5) % s.len() as i32) as usize).unwrap();

                if let Some(text) = ui_text.get_mut(score_text.status) {
                    if current_morse == '-' {
                        text.color = [0.13, 0.65, 0.87, 1.];
                    } else if current_morse == '.'
                    {
                        text.color = [0.4, 0.95, 0.07, 1.];
                    } else if current_morse == ' ' {
                        text.color = [1., 1., 1., 1.];
                    } else {
                        text.color = [0.95, 0.07, 0.07, 1.];
                    }
                }
            }

            if asteroid_y < (&rocket_y - ARENA_HEIGHT / 4.0) {
                let mut rng = rand::thread_rng();
                let roll = rng.gen_range(0, 10);

                rocket.y_speed += 1.0;

                asteroid_transform.set_translation_y(&rocket_y + (3.1 * ARENA_HEIGHT) / 4.0);
                asteroid_transform.set_translation_x((40 + 100 * roll) as f32);

                scores.score = scores.score + 1;

                if let Some(text) = ui_text.get_mut(score_text.score) {
                    text.text = scores.score.to_string();
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
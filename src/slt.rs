use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{Transform, math::Vector3},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use rand::Rng;
use std::vec::Vec;
use amethyst::ui::LineMode;

pub const ARENA_HEIGHT : f32 = 1000.0;
pub const ARENA_WIDTH : f32 = 1000.0;

pub const ASTEROID_RADIUS : f32 = 18.0;

pub const ROCKET_WIDTH : f32 = 50.0;
pub const ROCKET_HEIGHT : f32 = 50.0;

pub const ROCKET_X_SPEED : f32 = 400.0;
pub const ROCKET_Y_SPEED : f32 = 200.0;

#[derive(Default)]
pub struct ScoreBoard {
    pub score: i32,
    pub status: String,
}

pub struct ScoreText {
    pub score: Entity,
    pub status : Entity,
}

pub struct SpaceLaunchTrainer {
    pub game_over: bool,
}

impl SimpleState for SpaceLaunchTrainer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let rocket_sprite = load_sprite_sheet(world, "rocketship");
        let asteroids_sprite = load_sprite_sheet(world, "asteroids");

        world.register::<Asteroid>();

        initialise_rocket(world, rocket_sprite);
        initialise_camera(world);
        initialise_scoreboard(world);

        let mut rng = rand::thread_rng();
        for _x in 0..18 {
            let x_roll = rng.gen_range(0, 11);
            let y_roll = rng.gen_range(0, 10);
            let sprite_roll = rng.gen_range(0, 2);

            initialise_asteroid(world, asteroids_sprite.clone(), 15.0 + (x_roll as f32) * 100.0, 550.0 + (y_roll as f32) * 100.0, sprite_roll);
        }
    }
    fn fixed_update(&mut self, mut data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        let mut scores = world.write_resource::<ScoreBoard>();

        if scores.status == "Game Over" {
            scores.score = 0;
            scores.status = "".to_string();

            self.game_over = true;
        }
        Trans::None
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.game_over {
            let world = &mut data.world;

            world.delete_all();

            return Trans::Push(Box::new(SpaceLaunchTrainer { game_over: false }));
        }
        Trans::None
    }
}

pub struct Rocket {
    pub width: f32,
    pub height: f32,

    pub y_speed: f32,
}

impl Rocket {
    fn new() -> Rocket {
        Rocket {
            width: ROCKET_WIDTH,
            height: ROCKET_HEIGHT,
            y_speed: ROCKET_Y_SPEED,
        }
    }
}

impl Component for Rocket {
    type Storage = DenseVecStorage<Self>;
}

pub struct Asteroid {
    pub radius : f32,
}

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) -> Entity{
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build()
}

fn initialise_rocket(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity{
    let mut rocket_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    rocket_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 4.0, 0.0);
    rocket_transform.set_scale(Vector3::new(0.5, 0.5, 0.0));

    world.create_entity()
        .with(Rocket::new())
        .with(rocket_transform)
        .with(sprite_render)
        .build()
}

pub fn initialise_asteroid(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, x: f32, y: f32, sprite: usize) -> Entity {
    let mut asteroid_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite,
    };

    asteroid_transform.set_translation_xyz(x, y, 0.0);

    world.create_entity()
        .with(Asteroid {
            radius: ASTEROID_RADIUS,
        })
        .with(asteroid_transform)
        .with(sprite_render)
        .build()
}

fn initialise_scoreboard(world: &mut World) -> Vec<Entity>{
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let score_transform = UiTransform::new(
        "Score".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        0., -50., 1., 200., 50.,
    );

    let score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::TopMiddle,
        ))
        .build();

    let status_transform = UiTransform::new(
        "Status".to_string(), Anchor::BottomMiddle, Anchor::BottomMiddle,
        0., 25., 1., 800., 25.,
    );

    let status = world
        .create_entity()
        .with(status_transform)
        .with(UiText::new(
            font.clone(),
            "Asteroids".to_string(),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::TopMiddle,
        ))
        .build();

    world.insert(ScoreText { score, status });

    return vec![score, status];
}


fn load_sprite_sheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            format!("sprites/{}.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        format!("sprites/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
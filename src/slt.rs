use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT : f32 = 1000.0;
pub const ARENA_WIDTH : f32 = 1000.0;

pub const ROCKET_WIDTH : f32 = 238.0;
pub const ROCKET_HEIGHT : f32 = 250.0;

pub struct SpaceLaunchTrainer;

impl SimpleState for SpaceLaunchTrainer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_rocket(world, load_sprite_sheet(world, "rocketship"));
        initialise_camera(world);
        initialise_asteroid(world, load_sprite_sheet(world, "asteroids"))
    }
}

pub struct Rocket {
    pub width: f32,
    pub height: f32,
}

impl Rocket {
    fn new() -> Rocket {
        Rocket {
            width: ROCKET_WIDTH,
            height: ROCKET_HEIGHT,
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

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_rocket(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rocket_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    rocket_transform.set_translation_xyz(ARENA_WIDTH / 2.0 + ROCKET_WIDTH / 2.0, ARENA_HEIGHT / 2.0 + ROCKET_HEIGHT / 2.0, 0.0);

    world.create_entity()
        .with(Rocket::new())
        .with(rocket_transform)
        .with(sprite_render)
        .build();
}

fn initialise_asteroid(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut asteroid_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    asteroid_transform.set_translation_xyz(10.0, 10.0, 0.0);

    world.create_entity()
        .with(Rocket::new())
        .with(rocket_transform)
        .with(sprite_render)
        .build();
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
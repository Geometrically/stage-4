use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT : f32 = 1000.0;
pub const ARENA_WIDTH : f32 = 1000.0;

pub const ROCKET_WIDTH : f32 = 4.0;
pub const ROCKET_HEIGHT : f32 = 16.0;

pub struct SpaceLaunchTrainer;

impl SimpleState for SpaceLaunchTrainer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Rocket>();

        initialise_rocket(world, sprite_sheet_handle);
        initialise_camera(world);
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

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
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

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "sprites/rocketship.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "sprites/rocketship.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
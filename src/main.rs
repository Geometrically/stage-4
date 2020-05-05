use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings}
};

mod systems;
mod slt;

use crate::slt::SpaceLaunchTrainer;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");

    let display_config = resources.join("display.ron");
    let binding_config = resources.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(systems::RocketSystem, "rocket_system", &["input_system"])
        .with(systems::MoveCameraSystem, "camera_system", &[])
        .with(systems::SpawnAsteroidSystem, "asteroid_system", &[]);

    let mut game = Application::new(resources, SpaceLaunchTrainer, game_data)?;
    game.run();

    Ok(())
}
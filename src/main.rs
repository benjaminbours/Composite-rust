use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgb,
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

mod composite;
mod states;
mod systems;
mod components;
use crate::composite::MyPrefabData;
use crate::states::Loading;
use crate::systems::FpsSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");

    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with(FpsSystem::default(), "fps_system", &[])
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                )),
        )?;
    let mut game = Application::new(assets_dir, Loading::default(), game_data)?;
    game.run();
    Ok(())
}

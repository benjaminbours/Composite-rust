use amethyst::{
    assets::{PrefabLoader, RonFormat},
    // core::{math::Vector3, timing::Time, transform::Transform},
    // ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::rendy::{
        mesh::{Normal, Position, TexCoord},
        texture::palette::load_from_srgba,
    },
    utils::scene::BasicScenePrefab, // ui::{Anchor, TtfFormat, UiText, UiTransform, UiCreator},
};

use crate::{
    entities::{load_camera, load_player},
    resources::Context,
};

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
pub struct BasicScene;

impl SimpleState for BasicScene {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(Context::new());

        let ctx = *world.read_resource::<Context>();
        load_player(world, &ctx);

        load_camera(world);

        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/basic_scene.ron", RonFormat, ())
        });
        world.create_entity().with(handle).build();
    }
}

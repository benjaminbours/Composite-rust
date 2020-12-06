use amethyst::{
    assets::Handle,
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::{camera::Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
};

use crate::{components::Player, ressources::Context};

pub fn load_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let ctx = *world.read_resource::<Context>();
    let mut position = Transform::default();
    position.set_translation_xyz(ctx.width * 0.5, ctx.height * 0.5, 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // first sprite
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(position)
        .with(Player::new())
        .build();
}

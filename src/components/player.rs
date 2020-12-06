use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector2, timing::Time, transform::Transform},
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub is_moving: bool,
}

impl Player {
    pub fn new() -> Self {
        Player { is_moving: false }
    }
}

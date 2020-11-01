use amethyst::{
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    prelude::*,
};

use crate::{
    components::{Collider, Player},
    resources::Context,
};

pub fn load_player(world: &mut World, ctx: &Context) {
    let x_position = 384.;
    let y_position = 176.;
    let scale = ctx.scale;

    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_x(x_position);
    transform.set_translation_y(y_position);

    let mut collider = Collider::new(32. * scale, 32. * scale);
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(x_position, y_position);
    bbox.old_position = bbox.position;

    world
        .create_entity()
        .with(Player::new())
        .named("MainPlayer")
        .with(collider)
        .build();
}

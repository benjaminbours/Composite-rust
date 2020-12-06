use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

pub use crate::ressources::Context;

pub fn load_camera(world: &mut World) {
    let ctx = *world.read_resource::<Context>();
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left
    let mut transform = Transform::default();
    transform.set_translation_xyz(ctx.width * 0.5, ctx.height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ctx.width * 2., ctx.height * 2.))
        .with(transform)
        .build();
}

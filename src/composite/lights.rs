use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{
        light::{Light, PointLight},
        palette::Srgb,
    },
};

pub fn initialise_lights(world: &mut World) {
    let light: Light = PointLight {
        intensity: 100.0,
        radius: 1.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..Default::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, -20.0, 15.0);

    // Add point light
    world.create_entity().with(light).with(transform).build();
}

use amethyst::{core::Transform, prelude::*, renderer::camera::Camera, window::ScreenDimensions};

pub fn load_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 10.);

    world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}

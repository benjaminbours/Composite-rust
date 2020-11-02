use amethyst::{
    assets::AssetLoaderSystemData,
    core::{
        math::{Vector2, Vector3},
        Transform,
    },
    prelude::*,
    renderer::{
        camera::Camera,
        light::{Light, PointLight},
        mtl::{Material, MaterialDefaults},
        palette::{LinSrgba, Srgb},
        rendy::{
            mesh::{Normal, Position, Tangent, TexCoord},
            texture::palette::load_from_linear_rgba,
        },
        shape::Shape,
        transparent::Transparent,
        Mesh, Texture,
    },
    window::ScreenDimensions,
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

    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    // Load mesh
    let (mesh, albedo) = {
        let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Sphere(32, 32)
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                    .into(),
                (),
            )
        });
        let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
                (),
            )
        });

        (mesh, albedo)
    };

    // Create player
    let mut pos = Transform::default();
    pos.set_translation_xyz(0., 0., 0.);

    let roughness = 1.0f32 * 1. / 4.0f32;
    let metallic = 1.0f32 * 1. / 4.0f32;

    let mtl = world.exec(
        |(mtl_loader, text_loader): (
            AssetLoaderSystemData<'_, Material>,
            AssetLoaderSystemData<'_, Texture>,
        )| {
            let metallic_roughness = text_loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
                (),
            );

            mtl_loader.load_from_data(
                Material {
                    albedo: albedo.clone(),
                    metallic_roughness,
                    ..mat_defaults.clone()
                },
                (),
            )
        },
    );

    world
        .create_entity()
        .with(Player::new())
        .named("MainPlayer")
        .with(collider)
        .with(pos)
        .with(mesh.clone())
        .with(mtl)
        // .with(Transparent)
        .build();
}

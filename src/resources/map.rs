use amethyst::{
    assets::{Asset, AssetLoaderSystemData, Handle, ProcessingState},
    core::{math::Vector3, Transform},
    ecs::VecStorage,
    error::Error,
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
};
use serde::{Deserialize, Serialize};

use crate::{components::Collider, resources::Context};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Property {
    pub name: String,
    pub value: usize,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub name: String,
    pub rotation: f32,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub properties: Option<Vec<Property>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub layers: Vec<Layer>,
}

impl Asset for Map {
    const NAME: &'static str = "composite::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        Ok(ProcessingState::Loaded(map))
    }
}

impl Map {
    pub fn load_layers(&self, world: &mut World, ctx: &Context) {
        for layer in self.layers.iter() {
            match layer.name.as_ref() {
                "collision" => {
                    self.load_collision_layer(world, layer, ctx);
                }
                _ => {
                    self.load_non_collision_layer(world, layer, ctx);
                }
            }
        }
    }

    fn load_collision_layer(&self, world: &mut World, layer: &Layer, ctx: &Context) {
        let scale = ctx.scale;
        println!("Hola I am here");

        for obj in layer.objects.iter() {
            let mut transform = Transform::default();
            transform.set_translation_xyz(0., 30., 0.);

            let mut collider =
                Collider::new(obj.width * scale, obj.height * scale, obj.depth * scale);
            let bbox = &mut collider.bounding_box;
            bbox.position = Vector3::new(0., 0., 0.);
            bbox.old_position = bbox.position;

            let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

            // Load mesh
            let (mesh, albedo) = {
                let mesh =
                    world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
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
                .named("Collision")
                .with(transform)
                .with(collider)
                .with(mesh.clone())
                .with(mtl)
                .build();
        }
    }

    fn load_non_collision_layer(&self, world: &mut World, layer: &Layer, ctx: &Context) {}
}

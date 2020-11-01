use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone)]
pub struct GenericBox {
    pub half_size: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
}

impl Default for GenericBox {
    fn default() -> Self {
        Self {
            half_size: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
        }
    }
}

impl GenericBox {
    pub fn new(width: f32, height: f32) -> Self {
        GenericBox {
            half_size: Vector2::new(width / 2., height / 2.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
        }
    }
}

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub bounding_box: GenericBox,
    pub hit_box: GenericBox,
    pub hit_box_offset: Vector2<f32>,
    pub on_ground: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
    pub is_collidable: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            bounding_box: GenericBox::default(),
            hit_box: GenericBox::default(),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: true,
        }
    }
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Collider {
            bounding_box: GenericBox::new(width, height),
            hit_box: GenericBox::new(width, height),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: false,
        }
    }
}

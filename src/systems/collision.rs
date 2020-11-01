use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Collidee, Collider, Player},
    resources::Context,
};
// pub struct CollisionSystem;

// impl<'a> System<'a> for CollisionSystem {
//     type SystemData = (
//         Entities<'s>,
//         ReadStorage<
//     );

//     fn run(mut self, data: Self::SystemData) {
//     }
// }

pub struct PlayerCollisionSystem;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Collidee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut colliders, collidees) = data;

        // for (_, collider, collidee) in (&players, &mut colliders, &collidees).join() {
        //     if let Some(collidee_horizontal) = &collidee.horizontal {
        //         if let "Pincer" = collidee_horizontal.name.as_ref() {
        //             collider.is_collidable = false;
        //         }
        //         if let "Flier" = collidee_horizontal.name.as_ref() {
        //             collider.is_collidable = false;
        //         }
        //     }
        // }
    }
}

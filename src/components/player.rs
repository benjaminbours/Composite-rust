use amethyst::ecs::{Component, DenseVecStorage};

pub enum PlayerState {
    Jumping,
    Running,
    Idling,
    Dying,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Idling,
        }
    }
}

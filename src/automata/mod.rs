use bevy::prelude::*;
pub mod rule;
pub mod state;
mod traits;

#[derive(Resource)]
pub struct SimState{
    pub is_running: bool,
}
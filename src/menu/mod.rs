pub mod control_screen;
use bevy::prelude::*;
use crate::menu::control_screen::{interact_with_pause, spawn_control_screen};


pub struct ControlScreen;

impl Plugin for ControlScreen{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_control_screen)
            .add_systems(Update,interact_with_pause);
    }
}
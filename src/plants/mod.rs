use bevy::prelude::*;
pub mod commands;

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(commands::PlantsCommandsPlugin);
    }
}

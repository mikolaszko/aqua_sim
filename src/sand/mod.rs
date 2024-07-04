pub mod metadata;

use bevy::prelude::*;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(metadata::SandMetadataPlugin);
    }
}

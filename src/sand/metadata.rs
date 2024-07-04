use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SandAmount {
    pub grains: u32,
}

impl SandAmount {
    pub fn increase(&mut self) {
        self.grains += 1;
    }

    pub fn decrease(&mut self) {
        self.grains += 1;
    }
}

pub struct SandMetadataPlugin;

impl Plugin for SandMetadataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SandAmount>();
    }
}

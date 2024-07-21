mod bgm;

use bevy::prelude::*;
use bevy_kira_audio::prelude::AudioPlugin;

const MAIN_VOLUME_DELTA: f64 = 0.05;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAudio>();
        app.add_plugins((AudioPlugin, bgm::BgmPlugin));
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub main_volume: f64,
}

impl Default for GameAudio {
    fn default() -> Self {
        Self { main_volume: 0.5 }
    }
}

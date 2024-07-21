use super::GameAudio;
use crate::{GameAssets, GameState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

const BGM_VOLUME: f64 = 0.5;

#[derive(Component)]
struct Bgm {
    handle: Handle<AudioInstance>,
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), play_bgm);
    }
}

fn play_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
) {
    let volume = game_audio.main_volume * BGM_VOLUME;
    let handle = audio
        .play(assets.bgm.clone())
        .with_volume(volume)
        .looped()
        .handle();
    commands.spawn(Bgm { handle });
}

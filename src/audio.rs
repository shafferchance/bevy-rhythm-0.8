use crate::{types::SongConfig, consts::AppState, time::ControlledTime};
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    // Soing starts 3 seconds after real time
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 3. && 3. <= secs {
        audio.play(config.song_audio.clone());
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(start_song));
    }
}
use bevy::{prelude::*, app::AppExit};

mod arrows;
mod consts;
mod types;
mod ui;
mod score;
mod audio;
mod shaders;
mod menu;
mod time;
mod map_maker;

use audio::AudioPlugin;
use consts::AppState;
use map_maker::MapMakerPlugin;
use menu::MenuPlugin;
use shaders::ShadersPlugin;
use ui::UIPlugin;
use arrows::ArrowsPlugins;
use score::ScoreResource;
use time::TimePlugin;

fn fire_on_exit(mut app_exit_events: EventWriter<AppExit>, input: Res<Input<KeyCode>>) {
     if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
     }
}

fn setup_ui_and_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);
    let camera = Camera2dBundle::default();

    println!("{:?}", &camera.global_transform.translation());

    commands
        .spawn_bundle(camera);
    
    commands
        .insert_resource(config);
}

fn main() {
    App::new()
        .init_resource::<ScoreResource>()
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup_ui_and_config)
        .add_system(fire_on_exit)
        .insert_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        // Changed 0.4 -> 0.5
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins) // Expands to CorePlugin, InputPlugin, and WindowPlugin
        .add_plugin(ArrowsPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(MapMakerPlugin)
        .run();
}

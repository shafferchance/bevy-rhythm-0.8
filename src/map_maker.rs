use crate::time::ControlledTime;
use crate::consts::*;
use crate::types::{
    ArrowTimeToml,
    Directions::{self, *},
    Speed
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*
};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Debug, Default)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>
) {
    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        if direction.key_jest_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml { 
                click_time: time.seconds_since_startup(),
                speed: Speed::Slow,
                direction: *direction
            });
        }
    }
}

fn save_to_file_on_exit(
    mut event_reader: EventReader<AppExit>,
    presses: Res<Presses>
) {
    for _event in event_reader.iter() {
        println!("Trying to write");
        let text = toml::to_string(&*presses).expect("Couldn't convert to toml text");
        
        let mut file = File::create("map.toml").expect("Couldn't open map.toml");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml");
    }
}

#[derive(Component)]
struct MapMakerAudio(Handle<AudioSource>);

impl FromWorld for MapMakerAudio {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let audio = asset_server.load("map_maker_song.mp3");
        Self(audio)
    }
}

fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}

#[derive(Component)]
struct MapMakerArrow(Directions);

fn setup_map_maker_arrows(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>
) {
    let border_handle = materials.add(asset_server.load("images/arrow_border.png").into());

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        let y = match direction {
            Up => 150.,
            Down => 50.,
            Left => -50.,
            Right => -150.,
        };

        let mut transform = Transform::from_translation(Vec3::new(0., y, 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                material: border_handle.clone(),
                mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(140., 140.)))).into(),
                transform,
                ..Default::default()
            })
            .insert(MapMakerArrow(*direction));
    }
}

fn toggle_map_maker_arrows(
    mut query: Query<(&mut Visibility, &MapMakerArrow)>,
    keyboard_input: Res<Input<KeyCode>>
) {
    for (mut visible, arrow) in query.iter_mut() {
        visible.is_visible = arrow.0.key_pressed(&keyboard_input);
    }
}

pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Presses>()
           .init_resource::<MapMakerAudio>()
           .add_system_set(
                SystemSet::on_enter(AppState::MakeMap)
                    .with_system(setup_map_maker_arrows)
                    .with_system(start_song)
           )
           .add_system_set(
                SystemSet::on_update(AppState::MakeMap)
                    .with_system(toggle_map_maker_arrows)
                    .with_system(save_key_presses)
                    .with_system(save_to_file_on_exit)
           );
    }
}

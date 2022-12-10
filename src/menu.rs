use std::{fs::read_dir, fmt::format};

use crate::{consts::*, types::load_config};
use bevy::prelude::*;

struct ButtonMaterials {
    none: UiColor,
    normal: UiColor,
    hovered: UiColor,
    pressed: UiColor,
    font: Handle<Font>,
}

fn get_font(world: &World) -> Handle<Font> {
    world.get_resource::<AssetServer>().unwrap().load("fonts/FiraSans-Bold.ttf")
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let font = get_font(world);

        ButtonMaterials { 
            none: Color::NONE.into(),
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            font
        }
    }
}

#[derive(Component)]
struct MenuUI;

fn setup_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    let mut buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();
    buttons.push(MenuButton::MakeMap);

    commands.spawn_bundle(
        NodeBundle {
            style: Style { 
                display: Display::Flex,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: button_materials.none,
            ..Default::default()
        }
    )
    .insert(MenuUI)
    .with_children(|parent| {
        for button in buttons {
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size { width: Val::Px(350.), height: Val::Px(65.0) },
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: button_materials.normal,
                ..Default::default()
            })
            .with_children(|parent_button| {
                parent_button.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        button.name(),
                        TextStyle {
                            font: button_materials.font.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        }).with_alignment(TextAlignment::CENTER),
                    ..Default::default()
                });
            })
            .insert(button);
        }
    });
}

fn tear_down_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::MakeMap => state
                    .set(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                MenuButton::PlaySong(song) => {
                    let config = load_config(&*format!("{}.toml", song), &asset_server);
                    commands.insert_resource(config);
                    state.set(AppState::Game)
                         .expect("Couldn't switch to state Game");
                }
            };
        }
    }
}

fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}

pub fn get_songs() -> Vec<String> {
    let paths = read_dir("assets/songs").unwrap();

    let mut vec = vec![];
    for path in paths {
        let path = path.unwrap().path();

        if "toml" == path.as_path().extension().unwrap() {
            vec.push(
                path.as_path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            );
        }
    }
    vec
}

#[derive(Component)]
enum MenuButton {
    MakeMap,
    PlaySong(String),
}

impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::MakeMap => "Make map".to_string(),
            Self::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>()
           .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    .with_system(setup_menu)
           )
           .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(button_color_system)
                    .with_system(button_press_system)
           )
           .add_system_set(
                SystemSet::on_exit(AppState::Menu)
                    .with_system(tear_down_menu)
           );
    }
}
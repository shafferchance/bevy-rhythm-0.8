use bevy::prelude::*;
use crate::{ScoreResource, consts::AppState};

#[derive(Component)]
struct TimeText;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    }).add_children(|parent| {
        parent
            .spawn_bundle(TextBundle::from_section(
                "Time: 0.0",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ))
            .insert(TimeText);
        }
    );

    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.),
                bottom: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    }).add_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Score: 0. Corrects: 0. Fails: 0",
            TextStyle {
                font: font.clone(),
                color: Color::rgb(0.8, 0.8, 0.8),
                font_size: 40.0,
                ..Default::default()
            },
        ))
        .insert(ScoreText);
    });
    
}

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, With<TimeText>)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

#[derive(Component)]
struct ScoreText;

fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, With<ScoreText>)>) {
    if !score.is_changed() {
        return;
    }
    
    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}. Corrects: {}. Fails: {}",
            score.score(),
            score.corrects(),
            score.fails()
        );
    }
}

pub  struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(setup_ui)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(update_time_text)
                    .with_system(update_score_text)   
            );
    }
}

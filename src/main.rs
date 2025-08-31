// #![allow(dead_code)]

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    cube::{IsCubeSolved, check_cube_solved},
    cubie::spawn_cubies,
    mouse::{MousePressed, handle_mouse_drag},
    rotation::{Rotation, RotationTimer, Rotations, apply_rotations},
    ui::{setup_ui, update_cube_solved_indicator},
};

mod cube;
mod cubie;
mod mouse;
mod rotation;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".into(),
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy".into()), // use the <canvas id="bevy">
                        fit_canvas_to_parent: true,   // fill parent size
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                ui::scene_button_system,
                ui::cube_control_button_system,
                handle_mouse_drag,
                apply_rotations,
                check_cube_solved,
                update_cube_solved_indicator,
                handle_play_mode,
            ),
        )
        .run();
}

/// Setup the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // spawn cube
    spawn_cubies(&mut commands, &mut meshes, &mut materials);

    // spawn lights
    spawn_lights(&mut commands);

    // spawn camera
    commands.spawn((Camera3d::default(), camera_start_position()));

    // insert resources
    commands.insert_resource(IsCubeSolved(true));
    commands.insert_resource(MousePressed(false));
    commands.insert_resource(RotationTimer::new());
    commands.insert_resource(Rotations::new(None, VecDeque::new()));
    commands.insert_resource(PlayMode::default());

    // setup UI
    setup_ui(commands, &asset_server);
}

pub fn camera_start_position() -> Transform {
    Transform::from_xyz(10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y)
}

fn spawn_lights(commands: &mut Commands) {
    // spawn lights
    let light_distance = 5.;
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(light_distance, light_distance, light_distance),
    ));
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-light_distance, -light_distance, -light_distance),
    ));
}

#[derive(Debug, Default, Resource)]
pub enum PlayMode {
    #[default]
    None,
    Shuffle,
    Solve,
}

fn handle_play_mode(play_mode: Res<PlayMode>, mut rotations: ResMut<Rotations>) {
    match &*play_mode {
        PlayMode::None => {}
        PlayMode::Shuffle => {
            if rotations.is_queue_empty() && rotations.current_remaining() == 0.0 {
                rotations.enqueue(Rotation::random());
            }
        }
        PlayMode::Solve => {}
    }
}

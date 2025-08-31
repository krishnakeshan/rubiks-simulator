use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{
    PlayMode, camera_start_position,
    cube::{Face, IsCubeSolved},
    rotation::{Direction, Rotation, Rotations},
};

#[derive(Debug, Component)]
pub enum ButtonType {
    ResetCamera,
    Shuffle,
    Solve,
}

/// Setup the UI :D
pub fn setup_ui(mut commands: Commands, asset_server: &AssetServer) {
    let ui = (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            column_gap: Val::Px(10.),
            ..default()
        },
        children![
            filler(),
            cube_solved_indicator(),
            cube_controls(&asset_server),
            toolbar(),
        ],
    );

    commands.spawn(ui);
}

/// Controls for rotating the different slices of the cube.
fn cube_controls(asset_server: &AssetServer) -> impl Bundle {
    let top = cube_control_button_pair(
        Face::Top,
        (
            GridPlacement::start_span(2, 3),
            GridPlacement::start_span(2, 1),
        ),
        FlexDirection::Row,
        asset_server,
    );
    let bottom = cube_control_button_pair(
        Face::Bottom,
        (
            GridPlacement::start_span(2, 3),
            GridPlacement::start_span(6, 1),
        ),
        FlexDirection::Row,
        asset_server,
    );
    let left = cube_control_button_pair(
        Face::Left,
        (
            GridPlacement::start_span(1, 1),
            GridPlacement::start_span(3, 3),
        ),
        FlexDirection::ColumnReverse,
        asset_server,
    );
    let right = cube_control_button_pair(
        Face::Right,
        (
            GridPlacement::start_span(5, 1),
            GridPlacement::start_span(3, 3),
        ),
        FlexDirection::ColumnReverse,
        asset_server,
    );
    let front = cube_control_button_pair(
        Face::Front,
        (
            GridPlacement::start_span(2, 3),
            GridPlacement::start_span(1, 1),
        ),
        FlexDirection::Row,
        asset_server,
    );
    let back = cube_control_button_pair(
        Face::Back,
        (
            GridPlacement::start_span(2, 3),
            GridPlacement::start_span(7, 1),
        ),
        FlexDirection::Row,
        asset_server,
    );
    let horizontal_center = cube_control_button_pair(
        Face::HorizontalCentre,
        (
            GridPlacement::start_span(2, 3),
            GridPlacement::start_span(4, 1),
        ),
        FlexDirection::Row,
        asset_server,
    );
    let vertical_center = cube_control_button_pair(
        Face::VerticalCentre,
        (
            GridPlacement::start_span(3, 1),
            GridPlacement::start_span(3, 3),
        ),
        FlexDirection::ColumnReverse,
        asset_server,
    );

    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            top: Val::Percent(50.0),
            width: Val::Vw(20.0),
            aspect_ratio: Some(1.0),
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::fr(5, 1.)],
            grid_template_rows: vec![RepeatedGridTrack::fr(7, 1.)],
            ..default()
        },
        children![
            top,
            bottom,
            left,
            right,
            front,
            back,
            horizontal_center,
            vertical_center,
        ],
    )
}

/// A pair of buttons to rotate a cube slice in the forward and backward directions.
fn cube_control_button_pair(
    face: Face,
    position: (GridPlacement, GridPlacement),
    flex_direction: FlexDirection,
    asset_server: &AssetServer,
) -> impl Bundle {
    let label = match &face {
        Face::HorizontalCentre | Face::VerticalCentre => String::new(),
        other => other.to_string(),
    };

    // let directions: [Direction; 2] = match FlexDirection {
    //     FlexDirection::Row => [Direction::Backward, Direction::Forward],
    //     FlexDirection::Column => [Direction::Forward, Direction::Backward],
    // }

    (
        Node {
            display: Display::Flex,
            flex_direction,
            border: UiRect::all(Val::Px(2.0)),
            grid_column: position.0,
            grid_row: position.1,
            ..default()
        },
        BorderColor(Color::WHITE),
        children![
            cube_control_button(face.clone(), Direction::Backward, asset_server),
            (
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(Text(label), TextFont::from_font_size(14.0),)]
            ),
            cube_control_button(face, Direction::Forward, asset_server),
        ],
    )
}

#[derive(Component)]
pub struct CubeControlButton {
    face: Face,
    rotate_direction: Direction,
}

impl CubeControlButton {
    pub fn new(face: Face, rotate_direction: Direction) -> Self {
        Self {
            face,
            rotate_direction,
        }
    }

    pub fn rotation(&self) -> Rotation {
        Rotation::new(self.face.clone(), self.rotate_direction.clone())
    }
}

/// An individual cube control button that rotates a cube slice in a fixed direction.
fn cube_control_button(
    face: Face,
    rotate_direction: Direction,
    asset_server: &AssetServer,
) -> impl Bundle {
    let arrow_image = asset_server.load("arrow.png");
    let arrow_rotation_radians = arrow_rotation(&face, &rotate_direction);

    (
        Button,
        CubeControlButton::new(face, rotate_direction),
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        children![(
            Node { ..default() },
            children![(
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    ..default()
                },
                ImageNode::new(arrow_image).with_color(Color::WHITE),
                Transform::from_rotation(Quat::from_rotation_z(arrow_rotation_radians)),
            )],
        )],
    )
}

/// Handle cube control button interactions.
pub fn cube_control_button_system(
    mut interaction_query: Query<
        (&Interaction, &CubeControlButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut rotations: ResMut<Rotations>,
) {
    for (interaction, cube_control_button, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::None => {
                background_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
            }
            Interaction::Pressed => {
                rotations.enqueue(cube_control_button.rotation());
            }
        }
    }
}

fn arrow_rotation(face: &Face, direction: &Direction) -> f32 {
    match face {
        Face::Top | Face::Bottom | Face::Front | Face::Back | Face::HorizontalCentre => {
            match direction {
                Direction::Forward => FRAC_PI_2,
                Direction::Backward => -FRAC_PI_2,
            }
        }
        Face::Left | Face::Right | Face::VerticalCentre => match direction {
            Direction::Forward => 0.0,
            Direction::Backward => PI,
        },
    }
}

/// Bottom toolbar for buttons like 'shuffle', 'solve', etc.
fn toolbar() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.),
            padding: UiRect::all(Val::Px(25.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            button("Reset Camera", ButtonType::ResetCamera),
            button("Shuffle", ButtonType::Shuffle),
            (
                Node {
                    display: Display::None,
                    ..default()
                },
                children![button("Solve", ButtonType::Solve)],
            ),
        ],
    )
}

/// Indicator for whether or not the cube is solved in its current state.
#[derive(Component)]
pub struct CubeSolvedIndicator;

fn cube_solved_indicator() -> impl Bundle {
    (
        CubeSolvedIndicator,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(25.0),
            margin: UiRect::horizontal(Val::Auto),
            padding: UiRect::axes(Val::Px(10.), Val::Px(5.)),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        BorderRadius::all(Val::Px(10.)),
        children![(Text::new("Solved"), TextColor(Color::WHITE),)],
    )
}

pub fn update_cube_solved_indicator(
    is_cube_solved: Res<IsCubeSolved>,
    mut indicator: Query<(&mut BackgroundColor, &Children), With<CubeSolvedIndicator>>,
    mut text_query: Query<&mut Text>,
) {
    let (mut background_color, children) = indicator.single_mut().unwrap();
    let mut solved_text = text_query.get_mut(children[0]).unwrap();
    *solved_text = if is_cube_solved.0 {
        *background_color = BackgroundColor(Color::srgb(0.0, 0.7, 0.0));
        Text::new("Solved")
    } else {
        *background_color = BackgroundColor(Color::srgb(0.7, 0.0, 0.0));
        Text::new("Not solved")
    }
}

/// A button with the given text.
fn button(text: &'static str, button_type: ButtonType) -> impl Bundle {
    (
        Button,
        button_type,
        Node {
            height: Val::Px(60.),
            border: UiRect::all(Val::Px(5.)),
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(50.)),
        children![(Text::new(text), TextColor(Color::WHITE))],
    )
}

/// Handle scene button interactions.
pub fn scene_button_system(
    mut interaction_query: Query<
        (&ButtonType, &Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut camera_query: Single<&mut Transform, With<Camera>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
    mut play_mode: ResMut<PlayMode>,
) {
    for (button_type, interaction, mut background_color, children) in &mut interaction_query {
        let (mut text, mut text_color) = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::None => {
                *background_color = Color::NONE.into();
                *text_color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                *background_color = Color::WHITE.into();
                *text_color = Color::BLACK.into();
            }
            Interaction::Pressed => match button_type {
                ButtonType::ResetCamera => {
                    **camera_query = camera_start_position();
                }
                ButtonType::Shuffle => {
                    handle_shuffle_press(&mut play_mode, &mut text);
                }
                ButtonType::Solve => {
                    handle_solve_press(&mut play_mode, &mut text);
                }
            },
        }
    }
}

/// Handles the 'shuffle' button being pressed
fn handle_shuffle_press(play_mode: &mut PlayMode, button_text: &mut Text) {
    match play_mode {
        PlayMode::Shuffle => {
            *play_mode = PlayMode::None;
            *button_text = Text::new("Shuffle");
        }
        // going from None to Shuffle
        PlayMode::None => {
            *play_mode = PlayMode::Shuffle;
            *button_text = Text::new("Stop shuffling");
        }
        _ => {}
    }
}

/// Handles the 'solve' button being pressed
fn handle_solve_press(play_mode: &mut PlayMode, button_text: &mut Text) {
    match play_mode {
        PlayMode::Solve => {
            *play_mode = PlayMode::None;
            *button_text = Text::new("Solve");
        }
        // going from None to Solve
        PlayMode::None => {
            *play_mode = PlayMode::Solve;
            *button_text = Text::new("Stop solving");
        }
        _ => {}
    }
}

/// A filler item that just grows into any flex box empty space.
fn filler() -> Node {
    Node {
        flex_grow: 1.,
        ..default()
    }
}

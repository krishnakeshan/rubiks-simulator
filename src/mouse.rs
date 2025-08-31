use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseButtonInput},
    prelude::*,
};

#[derive(Resource)]
pub struct MousePressed(pub bool);

impl MousePressed {
    pub fn set_pressed(&mut self, is_pressed: bool) {
        if self.0 != is_pressed {
            self.0 = is_pressed;
        }
    }
}

/// Pan the screen, effectively rotating the cube when the mouse is dragged
pub fn handle_mouse_drag(
    mut mouse_pressed: ResMut<MousePressed>,
    mut button_events: EventReader<MouseButtonInput>,
    motion_events: Res<AccumulatedMouseMotion>,
    mut camera_transform: Single<&mut Transform, With<Camera>>,
) {
    // store whether left mouse button is pressed or not
    for button_event in button_events.read() {
        if button_event.button == MouseButton::Left {
            mouse_pressed.set_pressed(button_event.state.is_pressed());
        }
    }

    // if mouse is pressed, handle motion events
    if mouse_pressed.0 {
        if motion_events.delta != Vec2::ZERO {
            let x_displacement = motion_events.delta.x;
            let y_displacement = motion_events.delta.y;

            if x_displacement.abs() > y_displacement.abs() {
                let y_rotation = Quat::from_rotation_y(-x_displacement / 75.);
                camera_transform.rotate_around(Vec3::ZERO, y_rotation);
            } else {
                let x_rotation = Quat::from_rotation_x(-y_displacement / 75.);
                camera_transform.rotate_around(Vec3::ZERO, x_rotation);
            }
        }
    }
}

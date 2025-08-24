use std::{collections::VecDeque, f32::consts::FRAC_PI_2};

use bevy::prelude::*;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::{cube::Face, cubie::Cubie};

pub const ONE_ROTATION_RADIANS: f32 = FRAC_PI_2;
pub const ROTATION_SPEED: f32 = 2.0;

/// The direction in which a cube face should be rotated
#[derive(Debug, Clone)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    pub fn variants() -> [Self; 2] {
        [Self::Forward, Self::Backward]
    }

    pub fn signum(&self) -> f32 {
        match self {
            Self::Forward => 1.,
            Self::Backward => -1.,
        }
    }
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.random_range(0..2) {
            1 => Direction::Forward,
            _ => Direction::Backward,
        }
    }
}

/// Describes a cube face rotation as a combination of the face to be rotated and the rotation direction
#[derive(Clone)]
pub struct Rotation {
    face: Face,
    direction: Direction,
}

impl Rotation {
    pub fn new(face: Face, direction: Direction) -> Self {
        Self { face, direction }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let direction: Direction = rng.random();
        let face: Face = rng.random();
        Self::new(face, direction)
    }
}

/// Adds some time between rotations so they're not too fast.
#[derive(Resource)]
pub struct RotationTimer(pub Timer);

impl RotationTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct Rotations {
    current: Option<Rotation>,
    current_remaining: f32,
    queue: VecDeque<Rotation>,
}

impl Rotations {
    pub fn new(in_progress: Option<Rotation>, queue: VecDeque<Rotation>) -> Self {
        let current_remaining = if let Some(in_progress) = &in_progress {
            ONE_ROTATION_RADIANS * in_progress.direction.signum()
        } else {
            0.0
        };

        Self {
            current: in_progress,
            current_remaining,
            queue,
        }
    }

    pub fn current_remaining(&self) -> f32 {
        self.current_remaining
    }

    pub fn progress_current_rotation(&mut self, progress: f32) {
        self.current_remaining -= progress;
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn enqueue(&mut self, rotation: Rotation) {
        self.queue.push_back(rotation);
    }

    /// Load the next rotation from the queue into `self.current`.
    pub fn load_next_rotation(&mut self) {
        self.current = self.queue.pop_front();
        if let Some(rotation) = &self.current {
            self.current_remaining = ONE_ROTATION_RADIANS * rotation.direction.signum();
        }
    }
}

pub fn apply_rotations(
    time: Res<Time>,
    mut rotation_timer: ResMut<RotationTimer>,
    mut rotations: ResMut<Rotations>,
    mut cubie_transforms: Query<&mut Transform, With<Cubie>>,
) {
    // progress the rotation currently in progress
    if let Some(current_rotation) = &rotations.current {
        let face_normal = current_rotation.face.normal();
        let is_center_slice = current_rotation.face.is_center();
        let step = (ONE_ROTATION_RADIANS * time.delta_secs() * ROTATION_SPEED)
            .min(rotations.current_remaining.abs())
            * rotations.current_remaining.signum();

        // rotate eligible cubies
        for mut cubie_transform in &mut cubie_transforms {
            if should_rotate_cubie(&cubie_transform.translation, face_normal, is_center_slice) {
                cubie_transform
                    .rotate_around(face_normal, Quat::from_axis_angle(face_normal, step));

                // if the rotation just completed, snap the cubie to the grid
                if rotations.current_remaining == step {
                    snap_cubie_to_grid(&mut cubie_transform);
                }
            }
        }
        rotations.progress_current_rotation(step);
    }

    // check if the current rotation has completed
    if rotations.current_remaining == 0.0
        && rotation_timer.0.tick(time.delta()).just_finished()
        && !rotations.is_queue_empty()
    {
        rotations.load_next_rotation();
    }
}

fn should_rotate_cubie(translation: &Vec3, axis: Vec3, is_center_slice: bool) -> bool {
    if is_center_slice {
        let dot = translation.dot(axis);
        dot <= 0.5 && dot >= 0.0
    } else {
        translation.dot(axis) >= 1.0
    }
}

/// Snaps the given cubie `Transform` to the 'grid'.
/// Being on the grid means having coordinates in [-1,0,1] i.e. no floating point components.
fn snap_cubie_to_grid(cubie: &mut Transform) {
    cubie.translation = snapped_translation(&cubie.translation);
    cubie.rotation = snapped_rotation(&cubie.rotation);
}

/// Snap the provided translation `Vec3` to {-1,0,1}
fn snapped_translation(translation: &Vec3) -> Vec3 {
    translation.map(|coordinate| {
        if coordinate.abs() < 0.5 {
            0.0
        } else {
            coordinate.signum()
        }
    })
}

/// Snap the provided rotation `Quat` to the nearest 90 degrees (PI/2 radians)
fn snapped_rotation(rotation: &Quat) -> Quat {
    let (mut x, mut y, mut z) = rotation.to_euler(EulerRot::XYZ);

    let step = FRAC_PI_2;
    x = (x / step).round() * step;
    y = (y / step).round() * step;
    z = (z / step).round() * step;

    Quat::from_euler(EulerRot::XYZ, x, y, z)
}

#[cfg(test)]
mod test {
    use bevy::math::Vec3;

    #[test]
    fn test_dot_product() {
        let cubie_face = Vec3::new(1.49, 1.0, 1.0);
        assert_eq!(Vec3::X.dot(cubie_face), 1.0);
    }
}

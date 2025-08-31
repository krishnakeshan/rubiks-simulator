use bevy::prelude::*;
use rand::distr::{Distribution, StandardUniform};

use crate::{
    cubie::{CUBIE_FACE_OFFSET, CubieFace},
    rotation::RotationTimer,
};

#[derive(Clone, Debug, Component)]
pub enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
    HorizontalCentre, // the centre slice that is horizontal
    VerticalCentre,   // the centre slice that is vertical
}

impl Face {
    pub fn is_center(&self) -> bool {
        match self {
            Self::HorizontalCentre | Self::VerticalCentre => true,
            _ => false,
        }
    }

    /// A flat cube face is a face that lies on one plane.
    /// Essentially it is all the faces except the centre ones.
    pub fn flat_faces() -> Vec<Self> {
        vec![
            Self::Top,
            Self::Bottom,
            Self::Left,
            Self::Right,
            Self::Front,
            Self::Back,
        ]
    }

    /// The unit vector representing the normal for a cube face.
    pub fn normal(&self) -> Vec3 {
        match self {
            Self::Top => Vec3::Y,
            Self::Bottom => -Vec3::Y,
            Self::Left => -Vec3::X,
            Self::Right => Vec3::X,
            Self::Front => Vec3::Z,
            Self::Back => -Vec3::Z,
            Self::HorizontalCentre => Vec3::Y,
            Self::VerticalCentre => Vec3::X,
        }
    }

    pub fn to_string(&self) -> String {
        let s = match self {
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Front => "Front",
            Self::Back => "Back",
            Self::HorizontalCentre => "Horizontal Center",
            Self::VerticalCentre => "Vertical Center",
        };

        s.to_string()
    }
}

impl Distribution<Face> for StandardUniform {
    /// Get a random `Face`
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.random_range(0..8) {
            1 => Face::Top,
            2 => Face::Bottom,
            3 => Face::Left,
            4 => Face::Right,
            5 => Face::Front,
            6 => Face::Back,
            7 => Face::HorizontalCentre,
            _ => Face::VerticalCentre,
        }
    }
}

#[derive(Resource)]
pub struct IsCubeSolved(pub bool);

/// Check whether the cube is in a solved state and update the `IsCubeSolved` resource.
pub fn check_cube_solved(
    mut is_cube_solved: ResMut<IsCubeSolved>,
    rotation_timer: Res<RotationTimer>,
    cubie_faces: Query<(&GlobalTransform, &MeshMaterial3d<StandardMaterial>), With<CubieFace>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    if rotation_timer.0.finished() {
        // check if all cubie faces on a given cube face have the same color
        let mut cube_solved = true;
        let cubie_faces = cubie_faces.iter().collect::<Vec<_>>();
        for face in Face::flat_faces() {
            if !are_all_colors_on_face_same(&face, &cubie_faces, &materials) {
                cube_solved = false;
                break;
            }
        }

        is_cube_solved.0 = cube_solved;
    }
}

/// Check whether a single face is in the solved state.
fn are_all_colors_on_face_same(
    face: &Face,
    cubie_faces: &Vec<(&GlobalTransform, &MeshMaterial3d<StandardMaterial>)>,
    materials: &Res<Assets<StandardMaterial>>,
) -> bool {
    let normal = face.normal();

    // a color chosen at random from the face being checked
    // this is used to assert that all other colors on this cube face are the same
    let mut sample_color: Option<Color> = None;

    for (face_transform, material_3d) in cubie_faces {
        if normal.dot(face_transform.translation()) == 1. + CUBIE_FACE_OFFSET {
            if let Some(material) = materials.get(&material_3d.0) {
                let color = material.base_color;

                // a sample color has already been picked, just compare
                if let Some(sample_color) = sample_color {
                    if sample_color != color {
                        return false;
                    }
                }
                // sample color needs to be picked
                else {
                    sample_color = Some(color);
                }
            }
        }
    }

    true
}

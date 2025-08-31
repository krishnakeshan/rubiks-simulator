use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct Cubie;

pub const CUBIE_FACE_OFFSET: f32 = 0.49;

pub fn spawn_cubies(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if let Some(kind) = Kind::from_coordinates(x, y, z) {
                    // spawn parent cubie to anchor faces
                    let cubie =
                        CubieBundle::new(kind, Transform::from_xyz(x as f32, y as f32, z as f32));

                    // spawn cubie faces
                    commands.spawn(cubie.clone()).with_children(|parent| {
                        let cubie_face_half_size = Vec2::new(CUBIE_FACE_OFFSET, CUBIE_FACE_OFFSET);
                        for face in Face::variants() {
                            let normal = face.normal();
                            let transform = Transform::from_translation(normal * CUBIE_FACE_OFFSET);
                            parent.spawn((
                                CubieFace,
                                Mesh3d(
                                    meshes.add(Plane3d::new(normal, cubie_face_half_size.clone())),
                                ),
                                MeshMaterial3d(materials.add(face.start_color().color())),
                                transform,
                                GlobalTransform::IDENTITY,
                            ));
                        }
                    });
                }
            }
        }
    }
}

#[derive(Component)]
pub struct CubieFace;

#[derive(Clone, Debug)]
pub enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    pub fn variants() -> [Face; 6] {
        [
            Self::Top,
            Self::Bottom,
            Self::Left,
            Self::Right,
            Self::Front,
            Self::Back,
        ]
    }

    pub fn normal(&self) -> Vec3 {
        match self {
            Self::Top => Vec3::Y,
            Self::Bottom => -Vec3::Y,
            Self::Left => -Vec3::X,
            Self::Right => Vec3::X,
            Self::Front => Vec3::Z,
            Self::Back => -Vec3::Z,
        }
    }

    /// Returns the starting color for a cube face in the solved state
    /// For example, in a solved cube with white on top and red facing the user, blue is on the right and green is on the left, etc.
    pub fn start_color(&self) -> FaceColor {
        match self {
            Self::Top => FaceColor::White,
            Self::Bottom => FaceColor::Yellow,
            Self::Left => FaceColor::Green,
            Self::Right => FaceColor::Blue,
            Self::Front => FaceColor::Red,
            Self::Back => FaceColor::Orange,
        }
    }
}

#[derive(Component)]
pub enum FaceColor {
    Orange,
    Red,
    White,
    Yellow,
    Blue,
    Green,
}

impl FaceColor {
    pub fn color(&self) -> Color {
        match self {
            Self::Orange => Color::srgb_u8(255, 88, 0),
            Self::Red => Color::srgb_u8(255, 0, 0),
            Self::White => Color::srgb_u8(255, 255, 255),
            Self::Yellow => Color::srgb_u8(255, 213, 0),
            Self::Blue => Color::srgb_u8(0, 0, 255),
            Self::Green => Color::srgb_u8(0, 255, 0),
        }
    }
}

#[derive(Clone, Component)]
pub enum Kind {
    Centre,
    Corner,
    Edge,
}

impl Kind {
    /// Gets the `CubieKind` given a set of `x`, `y`, and `z` coordinates.
    pub fn from_coordinates(x: i8, y: i8, z: i8) -> Option<Self> {
        let (x_abs, y_abs, z_abs) = (x.abs(), y.abs(), z.abs());
        let abs_sum = x_abs + y_abs + z_abs;
        if x == 0 && y == 0 && z == 0 {
            return None;
        } else if abs_sum == 1 {
            return Some(Self::Centre);
        } else if x.abs() == y.abs() && x.abs() == z.abs() {
            return Some(Self::Corner);
        }

        return Some(Kind::Edge);
    }
}

#[derive(Clone, Bundle)]
pub struct CubieBundle {
    visibility: Visibility,
    cubie: Cubie,
    kind: Kind,
    transform: Transform,
}

impl CubieBundle {
    pub fn new(kind: Kind, transform: Transform) -> Self {
        Self {
            visibility: Visibility::default(),
            cubie: Cubie,
            kind,
            transform,
        }
    }
}

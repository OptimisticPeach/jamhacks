use bevy::prelude::*;
use crate::{Handle, Mesh, Res, StandardMaterial, Vec3};

#[derive(Copy, Clone, Debug, PartialEq)]
struct SavedPlanet {
    pos: Vec3,
    mass: f32,
    radius: f32,
}

macro_rules! make_planet {
    ($pos:expr, $mass:expr, $rad:expr) => {
        SavedPlanet {
            pos: $pos,
            mass: $mass,
            radius: $rad,
        }
    }
}

struct PointOfInterest {
    planet: usize,
    direction: Vec3,
}

impl PointOfInterest {
    fn resolve(&self, level: &Level, len: f32) -> Vec3 {
        level.planets[self.planet].pos + self.direction.normalize() * (level.planets[self.planet].radius + len)
    }
}


struct CameraView(Vec3, Vec3);

struct Level {
    planets: Vec<SavedPlanet>,
    start: PointOfInterest,
    goal: PointOfInterest,
    // cameras: Vec<CameraView>,
}

lazy_static::lazy_static! {
    static ref LEVELS: Vec<Level> = vec![
        Level { //Level 1
            planets: vec![
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 600.0, 1.0),
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(1.0, 0.0, 1.0),
            },
            goal: PointOfInterest {
                planet: 0,
                direction: -Vec3::new(1.0, 2.0, 1.0),
            },
            // cameras: vec![
            //     CameraView(Vec3::new(5.0, 5.0, 5.0))
            // ]
        },
        Level { //Level 2
            planets: vec![
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 600.0, 1.0),
                make_planet!(Vec3::new(4.0, -1.0, -1.0), 600.0, 1.0),
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(1.0, 1.0, 1.0),
            },
            goal: PointOfInterest {
                planet: 0,
                direction: -Vec3::new(1.0, 1.0, 1.0),
            },
            // cameras: vec![
            //     CameraView(Vec3::new(5.0, 5.0, 5.0))
            // ]
        },
        Level { //Level 3
            planets: vec![
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 600.0, 1.0),
                make_planet!(Vec3::new(4.0, -1.0, -1.0), 600.0, 1.0),
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(1.0, 1.0, 1.0),
            },
            goal: PointOfInterest {
                planet: 0,
                direction: -Vec3::new(1.0, 1.0, 1.0),
            },
            // cameras: vec![
            //     CameraView(Vec3::new(5.0, 5.0, 5.0))
            // ]
        },
    ];
}

fn spawn_level<const LEVEL: usize>(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    player_mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    player_material: &Handle<StandardMaterial>,
    target_material: &Handle<StandardMaterial>,
) {
    for planet in &LEVELS[LEVEL].planets {
        crate::add_planet::<LEVEL>(
            commands,
            planet.pos,
            planet.mass,
            planet.radius,
            &mesh,
            &material,
        );
    }

    crate::add_ball::<LEVEL>(
        commands,
        LEVELS[LEVEL].start.resolve(&LEVELS[LEVEL], 0.3),
        1.0,
        0.2,
        &player_mesh,
        &player_material,
        None,
        LEVELS[LEVEL].planets[LEVELS[LEVEL].start.planet].pos
    );

    crate::add_target::<LEVEL>(
        commands,
        LEVELS[LEVEL].goal.resolve(&LEVELS[LEVEL], 0.2),
        &player_mesh,
        &target_material
    );
}

pub fn load<const LEVEL: usize>(
    mut commands: Commands,
    resources: Res<LoadResources>,
) {
    spawn_level::<LEVEL>(
        &mut commands,
        &resources.planet_mesh,
        &resources.player_mesh,
        &resources.planet_mat,
        &resources.player_mat,
        &resources.target_mat,
    )
}

pub struct LoadResources {
    pub(crate) planet_mesh: Handle<Mesh>,
    pub(crate) player_mesh: Handle<Mesh>,
    pub(crate) planet_mat: Handle<StandardMaterial>,
    pub(crate) player_mat: Handle<StandardMaterial>,
    pub(crate) target_mat: Handle<StandardMaterial>
}

#[derive(Component)]
pub struct LevelId<const X: usize>;

use bevy::prelude::*;
use crate::{Handle, Mesh, Res, StandardMaterial, Texture, Vec3};
use crate::setup::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct SavedPlanet {
    pos: Vec3,
    mass: f32,
    radius: f32,
    colour: Vec3,
}

macro_rules! make_planet {
    ($pos:expr, $mass:expr, $rad:expr, $colour:expr) => {
        SavedPlanet {
            pos: $pos,
            mass: $mass,
            radius: $rad,
            colour: *$colour

        }
    };
    ($pos:expr, $mass:expr, $rad:expr) => {
        SavedPlanet {
            pos: $pos,
            mass: $mass,
            radius: $rad,
            colour: *WHITE
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
}

lazy_static::lazy_static! {
    static ref RED: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    static ref GREEN: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    static ref BLUE: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    static ref WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    static ref BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    static ref PURPLE: Vec3 = Vec3::new(0.6, 0.0, 0.6);

    static ref LEVELS: Vec<Level> = vec![
        Level { //Level 1
            planets: vec![
                make_planet!(Vec3::new(0.0, 0.0, 0.0), 800.0, 1.0),
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
        Level { //Level 2
            planets: vec![
                make_planet!(Vec3::new(0.0, 0.0, 0.0), 800.0, 1.3, PURPLE),
                make_planet!(Vec3::new(3.5, 1.0, 1.5), 750.0, 0.9, PURPLE),
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(0.0, 1.0, 0.0),
            },
            goal: PointOfInterest {
                planet: 1,
                direction: -Vec3::new(0.0, 1.0, 0.),
            },
            // cameras: vec![
            //     CameraView(Vec3::new(5.0, 5.0, 5.0))
            // ]
        },
        Level { //Level 3
            planets: vec![
                make_planet!(Vec3::new(0.0, 0.0, 0.0), 80.0, 1.2, RED),
                make_planet!(Vec3::new(4.0, 1.0, 2.3), 1000.0, 1.6, RED),
                make_planet!(Vec3::new(3.0, 0.3, 5.7), 500.0, 0.75, RED),
                make_planet!(Vec3::new(0.4, 1.4, 3.0), -200.0, 0.3, GREEN), //anti-grav
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(0.0, 1.0, 0.0),
            },
            goal: PointOfInterest {
                planet: 2,
                direction: -Vec3::new(0.5, 1.0, 0.62),
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
    mut material: ResMut<Assets<StandardMaterial>>,
    player_material: &Handle<StandardMaterial>,
    target_material: &Handle<StandardMaterial>,
    texture: &Handle<Image>,
    nmap: &Handle<Image>,
) {
    for planet in &LEVELS[LEVEL].planets {
        crate::add_planet::<LEVEL>(
            commands,
            planet.pos,
            planet.mass,
            planet.radius,
            &mesh,
            material.add(StandardMaterial {
                base_color: Color::from(planet.colour.extend(1.0)).into(),
                base_color_texture: Some(texture.clone()),
                normal_map_texture: Some(nmap.clone()),
                perceptual_roughness: 0.5,
                ..default()
            }),
        );
    }

    crate::add_ball::<LEVEL>(
        commands,
        LEVELS[LEVEL].start.resolve(&LEVELS[LEVEL], BALL_RAD),
        1.0,
        BALL_RAD,
        &player_mesh,
        &player_material,
        None,
        LEVELS[LEVEL].planets[LEVELS[LEVEL].start.planet].pos
    );

    crate::add_target::<LEVEL>(
        commands,
        LEVELS[LEVEL].goal.resolve(&LEVELS[LEVEL], BALL_RAD),
        &player_mesh,
        &target_material
    );
}

pub fn load<const LEVEL: usize>(
    mut commands: Commands,
    resources: Res<LoadResources>,
    assets: ResMut<Assets<StandardMaterial>>
) {
    spawn_level::<LEVEL>(
        &mut commands,
        &resources.planet_mesh,
        &resources.player_mesh,
        assets,
        &resources.player_mat,
        &resources.target_mat,
        &resources.planet_texture,
        &resources.planet_nmap
    )
}

pub struct LoadResources {
    pub(crate) planet_mesh: Handle<Mesh>,
    pub(crate) player_mesh: Handle<Mesh>,
    pub(crate) planet_texture: Handle<Image>,
    pub(crate) planet_nmap: Handle<Image>,
    pub(crate) player_mat: Handle<StandardMaterial>,
    pub(crate) target_mat: Handle<StandardMaterial>
}

#[derive(Component)]
pub struct LevelId<const X: usize>;

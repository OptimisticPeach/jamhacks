use bevy::prelude::Commands;
use crate::{Handle, Mesh, StandardMaterial, Vec3};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

struct Level {
    planets: Vec<SavedPlanet>,
    start: PointOfInterest,
    goal: PointOfInterest,
}

lazy_static::lazy_static! {
    static ref LEVELS: &[Level] = &[
        Level {
            planets: vec![
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 300.0, 1.0),
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 300.0, 1.0),
            ],
            start: PointOfInterest {
                planet: 0,
                direction: Vec3::new(1.0, 1.0, 1.0),
            },
            goal: PointOfInterest {
                planet: 0,
                direction: -Vec3::new(1.0, 1.0, 1.0),
            }
        },
        Level {
            planets: vec![
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 300.0, 1.0),
                make_planet!(Vec3::new(1.0, 1.0, 1.0), 300.0, 1.0),
            ]
        },
    ];
}

fn spawn_level(
    commands: &mut Commands,
    level: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    for planet in &LEVELS[level].planets {
        crate::add_planet(
            commands,
            planet.pos,
            planet.mass,
            planet.radius,
            mesh.clone(),
            material.clone(),
        );
    }
}

/*
setup lvl1
    add_planet(
        &mut commands,
        Vec3::ZERO,
        200.0,
        2.0,
        &mesh,
        &material,

    add_ball(
        &mut commands,
        Vec3::new(4.0, 0.0, 0.0),
        3.0,
        0.1,
        &mesh,
        &material,
        Vec3::new(1.0, 1.0, 1.0) * 0.5,
setup lvl2

setup lvl3


 */

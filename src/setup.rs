use crate::{Handle, Mesh, StandardMaterial, Vec2, Vec3};
use bevy::prelude::*;
use dolly::prelude::*;

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const GRAVITY: f32 = 6.67e-2;

#[derive(Component)]
pub struct GravityAffected {
    pub mass: f32,
    pub radius: f32
}

#[derive(Component)]
pub struct Planet {
    pub mass: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct Dynamics {
    pub acc: Vec3,
    pub vel: Vec3,
}

#[derive(Component)]
pub struct CameraTag {
    pub(crate) rig: CameraRig
}

pub struct TargetedPlanet {
    pub pos: Vec3,
    pub radius: f32,
}

pub struct InputState {
    pub start: Option<Vec2>,
    pub cursor_pos: Vec2,
}

#[derive(Component)]
pub struct ProjectedSphere;

#[derive(Component)]
pub struct MainBall;

pub struct ProjectedResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct UIMainCamera;

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

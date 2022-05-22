//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use dolly::prelude::*;

mod setup;
use setup::*;
mod audio;
use audio::GameAudioPlugin;
mod start_menu;
use start_menu::MainMenuPlugin;
mod levels;
use levels::*;

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Game)
                .with_system(update_timestep)
                .with_system(update_acc)
                .with_system(update_pos_vel)
                .with_system(collide)
                .with_system(camera_movement)
                .with_system(aim)
            )
            .add_system_set(SystemSet::on_exit(GameState::Game)
                .with_system(despawn::<GameElement>)
            );
    }
}

struct MenuPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // todo, jerry
    }
}

fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameAudioPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        //game runningfl
        .run();
}

pub fn add_ball(
    commands: &mut Commands,
    pos: Vec3,
    mass: f32,
    radius: f32,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    vel: Vec3,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z)
            .with_scale(Vec3::splat(radius)),
        ..default()
    })
        .insert(GravityAffected {
            mass,
            radius,
        })
        .insert(Dynamics {
            acc: Vec3::ZERO,
            vel,
        })
        .insert(MainBall);
}

pub fn add_planet(
    commands: &mut Commands,
    pos: Vec3,
    mass: f32,
    radius: f32,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z)
            .with_scale(Vec3::splat(radius)),
        ..default()
    })
        .insert(Planet {
            mass,
            radius,
        })
        .insert(Dynamics {
            acc: Vec3::ZERO,
            vel: Vec3::ZERO,
        })
        .insert(GameElement);
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, sectors: 20, stacks: 20 }));
    let material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    add_planet(
        &mut commands,
        Vec3::ZERO,
        200.0,
        2.0,
        &mesh,
        &material,
    );

    commands
        .insert_resource(TargetedPlanet {
            pos: Vec3::ZERO,
            radius: 2.0,
        });

    add_ball(
        &mut commands,
        Vec3::new(4.0, 0.0, 0.0),
        3.0,
        0.1,
        &mesh,
        &material,
        Vec3::new(1.0, 1.0, 1.0) * 0.5,
    );

    commands
        .add_resource(CameraState::Follow);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(CameraTag {
            rig: CameraRig::builder()
                .with(Position::new(Vec3::new(4.0, 0.0, 0.0)))
                .with(Rotation::new(Quat::IDENTITY))
                .with(Smooth::new_position(1.25))
                .with(Arm::new(Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(
                    LookAt::new(Vec3::new(4.0, 0.0, 0.0))
                        .tracking_smoothness(1.25)
                        // .tracking_predictive(true)
                )
                .build()
        });

    commands.insert_resource(InputState {
        start: None,
        cursor_pos: Vec2::ZERO,
    });

    commands.insert_resource(ProjectedResources {
        mesh,
        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
    });

    commands.insert_resource(DeltaTime {
        time: 1.0 / 60.0,
    });

    let mut ortho_cam = OrthographicCameraBundle::new_2d();

    ortho_cam.orthographic_projection.top = 1.0;
    ortho_cam.orthographic_projection.bottom = -1.0;

    ortho_cam.orthographic_projection.right = 1.0 * RESOLUTION;
    ortho_cam.orthographic_projection.left = -1.0 * RESOLUTION;

    ortho_cam.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(ortho_cam).insert(UIMainCamera);
}

fn update_timestep(
    mut delta_time: ResMut<DeltaTime>,
    time: Res<Time>,
) {
    delta_time.time *= 0.8;
    delta_time.time += 0.2 * time.delta_seconds();
}

fn update_acc(
    query_pos: Query<(Entity, &GlobalTransform, &Planet)>,
    mut query_acc: Query<(Entity, &GlobalTransform, &GravityAffected, &mut Dynamics)>,
) {
    for (outer, outer_pos, outer_gravity, mut acc) in query_acc.iter_mut() {
        acc.acc = Vec3::ZERO;
        for (inner, inner_pos, inner_gravity) in query_pos.iter() {

            if outer == inner {
                continue;
            }

            let f = acc_of(inner_gravity.mass, outer_pos.translation, inner_pos.translation);

            // acc.acc += delta.normalize() * f / outer_gravity.mass;
            acc.acc += f;
        }
    }
}

fn update_pos_vel( // in the name
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Dynamics)>,
) {
    let delta = time.delta().as_secs_f32();
    for (mut pos, mut acc) in query.iter_mut() {
        let delta_vel = acc.acc * delta;
        acc.vel += delta_vel;
        pos.translation += acc.vel * delta;
    }
}

fn collide(
    mut query: Query<(
        Entity,
        &GravityAffected,
        &mut Dynamics,
        &GlobalTransform,
        &mut Transform
    )>,
    statics: Query<(
        &Planet,
        &GlobalTransform,
    )>,
    mut commands: Commands,
) { //checking for collisions between ball and planet
    for (ball_entity, ball, mut ball_dyn, ball_pos, mut ball_transform) in query.iter_mut() {
        for (planet, planet_transform) in statics.iter() {
            let delta = ball_pos.translation - planet_transform.translation;

            let needed_dist = ball.radius + planet.radius;

            if delta.length() - needed_dist < 0.0 {
                let displacement = planet_transform.translation - ball_transform.translation;
                let displacement = displacement.normalize() * needed_dist;

                ball_transform.translation = planet_transform.translation - displacement;

                let incident = ball_dyn.vel.normalize();
                let normal = delta.normalize();

                let reflected = incident - 2.0 * normal.dot(incident) * normal;

                ball_dyn.vel = reflected * ball_dyn.vel.length() * 0.75;

                if ball_dyn.vel.length() < 0.05 {
                    commands.entity(ball_entity)
                        .remove::<Dynamics>();
                }
            }
        }
    }
}

fn camera_movement(
    clicked: Res<Input<MouseButton>>,
    mut position: EventReader<MouseMotion>,
    target: Res<TargetedPlanet>,
    mut query: Query<(&mut Transform, &mut CameraTag)>,
    ball: Query<&GlobalTransform, With<MainBall>>,
    time: Res<Time>,
    state: Res<CameraState>,
) {
    let (mut transform, mut camera) = query.iter_mut().next().unwrap();

    if clicked.pressed(MouseButton::Left) {
        for event in position.iter() {
            let delta = event.delta / (60.0 * (target.pos - transform.translation).length());


            camera.rig.driver_mut::<Rotation>().rotation *= Quat::from_rotation_x(-delta.y) * Quat::from_rotation_y(-delta.x);
        }
    }

    let ball = ball.iter().next().unwrap();

    match *state {
        CameraState::Follow => {
            camera.rig.driver_mut::<Position>().position = ball.translation;
            camera.rig.driver_mut::<LookAt>().target = ball.translation;
        },
        CameraState::Around { pos, id: _id } => {
            camera.rig.driver_mut::<Position>().position = pos;
            camera.rig.driver_mut::<LookAt>().target = pos;
        }
    }

    let new_transform = camera.rig.update(time.delta_seconds());
    transform.translation = new_transform.position;
    transform.rotation = new_transform.rotation;
}

fn aim(
    clicked: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut input: ResMut<InputState>,
    mut commands: Commands,
    spheres: Query<Entity, With<ProjectedSphere>>,
    projected_resources: Res<ProjectedResources>,
    camera: Query<&Transform, With<CameraTag>>,
    gravity_sources: Query<(&Planet, &Transform)>,
    mut ball: Query<(Entity, &Transform), (With<MainBall>, Without<Dynamics>)>,
    time: Res<DeltaTime>,
) {
    let (ball_entity, ball_pos) = if let Some(x) = ball.iter_mut().next() {
        x
    } else {
        return;
    };

    cursor_moved_events
        .iter()
        .last()
        .map(|x| input.cursor_pos = x.position);

    if clicked.just_pressed(MouseButton::Right) {
        let pos = input.cursor_pos;
        input.start = Some(pos);
    }

    let camera = camera.iter().next().unwrap();

    spheres
        .iter()
        .for_each(|x| commands.entity(x).despawn());

    if clicked.just_released(MouseButton::Right) {
        if let Some(start) = input.start {
            let vel = vel_from_delta(input.cursor_pos - start, camera);
            println!("{:?}", vel);
            commands.entity(ball_entity)
                .insert(Dynamics {
                    vel,
                    acc: Vec3::ZERO,
                });

            input.start = None;
        }
    }

    if let Some(pos) = input.start {
        let delta = input.cursor_pos - pos;

        let vel = vel_from_delta(delta, camera);

        let positions = simulate_ball(
            gravity_sources,
            ball_pos.translation,
            vel,
            16,
            20,
            time.time
        );

        for (i, pos) in positions.iter().copied().enumerate() {
            commands.spawn_bundle(PbrBundle {
                mesh: projected_resources.mesh.clone(),
                material: projected_resources.material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                    .with_scale(0.05 * Vec3::splat(1.0 - i as f32 / positions.len() as f32)),
                ..default()
            })
                .insert(ProjectedSphere);
        }
    }
}

fn vel_from_delta(
    delta: Vec2,
    transform: &Transform,
) -> Vec3 {
    let dir_x = transform.right();
    let dir_y = transform.up();
    let dir_z = -transform.forward();

    let mut direction = delta.x * dir_x + delta.y * dir_y;

    direction /= 50.0;

    direction += direction.length() * dir_z;

    direction
}

fn simulate_ball(
    gravity: Query<(&Planet, &Transform)>,
    start: Vec3,
    vel: Vec3,
    num: usize,
    steps_between: usize,
    timestep: f32,
) -> Vec<Vec3> {
    let mut pos = start;
    let mut vel = vel;
    let mut result = Vec::new();

    let planets = gravity
        .iter()
        .map(|(planet, transform)| (planet.mass, planet.radius, transform.translation))
        .collect::<Vec<_>>();

    for _ in 0..num {
        for _ in 0..steps_between {
            let mut acc_sum = Vec3::ZERO;

            let mut inside = false;

            planets
                .iter()
                .copied()
                .for_each(|(p_mass, p_rad, p_pos)| {
                    acc_sum += acc_of(p_mass, pos, p_pos);
                    inside |= (pos - p_pos).length() < p_rad;
                });

            if inside {
                return result;
            }

            vel += acc_sum * timestep;

            pos += vel * timestep;
        }

        result.push(pos);
    }

    result
}

fn acc_of(m2: f32, p1: Vec3, p2: Vec3) -> Vec3 {
    (p2 - p1).normalize() * GRAVITY * m2 / (p1 - p2).length_squared()
}

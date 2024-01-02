use std::f32::{consts::PI, EPSILON};

use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{Quat, Vec2, Vec3},
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{
        camera::Camera,
        color::Color,
        mesh::{shape, Mesh},
    },
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{actions::Actions, GameState};

pub struct SimpleScenePlugin;

impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraAngle(Vec2::new(0., PI / 8.)));
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, move_camera.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
pub struct CameraAngle(Vec2);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_angle: Res<CameraAngle>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: camera_transform(&camera_angle),
        ..default()
    });
}

fn move_camera(
    time: Res<Time>,
    actions: Res<Actions>,
    mut camera_angle: ResMut<CameraAngle>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if actions.camera_orbit.is_none() {
        return;
    }
    let speed = 1.;
    camera_angle.0.x += actions.camera_orbit.unwrap().x * speed * time.delta_seconds();
    camera_angle.0.x = camera_angle.0.x % (2. * PI);
    camera_angle.0.y += actions.camera_orbit.unwrap().y * speed * time.delta_seconds();
    camera_angle.0.y = camera_angle.0.y.clamp(0. + EPSILON, PI / 2. - EPSILON);
    for mut camera_xform in &mut camera_query {
        *camera_xform = camera_transform(&camera_angle);
    }
}

fn camera_transform(camera_angle: &CameraAngle) -> Transform {
    let radius = 10.;
    let y = radius * camera_angle.0.y.sin();
    let z = radius * camera_angle.0.x.sin() * camera_angle.0.y.cos();
    let x = radius * camera_angle.0.x.cos() * camera_angle.0.y.cos();
    Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y)
}

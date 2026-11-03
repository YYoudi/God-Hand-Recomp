//! Synthetic primitives only. No game art, executable, animation or level is loaded.
use bevy::prelude::*;
use godhand_engine::{Position, TICK_HZ, step};

#[derive(Component)]
struct Actor;

#[derive(Resource, Default)]
struct Intent([f32; 2]);

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.05)))
        .insert_resource(Time::<Fixed>::from_hz(TICK_HZ as f64))
        .init_resource::<Intent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "God Hand independent engine lab | synthetic scene | WASD / arrows".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, sample_input)
        .add_systems(FixedUpdate, move_actor)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(24.0, 0.2, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.14, 0.18))),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));
    commands.spawn((
        Actor,
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.95, 0.42, 0.17))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 7.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 9.0, 13.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn sample_input(keys: Res<ButtonInput<KeyCode>>, mut intent: ResMut<Intent>) {
    let held = |a, b| u8::from(keys.pressed(a) || keys.pressed(b)) as f32;
    intent.0 = [
        held(KeyCode::KeyD, KeyCode::ArrowRight) - held(KeyCode::KeyA, KeyCode::ArrowLeft),
        held(KeyCode::KeyS, KeyCode::ArrowDown) - held(KeyCode::KeyW, KeyCode::ArrowUp),
    ];
}

fn move_actor(
    time: Res<Time<Fixed>>,
    intent: Res<Intent>,
    mut actors: Query<&mut Transform, With<Actor>>,
) {
    for mut transform in &mut actors {
        let mut p = Position {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
        };
        step(&mut p, intent.0, 4.0, time.delta_secs());
        transform.translation = Vec3::new(p.x.clamp(-11.0, 11.0), p.y, p.z.clamp(-11.0, 11.0));
    }
}

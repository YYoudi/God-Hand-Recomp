//! Synthetic primitives only. No game art, executable, animation or level is loaded.
use bevy::app::AppExit;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use godhand_engine::{Position, TICK_HZ, step};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

#[derive(Component)]
struct Actor;

#[derive(Resource, Default)]
struct Intent([f32; 2]);

#[derive(Resource)]
struct SmokeLimit {
    duration: Option<Duration>,
    first_update: Option<Instant>,
    completed: Arc<AtomicBool>,
}

pub fn run(smoke_seconds: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    let completed = Arc::new(AtomicBool::new(false));
    let result = App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.05)))
        .insert_resource(Time::<Fixed>::from_hz(TICK_HZ as f64))
        .init_resource::<Intent>()
        .insert_resource(SmokeLimit {
            duration: smoke_seconds.map(Duration::from_secs),
            first_update: None,
            completed: Arc::clone(&completed),
        })
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
        .add_systems(Update, finish_smoke)
        .add_systems(FixedUpdate, move_actor)
        .run();
    if smoke_seconds.is_some() && !completed.load(Ordering::SeqCst) {
        return Err("smoke window closed before the configured deadline".into());
    }
    match result {
        AppExit::Success => Ok(()),
        AppExit::Error(code) => Err(format!("viewer exited with code {code}").into()),
    }
}

fn finish_smoke(mut limit: ResMut<SmokeLimit>, mut exit: MessageWriter<AppExit>) {
    let Some(duration) = limit.duration else {
        return;
    };
    let started = limit.first_update.get_or_insert_with(Instant::now);
    if started.elapsed() >= duration {
        completed_notice(&limit.completed);
        exit.write(AppExit::Success);
    }
}

fn completed_notice(completed: &AtomicBool) {
    if !completed.swap(true, Ordering::SeqCst) {
        println!("Synthetic sandbox smoke completed; deadline reached, exiting normally.");
    }
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
        Tonemapping::None,
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

//! Separate synthetic build check and ISO-backed texture inspection modes.
use crate::color_material::{SourceColorMaterial, SourceColorPlugin};
use bevy::app::AppExit;
use bevy::asset::RenderAssetUsages;
use bevy::camera::{CameraProjection, SubCameraView};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Vec3A;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use godhand_disc::tim2::RgbaImage;
use godhand_engine::inspection_camera::{FAR, InspectionCamera, NEAR};
use godhand_engine::{Position, TICK_HZ, step};
use std::path::PathBuf;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

type CaptureResult = Arc<Mutex<Option<Result<(), String>>>>;

/// Preserve an explicit inspection aspect through Bevy's resize updates.
/// Matrix/subview/frustum semantics remain the pinned Bevy perspective API.
#[derive(Debug, Clone)]
struct FixedAspectProjection(PerspectiveProjection);
impl CameraProjection for FixedAspectProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        self.0.get_clip_from_view()
    }
    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4 {
        self.0.get_clip_from_view_for_sub(sub_view)
    }
    fn update(&mut self, _width: f32, _height: f32) {}
    fn far(&self) -> f32 {
        self.0.far
    }
    fn get_frustum_corners(&self, near: f32, far: f32) -> [Vec3A; 8] {
        self.0.get_frustum_corners(near, far)
    }
}

fn inspection_projection(fov: f32, near: f32, far: f32, aspect: Option<f32>) -> Projection {
    let perspective = PerspectiveProjection {
        fov,
        near,
        far,
        ..default()
    };
    if let Some(aspect_ratio) = aspect {
        Projection::custom(FixedAspectProjection(PerspectiveProjection {
            aspect_ratio,
            ..perspective
        }))
    } else {
        Projection::Perspective(perspective)
    }
}

#[derive(Resource)]
struct CaptureRequest {
    path: Option<PathBuf>,
    delay: Duration,
    result: CaptureResult,
}

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

#[derive(Resource)]
struct TextureInput(Option<RgbaImage>);
#[derive(Resource)]
struct MeshInput(Option<crate::room::MeshView>);
#[derive(Resource)]
struct CameraInput(Option<InspectionCamera>);
#[derive(Resource)]
struct SourceColorInput(bool);
#[derive(Resource)]
struct BackfaceCulling(bool);

pub fn run(
    smoke_seconds: Option<u64>,
    texture: Option<RgbaImage>,
    mesh: Option<crate::room::MeshView>,
    capture_png: Option<PathBuf>,
    camera: Option<InspectionCamera>,
    view_size: [u32; 2],
    static_colors: bool,
    backface_culling: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = &capture_png {
        if path.extension().is_none_or(|v| v != "png") || path.exists() {
            return Err("capture destination must be a new .png file".into());
        }
        if smoke_seconds.is_none_or(|v| v < 10) {
            return Err("capture requires a bounded smoke of at least 10 seconds".into());
        }
    }
    if mesh
        .as_ref()
        .is_some_and(|m| m.mesh.primitives.iter().all(|p| p.indices.is_empty()))
    {
        return Err("selected mesh has no emitted faces".into());
    }
    println!(
        "Inspection camera: {camera:?}; requested physical output: {view_size:?}. Explicit pose is diagnostic, not recovered game camera behavior."
    );
    if static_colors {
        let missing = mesh.as_ref().map_or(0, |m| {
            m.mesh
                .primitives
                .iter()
                .filter(|p| p.colors.is_none())
                .count()
        });
        println!(
            "Static-color experiment enabled: raw RGB modulate /128, diagnostic host alpha; {missing} buffers use neutral fallback. No full runtime lighting/fog/blending parity."
        );
    }
    let completed = Arc::new(AtomicBool::new(false));
    let capture_expected = capture_png.is_some();
    let capture_result: CaptureResult = Arc::new(Mutex::new(None));
    let title = if mesh.as_ref().is_some_and(|m| m.layout) {
        "God Hand | candidate layout | S-R-T hypothesis, unlit; not runtime parity"
    } else if mesh.is_some() {
        "God Hand | ISO static mesh inspection | unlit, local coordinates"
    } else if texture.is_some() {
        "God Hand | ISO texture inspection | not a game scene"
    } else {
        "God Hand independent engine lab | synthetic scene | WASD / arrows"
    };
    let result = App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.035, 0.05)))
        .insert_resource(Time::<Fixed>::from_hz(TICK_HZ as f64))
        .init_resource::<Intent>()
        .insert_resource(TextureInput(texture))
        .insert_resource(MeshInput(mesh))
        .insert_resource(CameraInput(camera))
        .insert_resource(SourceColorInput(static_colors))
        .insert_resource(BackfaceCulling(backface_culling))
        .insert_resource(CaptureRequest {
            path: capture_png,
            delay: Duration::from_secs(smoke_seconds.unwrap_or(20) / 2),
            result: Arc::clone(&capture_result),
        })
        .insert_resource(SmokeLimit {
            duration: smoke_seconds.map(Duration::from_secs),
            first_update: None,
            completed: Arc::clone(&completed),
        })
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: title.into(),
                    resolution: bevy::window::WindowResolution::new(view_size[0], view_size[1])
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(SourceColorPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, sample_input)
        .add_systems(Update, (request_capture, finish_smoke).chain())
        .add_systems(FixedUpdate, move_actor)
        .run();
    if smoke_seconds.is_some() && !completed.load(Ordering::SeqCst) {
        return Err("smoke window closed before the configured deadline".into());
    }
    if capture_expected {
        match capture_result.lock().unwrap().as_ref() {
            Some(Ok(())) => (),
            Some(Err(message)) => return Err(format!("capture failed: {message}").into()),
            None => return Err("capture did not finish before the smoke deadline".into()),
        }
    }
    match result {
        AppExit::Success => Ok(()),
        AppExit::Error(code) => Err(format!("viewer exited with code {code}").into()),
    }
}

fn request_capture(
    mut commands: Commands,
    limit: Res<SmokeLimit>,
    mut request: ResMut<CaptureRequest>,
) {
    if !limit
        .first_update
        .is_some_and(|t| t.elapsed() >= request.delay)
    {
        return;
    }
    let Some(path) = request.path.take() else {
        return;
    };
    let result = Arc::clone(&request.result);
    commands
        .spawn(Screenshot::primary_window())
        .observe(move |capture: On<ScreenshotCaptured>| {
            let saved = capture
                .image
                .clone()
                .try_into_dynamic()
                .map_err(|e| e.to_string())
                .and_then(|image| {
                    // create_new protects originals even if a file appeared after
                    // the early path check while asynchronous capture was pending.
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&path)
                        .map_err(|e| e.to_string())?;
                    let format = bevy::image::ImageFormat::Png
                        .as_image_crate_format()
                        .ok_or_else(|| "PNG codec unavailable".to_owned())?;
                    image.write_to(&mut file, format).map_err(|e| e.to_string())
                });
            match &saved {
                Ok(()) => println!("Original renderer PNG captured: {}", path.display()),
                Err(e) => eprintln!("Renderer PNG capture failed: {e}"),
            }
            *result.lock().unwrap() = Some(saved);
        });
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
        println!("Viewer smoke completed; deadline reached, exiting normally.");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut input: ResMut<TextureInput>,
    mut mesh_input: ResMut<MeshInput>,
    camera_input: Res<CameraInput>,
    source_colors: Res<SourceColorInput>,
    backface_culling: Res<BackfaceCulling>,
    mut color_materials: ResMut<Assets<SourceColorMaterial>>,
) {
    if let Some(view) = mesh_input.0.take() {
        let mut handles = std::collections::BTreeMap::new();
        for (ordinal, texture) in view.textures {
            let mut image = Image::new(
                Extent3d {
                    width: u32::from(texture.width),
                    height: u32::from(texture.height),
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                texture.pixels,
                if source_colors.0 {
                    TextureFormat::Rgba8Unorm
                } else {
                    TextureFormat::Rgba8UnormSrgb
                },
                RenderAssetUsages::default(),
            );
            let mut sampler = ImageSamplerDescriptor::nearest();
            sampler.address_mode_u = ImageAddressMode::Repeat;
            sampler.address_mode_v = ImageAddressMode::Repeat;
            image.sampler = ImageSampler::Descriptor(sampler);
            handles.insert(ordinal, images.add(image));
        }
        let mut minimum = Vec3::splat(f32::INFINITY);
        let mut maximum = Vec3::splat(f32::NEG_INFINITY);
        for primitive in view.mesh.primitives {
            for &position in &primitive.positions {
                minimum = minimum.min(Vec3::from_array(position));
                maximum = maximum.max(Vec3::from_array(position));
            }
            if primitive.indices.is_empty() {
                continue;
            }
            let colors = if source_colors.0 {
                Some(
                    primitive
                        .colors
                        .unwrap_or_else(|| vec![[128; 4]; primitive.positions.len()])
                        .into_iter()
                        .map(|c| c.map(|v| f32::from(v) / 128.))
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, primitive.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, primitive.uvs)
            .with_inserted_indices(Indices::U32(primitive.indices))
            .with_computed_normals();
            if let Some(colors) = colors {
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(color_materials.add(SourceColorMaterial {
                        texture: handles[&primitive.material].clone(),
                        backface_culling: backface_culling.0,
                    })),
                    Transform::default(),
                ));
                continue;
            }
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        base_color_texture: Some(handles[&primitive.material].clone()),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: backface_culling
                            .0
                            .then_some(bevy::render::render_resource::Face::Back),
                        double_sided: true,
                        ..default()
                    }),
                ),
                Transform::default(),
            ));
        }
        let center = (minimum + maximum) * 0.5;
        let radius = ((maximum - minimum).length() * 0.5).max(0.1);
        let (pose, fov, near, far) = if let Some(camera) = camera_input.0 {
            (
                Transform::from_translation(Vec3::from_array(camera.eye()))
                    .looking_to(Vec3::from_array(camera.forward()), Vec3::Y),
                camera.fov_radians(),
                NEAR,
                FAR,
            )
        } else {
            (
                Transform::from_translation(
                    center + Vec3::new(2.0, 1.2, 2.0).normalize() * radius * 3.3,
                )
                .looking_at(center, Vec3::Y),
                std::f32::consts::FRAC_PI_4,
                radius * 0.01,
                radius * 20.0,
            )
        };
        commands.spawn((
            Camera3d::default(),
            Tonemapping::None,
            DebandDither::Disabled,
            inspection_projection(
                fov,
                near,
                far,
                camera_input.0.and_then(|c| c.projection_aspect()),
            ),
            pose,
        ));
        return;
    }
    if let Some(texture) = input.0.take() {
        let width = f32::from(texture.width);
        let height = f32::from(texture.height);
        let mut image = Image::new(
            Extent3d {
                width: u32::from(texture.width),
                height: u32::from(texture.height),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            texture.pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = ImageSampler::nearest();
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(images.add(image)),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::default(),
        ));
        commands.spawn((
            Camera3d::default(),
            Tonemapping::None,
            DebandDither::Disabled,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                    viewport_height: 720.0,
                },
                ..OrthographicProjection::default_3d()
            }),
            Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
        return;
    }
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
        DebandDither::Disabled,
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

#[cfg(test)]
mod projection_tests {
    use super::*;

    #[test]
    fn default_stays_automatic_and_override_survives_resize() {
        let mut automatic = inspection_projection(1., NEAR, FAR, None);
        assert!(matches!(automatic, Projection::Perspective(_)));
        automatic.update(512., 448.);
        let expected = PerspectiveProjection {
            fov: 1.,
            near: NEAR,
            far: FAR,
            aspect_ratio: 512. / 448.,
            ..default()
        };
        assert_eq!(
            automatic.get_clip_from_view(),
            expected.get_clip_from_view()
        );
        let mut fixed = inspection_projection(1., NEAR, FAR, Some(4. / 3.));
        let initial = fixed.get_clip_from_view();
        let corners = fixed.get_frustum_corners(-NEAR, -FAR);
        for (w, h) in [(512., 448.), (1920., 1080.), (448., 512.)] {
            fixed.update(w, h);
            assert_eq!(fixed.get_clip_from_view(), initial);
            assert_eq!(fixed.get_frustum_corners(-NEAR, -FAR), corners);
            assert_eq!(fixed.far(), FAR);
        }
    }

    #[test]
    fn bevy_matrix_matches_cpu_oracle_with_non_square_projection() {
        let eye = [2., 3., 9.];
        let camera = InspectionCamera::new(eye, [-1., 1., -6.], 60.)
            .unwrap()
            .with_projection_aspect(4. / 3.)
            .unwrap();
        let world_from_view = Transform::from_translation(Vec3::from_array(eye))
            .looking_to(Vec3::from_array(camera.forward()), Vec3::Y)
            .to_matrix();
        let mut projection =
            inspection_projection(camera.fov_radians(), NEAR, FAR, camera.projection_aspect());
        for size in [[512, 448], [1024, 448], [448, 512]] {
            projection.update(size[0] as f32, size[1] as f32);
            let clip_from_world = projection.get_clip_from_view() * world_from_view.inverse();
            for point in [[0., 0., 0.], [-4., 2., -20.], [8., -1., -5.]] {
                let clip = clip_from_world * Vec3::from_array(point).extend(1.);
                let ndc = clip.truncate() / clip.w;
                let pixel = [
                    (ndc.x as f64 + 1.) * size[0] as f64 / 2.,
                    (1. - ndc.y as f64) * size[1] as f64 / 2.,
                ];
                let expected = camera.project(point, size).unwrap();
                assert!((pixel[0] - expected[0]).abs() < 0.001);
                assert!((pixel[1] - expected[1]).abs() < 0.001);
            }
        }
    }
}

//! Opt-in, no-disc GPU conformance. CI compiles/tests helpers, never runs this main.
use bevy::{
    app::PluginsState,
    prelude::*,
    render::{
        RenderApp,
        pipelined_rendering::PipelinedRenderingPlugin,
        renderer::{RenderAdapterInfo, RenderDevice, RenderQueue},
    },
    window::{ExitCondition, WindowPlugin},
    winit::WinitPlugin,
};
use godhand_engine::fragment_alpha::{AlphaComparison::*, AlphaFailure::*, AlphaStage};
use godhand_lab::alpha_gpu::*;
use std::time::{Duration, Instant};

fn quad(
    rect: [f32; 4],
    depth: f32,
    texture: usize,
    stage: AlphaStage,
    color: [f32; 4],
) -> AlphaDraw {
    let [x0, y0, x1, y1] = rect;
    let points = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]];
    let uv = [[0., 0.5], [1., 0.5], [1., 0.5], [0., 0.5]];
    AlphaDraw {
        texture,
        stage,
        vertices: [0, 1, 2, 0, 2, 3]
            .into_iter()
            .map(|i| AlphaVertex {
                clip: [points[i][0], points[i][1], depth, 1.],
                uv: uv[i],
                color,
            })
            .collect(),
    }
}

fn setup() -> GpuResult<(App, RenderDevice, RenderQueue)> {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..Default::default()
            })
            .disable::<WinitPlugin>()
            .disable::<PipelinedRenderingPlugin>(),
    );
    let start = Instant::now();
    while app.plugins_state() != PluginsState::Ready {
        if start.elapsed() > Duration::from_secs(30) {
            return Err("GPU plugin initialization timeout".into());
        }
        bevy::tasks::tick_global_task_pools_on_main_thread();
        std::thread::sleep(Duration::from_millis(1));
    }
    app.finish();
    app.cleanup();
    let render = app.sub_app(RenderApp).world();
    let info = render.resource::<RenderAdapterInfo>();
    println!("GPU adapter: {} / {:?}", info.name, info.backend);
    let device = render.resource::<RenderDevice>().clone();
    let queue = render.resource::<RenderQueue>().clone();
    Ok((app, device, queue))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("alpha conformance FAILED: {error}");
        std::process::exit(1);
    }
}

fn run() -> GpuResult<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        println!(
            "godhand-alpha-check [--quick]\nNo ISO, window or game data. Tests the ordered GPU alpha adapter against the CPU oracle. --quick uses six reference boundaries; default uses all256.\nRGB overwrite and write permissions only, NOT GS blending or retail rendering."
        );
        return Ok(());
    }
    let quick = args == ["--quick"];
    if !args.is_empty() && !quick {
        return Err("unknown conformance argument".into());
    }
    let references: Vec<u8> = if quick {
        vec![0, 1, 127, 128, 254, 255]
    } else {
        (0..=255).collect()
    };
    let height = if quick { 64 } else { 256 };
    let start = Instant::now();
    let (_app, device, queue) = setup()?;
    let mut gpu = OrderedAlphaGpu::new(&device);
    // Original indexed TIM2 fixture, not game pixels: exercise decoder -> typed GPU input.
    let mut encoded = vec![0u8; 16 + 48 + 256 + 1024];
    encoded[..4].copy_from_slice(b"TIM2");
    encoded[4] = 4;
    encoded[6] = 1;
    encoded[16..20].copy_from_slice(&1328u32.to_le_bytes());
    encoded[20..24].copy_from_slice(&1024u32.to_le_bytes());
    encoded[24..28].copy_from_slice(&256u32.to_le_bytes());
    encoded[28] = 48;
    encoded[30..32].copy_from_slice(&256u16.to_le_bytes());
    encoded[33] = 1;
    encoded[34] = 3;
    encoded[35] = 5;
    encoded[36..38].copy_from_slice(&256u16.to_le_bytes());
    encoded[38] = 1;
    for (i, byte) in encoded[64..320].iter_mut().enumerate() {
        *byte = i as u8;
    }
    // Explicit CSM1 block ordering, independent of the decoder bit expression.
    let palette_order = (0..8).flat_map(|block| {
        [0, 16, 8, 24]
            .into_iter()
            .flat_map(move |group| (0..8).map(move |lane| block * 32 + group + lane))
    });
    for (alpha, palette_index) in palette_order.enumerate() {
        encoded[320 + palette_index * 4..324 + palette_index * 4].copy_from_slice(&[
            203,
            101,
            47,
            alpha as u8,
        ]);
    }
    let ramp = RawAlphaTexture::from(godhand_disc::tim2::decode_raw(encoded.as_slice())?);
    let mut checked = 0u64;
    for enabled in [true, false] {
        let comparisons = if enabled {
            vec![
                Never,
                Always,
                Less,
                LessEqual,
                Equal,
                GreaterEqual,
                Greater,
                NotEqual,
            ]
        } else {
            vec![Never]
        };
        for comparison in comparisons {
            for failure in [Keep, FramebufferOnly, DepthOnly, RgbOnly] {
                for depth_write_masked in [false, true] {
                    if start.elapsed() > Duration::from_secs(120) {
                        return Err("GPU conformance deadline exceeded".into());
                    }
                    let draws: Vec<_> = references
                        .iter()
                        .enumerate()
                        .map(|(row, &reference)| {
                            let y1 = 1. - row as f32 * 2. / height as f32;
                            let y0 = 1. - (row + 1) as f32 * 2. / height as f32;
                            quad(
                                [-1., y0, 1., y1],
                                0.5,
                                0,
                                AlphaStage {
                                    enabled,
                                    comparison,
                                    reference,
                                    failure,
                                    depth_write_masked,
                                },
                                [128.; 4],
                            )
                        })
                        .collect();
                    let frame = gpu.render(
                        &device,
                        &queue,
                        [256, height],
                        [17, 39, 73, 91],
                        std::slice::from_ref(&ramp),
                        &draws,
                    )?;
                    let data = frame.readback(&device, &queue)?;
                    for row in 0..height as usize {
                        for a in 0..=255u8 {
                            let i = row * 256 + a as usize;
                            let writes = references.get(row).map(|&reference| {
                                AlphaStage {
                                    enabled,
                                    comparison,
                                    reference,
                                    failure,
                                    depth_write_masked,
                                }
                                .permissions(a)
                            });
                            let rgb = if writes.is_some_and(|w| w.rgb) {
                                [203, 101, 47]
                            } else {
                                [17, 39, 73]
                            };
                            let alpha = if writes.is_some_and(|w| w.framebuffer_alpha) {
                                a
                            } else {
                                91
                            };
                            let depth = if writes.is_some_and(|w| w.depth) {
                                0.5
                            } else {
                                0.
                            };
                            if data.color[i] != [rgb[0], rgb[1], rgb[2], 255]
                                || data.framebuffer_alpha[i] != alpha
                                || data.depth[i] != depth
                            {
                                return Err(format!("{enabled}/{comparison:?}/{failure:?}/masked={depth_write_masked}, row{row}, alpha{a}: color{:?}, stored alpha{}, depth{}; expected{rgb:?}/{alpha}/{depth}",data.color[i],data.framebuffer_alpha[i],data.depth[i]).into());
                            }
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    let spatial = spatial_checks(&mut gpu, &device, &queue)?;
    println!(
        "PASS: {spatial} additional raw RGBA/depth spatial samples (ordering/equality/occlusion/modulation/interpolation)."
    );
    println!(
        "PASS: {checked} GPU alpha-stage pixels; every RGB/stored-alpha/depth value matches CPU. Mode:{} elapsed:{:.3}s. No blending or retail fidelity implied.",
        if quick { "quick" } else { "full" },
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

fn spatial_checks(
    gpu: &mut OrderedAlphaGpu,
    device: &RenderDevice,
    queue: &RenderQueue,
) -> GpuResult<u64> {
    let enabled_pass = AlphaStage {
        enabled: false,
        comparison: Never,
        reference: 0,
        failure: Keep,
        depth_write_masked: false,
    };
    let full = [-1., -1., 1., 1.];
    let mut checks = 0;
    for enabled in [true, false] {
        for alpha in [0, 127, 128, 255] {
            for failure in [Keep, FramebufferOnly, DepthOnly, RgbOnly] {
                for masked in [false, true] {
                    for middle in [false, true] {
                        let textures = [
                            RawAlphaTexture {
                                width: 1,
                                height: 1,
                                rgba: vec![0, 0, 200, 19],
                            },
                            RawAlphaTexture {
                                width: 1,
                                height: 1,
                                rgba: vec![220, 20, 35, alpha],
                            },
                            RawAlphaTexture {
                                width: 1,
                                height: 1,
                                rgba: vec![0, 211, 0, 211],
                            },
                        ];
                        let stage = AlphaStage {
                            enabled,
                            comparison: Greater,
                            reference: 127,
                            failure,
                            depth_write_masked: masked,
                        };
                        let mut draws = vec![
                            quad(full, 0.25, 0, enabled_pass, [128.; 4]),
                            quad(full, 0.75, 1, stage, [128.; 4]),
                        ];
                        if middle {
                            draws.push(quad(full, 0.5, 2, enabled_pass, [128.; 4]));
                        }
                        let data = gpu
                            .render(device, queue, [64, 64], [17, 39, 73, 91], &textures, &draws)?
                            .readback(device, queue)?;
                        let writes = stage.permissions(alpha);
                        let mut rgb = if writes.rgb {
                            [220, 20, 35]
                        } else {
                            [0, 0, 200]
                        };
                        let mut a = if writes.framebuffer_alpha { alpha } else { 19 };
                        let mut z = if writes.depth { 0.75 } else { 0.25 };
                        if middle && z <= 0.5 {
                            rgb = [0, 211, 0];
                            a = 211;
                            z = 0.5;
                        }
                        for i in 0..4096 {
                            if data.color[i] != [rgb[0], rgb[1], rgb[2], 255]
                                || data.framebuffer_alpha[i] != a
                                || data.depth[i] != z
                            {
                                return Err(format!("spatial occlusion {enabled}/{alpha}/{failure:?}/{masked}/{middle} at{i}").into());
                            }
                            checks += 1;
                        }
                    }
                }
            }
        }
    }
    // Preserve caller submission order, including equal depth and no-depth-write draws.
    let colors = [
        RawAlphaTexture {
            width: 1,
            height: 1,
            rgba: vec![210, 0, 0, 0],
        },
        RawAlphaTexture {
            width: 1,
            height: 1,
            rgba: vec![0, 0, 190, 255],
        },
    ];
    for color_only in [false, true] {
        for reversed in [false, true] {
            let stage = if color_only {
                AlphaStage {
                    enabled: true,
                    comparison: Never,
                    failure: FramebufferOnly,
                    ..enabled_pass
                }
            } else {
                enabled_pass
            };
            let mut draws = vec![
                quad(full, 0.5, 0, stage, [128.; 4]),
                quad(full, 0.5, 1, stage, [128.; 4]),
            ];
            if reversed {
                draws.reverse();
            }
            let data = gpu
                .render(device, queue, [64, 64], [17, 39, 73, 91], &colors, &draws)?
                .readback(device, queue)?;
            let expected = if reversed {
                [210, 0, 0, 255]
            } else {
                [0, 0, 190, 255]
            };
            let a = if reversed { 0 } else { 255 };
            for i in 0..4096 {
                if data.color[i] != expected
                    || data.framebuffer_alpha[i] != a
                    || data.depth[i] != if color_only { 0. } else { 0.5 }
                {
                    return Err("submission order/equal depth mismatch".into());
                }
                checks += 1;
            }
        }
    }
    // A single draw may contain overlapping primitives with DIFFERENT alpha outcomes.
    // Whole-draw pass/fail partitioning would reorder them and corrupt RGB/depth.
    let mixed_texture = RawAlphaTexture {
        width: 2,
        height: 1,
        rgba: vec![210, 0, 0, 0, 0, 0, 190, 255],
    };
    let mixed_stage = AlphaStage {
        enabled: true,
        comparison: Greater,
        reference: 127,
        failure: FramebufferOnly,
        ..enabled_pass
    };
    for reversed in [false, true] {
        let mut red = quad(full, 0.5, 0, mixed_stage, [128.; 4]);
        let mut blue = red.clone();
        for v in &mut red.vertices {
            v.uv = [0.25, 0.5];
        }
        for v in &mut blue.vertices {
            v.uv = [0.75, 0.5];
        }
        let (mut first, last) = if reversed { (blue, red) } else { (red, blue) };
        first.vertices.extend(last.vertices);
        let data = gpu
            .render(
                device,
                queue,
                [64, 64],
                [17, 39, 73, 91],
                std::slice::from_ref(&mixed_texture),
                &[first],
            )?
            .readback(device, queue)?;
        let expected = if reversed {
            [210, 0, 0, 255]
        } else {
            [0, 0, 190, 255]
        };
        let a = if reversed { 0 } else { 255 };
        for i in 0..4096 {
            if data.color[i] != expected || data.framebuffer_alpha[i] != a || data.depth[i] != 0.5 {
                return Err("intra-draw primitive order changed by alpha branch partition".into());
            }
            checks += 1;
        }
    }
    // Exact byte multiplication, truncation/saturation, with the test AFTER modulation.
    let texture = RawAlphaTexture {
        width: 1,
        height: 1,
        rgba: vec![129, 200, 255, 129],
    };
    let stage = AlphaStage {
        enabled: true,
        comparison: Equal,
        reference: 64,
        ..enabled_pass
    };
    let draw = quad(full, 0.5, 0, stage, [64.9, 128.9, 255., 64.9]);
    let data = gpu
        .render(
            device,
            queue,
            [64, 64],
            [17, 39, 73, 91],
            &[texture],
            &[draw],
        )?
        .readback(device, queue)?;
    for i in 0..4096 {
        if data.color[i] != [64, 200, 255, 255]
            || data.framebuffer_alpha[i] != 64
            || data.depth[i] != 0.5
        {
            return Err("byte modulation before alpha test mismatch".into());
        }
        checks += 1;
    }
    // Different clipW at left/right: byte color must remain SCREEN linear, not perspective.
    let texture = RawAlphaTexture {
        width: 1,
        height: 1,
        rgba: vec![100, 20, 200, 128],
    };
    let stage = AlphaStage {
        enabled: true,
        comparison: Greater,
        reference: 63,
        failure: FramebufferOnly,
        ..enabled_pass
    };
    let mut draw = quad(full, 0.5, 0, stage, [128.; 4]);
    for v in &mut draw.vertices {
        let right = v.clip[0] > 0.;
        let w = if right { 3. } else { 1. };
        for c in &mut v.clip[..3] {
            *c *= w;
        }
        v.clip[3] = w;
        v.color[3] = if right { 127. } else { 0. };
    }
    let data = gpu
        .render(
            device,
            queue,
            [64, 64],
            [17, 39, 73, 91],
            &[texture],
            &[draw],
        )?
        .readback(device, queue)?;
    for y in 0..64 {
        for x in 0..64 {
            let i = y * 64 + x;
            let alpha = ((x as f32 + 0.5) * 127. / 64.).floor() as u8;
            let z = if alpha > 63 { 0.5 } else { 0. };
            if data.color[i] != [100, 20, 200, 255]
                || data.framebuffer_alpha[i] != alpha
                || (data.depth[i] - z).abs() > 1e-6
            {
                return Err(format!(
                    "screen-linear alpha mismatch at{x},{y}: stored{}, z{}",
                    data.framebuffer_alpha[i], data.depth[i]
                )
                .into());
            }
            checks += 1;
        }
    }
    // Do not use an epsilon that silently rounds a representable value across127/128.
    let texture = RawAlphaTexture {
        width: 1,
        height: 1,
        rgba: vec![100, 20, 200, 128],
    };
    for alpha in [f32::from_bits(128f32.to_bits() - 1), 128.] {
        let stage = AlphaStage {
            enabled: true,
            comparison: Greater,
            reference: 127,
            failure: Keep,
            ..enabled_pass
        };
        let draw = quad(full, 0.5, 0, stage, [128., 128., 128., alpha]);
        let data = gpu
            .render(
                device,
                queue,
                [64, 64],
                [17, 39, 73, 91],
                std::slice::from_ref(&texture),
                &[draw],
            )?
            .readback(device, queue)?;
        let passed = alpha >= 128.;
        let rgb = if passed {
            [100, 20, 200, 255]
        } else {
            [17, 39, 73, 255]
        };
        let a = if passed { 128 } else { 91 };
        let z = if passed { 0.5 } else { 0. };
        for i in 0..4096 {
            if data.color[i] != rgb || data.framebuffer_alpha[i] != a || data.depth[i] != z {
                return Err("float-to-byte threshold rounded across127/128".into());
            }
            checks += 1;
        }
    }
    // Negative/zero homogeneous W are valid clip inputs, not whole-draw errors.
    let texture = RawAlphaTexture {
        width: 1,
        height: 1,
        rgba: vec![100, 20, 200, 128],
    };
    for w in [-1., 0.] {
        let mut draw = quad(full, 0.25, 0, enabled_pass, [128.; 4]);
        for v in &mut draw.vertices {
            v.clip[3] = w;
        }
        let data = gpu
            .render(
                device,
                queue,
                [64, 64],
                [17, 39, 73, 91],
                std::slice::from_ref(&texture),
                &[draw],
            )?
            .readback(device, queue)?;
        for i in 0..4096 {
            if data.color[i] != [17, 39, 73, 255]
                || data.framebuffer_alpha[i] != 91
                || data.depth[i] != 0.
            {
                return Err("fully clipped geometry wrote a target".into());
            }
            checks += 1;
        }
    }
    let draw = AlphaDraw {
        texture: 0,
        stage: enabled_pass,
        vertices: [
            [-0.8, -0.8, 0.25, 2.],
            [0.8, -0.8, 0.25, 2.],
            [0., 1., 0.25, -1.],
        ]
        .into_iter()
        .map(|clip| AlphaVertex {
            clip,
            uv: [0.5, 0.5],
            color: [128.; 4],
        })
        .collect(),
    };
    let data = gpu
        .render(
            device,
            queue,
            [64, 64],
            [17, 39, 73, 91],
            &[texture],
            &[draw],
        )?
        .readback(device, queue)?;
    let inside = 32 * 64 + 32;
    let outside = 60 * 64 + 32;
    if data.color[inside] != [100, 20, 200, 255]
        || data.framebuffer_alpha[inside] != 128
        || data.depth[inside] <= 0.
    {
        return Err("straddling triangle rejected visible geometry".into());
    }
    if data.color[outside] != [17, 39, 73, 255]
        || data.framebuffer_alpha[outside] != 91
        || data.depth[outside] != 0.
    {
        return Err("straddling triangle escaped clipping".into());
    }
    checks += 2;
    Ok(checks)
}

//! Original synthetic feedback/blend checks. No game data or stored screenshots.
use super::*;

fn full_quad(depth: f32, texture: usize, stage: AlphaStage, blend: ByteBlend) -> AlphaDraw {
    let mut draw = quad([-1., -1., 1., 1.], depth, texture, stage, [128.; 4]);
    for v in &mut draw.vertices {
        v.uv = [(v.clip[0] + 1.) * 0.5, (1. - v.clip[1]) * 0.5];
    }
    draw.blend = blend;
    draw
}
fn pass_stage() -> AlphaStage {
    AlphaStage {
        enabled: false,
        comparison: Always,
        reference: 0,
        failure: Keep,
        depth_write_masked: false,
    }
}
fn pixel_oracle(
    mut color: [u8; 4],
    mut depth: f32,
    sequence: &[([u8; 4], f32, AlphaStage, ByteBlend)],
) -> ([u8; 4], f32) {
    for &(source, z, stage, blend) in sequence {
        if z < depth {
            continue;
        }
        let writes = stage.permissions(source[3]);
        if writes.rgb {
            let rgb = blend.rgb(source, color[..3].try_into().unwrap());
            color[..3].copy_from_slice(&rgb);
        }
        if writes.framebuffer_alpha {
            color[3] = source[3];
        }
        if writes.depth {
            depth = z;
        }
    }
    (color, depth)
}
fn check_constant(data: AlphaReadback, expected: ([u8; 4], f32)) -> GpuResult<u64> {
    for i in 0..4096 {
        if data.color[i] != [expected.0[0], expected.0[1], expected.0[2], 255]
            || data.framebuffer_alpha[i] != expected.0[3]
            || data.depth[i] != expected.1
        {
            return Err(format!(
                "blend feedback/write-mask mismatch at{i}: {:?}/{}/{} != {:?}/{}",
                data.color[i], data.framebuffer_alpha[i], data.depth[i], expected.0, expected.1
            )
            .into());
        }
    }
    Ok(4096)
}
pub fn run(
    gpu: &mut OrderedAlphaGpu,
    device: &RenderDevice,
    queue: &RenderQueue,
    quick: bool,
) -> GpuResult<(u64, u64)> {
    let start = Instant::now();
    let mut arithmetic = 0;
    let mut spatial = 0;
    let alphas: Vec<u8> = if quick {
        vec![0, 1, 63, 64, 127, 128, 129, 254, 255]
    } else {
        (0..=255).collect()
    };
    let background = RawAlphaTexture {
        width: 256,
        height: 256,
        rgba: (0..256)
            .flat_map(|y| {
                (0..256)
                    .flat_map(move |x| [y as u8, (255 - y) as u8, ((x + 3 * y) % 256) as u8, 91])
            })
            .collect(),
    };
    for alpha in alphas {
        if start.elapsed() > Duration::from_secs(120) {
            return Err("byte-blend conformance deadline exceeded".into());
        }
        let source = RawAlphaTexture {
            width: 256,
            height: 256,
            rgba: (0..256)
                .flat_map(|y| {
                    (0..256).flat_map(move |x| {
                        [x as u8, (255 - x) as u8, ((3 * x + y) % 256) as u8, alpha]
                    })
                })
                .collect(),
        };
        let draws = [
            full_quad(0.25, 0, pass_stage(), ByteBlend::Overwrite),
            full_quad(0.5, 1, pass_stage(), ByteBlend::SourceOver),
        ];
        let data = gpu
            .render(
                device,
                queue,
                [256, 256],
                [17, 39, 73, 91],
                &[background.clone(), source],
                &draws,
            )?
            .readback(device, queue)?;
        for y in 0..256 {
            for x in 0..256 {
                let s = [x as u8, (255 - x) as u8, ((3 * x + y) % 256) as u8, alpha];
                let d = [y as u8, (255 - y) as u8, ((x + 3 * y) % 256) as u8];
                let rgb = ByteBlend::SourceOver.rgb(s, d);
                let i = y * 256 + x;
                if data.color[i] != [rgb[0], rgb[1], rgb[2], 255]
                    || data.framebuffer_alpha[i] != alpha
                    || data.depth[i] != 0.5
                {
                    return Err(format!("byte blend differs at{x},{y},a{alpha}: {:?}/{}/{} expected{rgb:?}/{alpha}/0.5",data.color[i],data.framebuffer_alpha[i],data.depth[i]).into());
                }
                arithmetic += 1;
            }
        }
    }
    // Passing/failing blend branches, later occlusion and equality are independent.
    let base = [20, 40, 200, 211];
    let green = [10, 210, 30, 128];
    let equal = [180, 40, 10, 32];
    for failure in [Keep, FramebufferOnly, DepthOnly, RgbOnly] {
        for masked in [false, true] {
            for alpha in [0, 64, 128, 255] {
                let red = [200, 20, 40, alpha];
                let stage = AlphaStage {
                    enabled: true,
                    comparison: Greater,
                    reference: 127,
                    failure,
                    depth_write_masked: masked,
                };
                let sequence = [
                    (base, 0.2, pass_stage(), ByteBlend::Overwrite),
                    (red, 0.8, stage, ByteBlend::SourceOver),
                    (green, 0.5, pass_stage(), ByteBlend::SourceOver),
                    (equal, 0.5, pass_stage(), ByteBlend::SourceOver),
                ];
                let textures: Vec<_> = [base, red, green, equal]
                    .into_iter()
                    .map(|p| RawAlphaTexture {
                        width: 1,
                        height: 1,
                        rgba: p.to_vec(),
                    })
                    .collect();
                let draws: Vec<_> = sequence
                    .iter()
                    .enumerate()
                    .map(|(i, (_, z, s, b))| full_quad(*z, i, *s, *b))
                    .collect();
                let data = gpu
                    .render(device, queue, [64, 64], [17, 39, 73, 91], &textures, &draws)?
                    .readback(device, queue)?;
                spatial += check_constant(data, pixel_oracle([17, 39, 73, 91], 0., &sequence))?;
            }
        }
    }
    // Reversing translucent equal-depth layers must change the result predictably.
    for reversed in [false, true] {
        let mut sequence = vec![
            (base, 0.5, pass_stage(), ByteBlend::SourceOver),
            ([240, 20, 10, 96], 0.5, pass_stage(), ByteBlend::SourceOver),
        ];
        if reversed {
            sequence.reverse();
        }
        let textures: Vec<_> = sequence
            .iter()
            .map(|(p, _, _, _)| RawAlphaTexture {
                width: 1,
                height: 1,
                rgba: p.to_vec(),
            })
            .collect();
        let draws: Vec<_> = sequence
            .iter()
            .enumerate()
            .map(|(i, (_, z, s, b))| full_quad(*z, i, *s, *b))
            .collect();
        let data = gpu
            .render(device, queue, [64, 64], [17, 39, 73, 91], &textures, &draws)?
            .readback(device, queue)?;
        spatial += check_constant(data, pixel_oracle([17, 39, 73, 91], 0., &sequence))?;
    }
    // Overlapping primitives in ONE draw need a fresh destination per primitive.
    let mixed = RawAlphaTexture {
        width: 2,
        height: 1,
        rgba: [green, [200, 20, 40, 64]].concat(),
    };
    for reversed in [false, true] {
        let stage = AlphaStage {
            enabled: true,
            comparison: Greater,
            reference: 127,
            failure: FramebufferOnly,
            depth_write_masked: false,
        };
        let mut first = full_quad(0.8, 1, stage, ByteBlend::SourceOver);
        let mut last = first.clone();
        for v in &mut first.vertices {
            v.uv = [if reversed { 0.75 } else { 0.25 }, 0.5];
        }
        for v in &mut last.vertices {
            v.uv = [if reversed { 0.25 } else { 0.75 }, 0.5];
        }
        first.vertices.extend(last.vertices);
        let sources = if reversed {
            [[200, 20, 40, 64], green]
        } else {
            [green, [200, 20, 40, 64]]
        };
        let sequence = [
            (base, 0.2, pass_stage(), ByteBlend::Overwrite),
            (sources[0], 0.8, stage, ByteBlend::SourceOver),
            (sources[1], 0.8, stage, ByteBlend::SourceOver),
        ];
        let data = gpu
            .render(
                device,
                queue,
                [64, 64],
                [17, 39, 73, 91],
                &[
                    RawAlphaTexture {
                        width: 1,
                        height: 1,
                        rgba: base.to_vec(),
                    },
                    mixed.clone(),
                ],
                &[full_quad(0.2, 0, pass_stage(), ByteBlend::Overwrite), first],
            )?
            .readback(device, queue)?;
        spatial += check_constant(data, pixel_oracle([17, 39, 73, 91], 0., &sequence))?;
    }
    // Explicit culling must not silently affect existing two-sided fixtures.
    for cull in [false, true] {
        for reversed in [false, true] {
            let mut draw = full_quad(0.5, 0, pass_stage(), ByteBlend::Overwrite);
            draw.cull_backfaces = cull;
            if reversed {
                for triangle in draw.vertices.chunks_exact_mut(3) {
                    triangle.swap(1, 2);
                }
            }
            let image = RawAlphaTexture {
                width: 1,
                height: 1,
                rgba: vec![123, 45, 67, 128],
            };
            let data = gpu
                .render(device, queue, [64, 64], [17, 39, 73, 91], &[image], &[draw])?
                .readback(device, queue)?;
            let expected = if cull && reversed {
                ([17, 39, 73, 91], 0.)
            } else {
                ([123, 45, 67, 128], 0.5)
            };
            spatial += check_constant(data, expected)?;
        }
    }
    Ok((arithmetic, spatial))
}

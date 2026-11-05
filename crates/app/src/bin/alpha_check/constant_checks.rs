//! Original mathematical constant-channel fixtures; no game geometry or pixels.
use super::*;
fn stage(tested: bool) -> AlphaStage {
    AlphaStage {
        enabled: tested,
        comparison: Greater,
        reference: 127,
        failure: FramebufferOnly,
        depth_write_masked: false,
    }
}
fn expected(color: [f32; 4]) -> [u8; 4] {
    color.map(|v| v.clamp(0., 255.).floor() as u8)
}
fn full_quad(size: f32, ws: [f32; 4], color: [f32; 4], tested: bool) -> AlphaDraw {
    let mut d = quad([-size, -size, size, size], 0.5, 0, stage(tested), color);
    // Same four screen corners, unrelated positive homogeneous W values.
    for (v, &i) in d.vertices.iter_mut().zip([0, 1, 2, 0, 2, 3].iter()) {
        for c in &mut v.clip[..3] {
            *c *= ws[i];
        }
        v.clip[3] = ws[i];
    }
    d
}
fn check_pixel(
    data: &AlphaReadback,
    i: usize,
    color: [f32; 4],
    tested: bool,
    variable_depth: bool,
) -> GpuResult<()> {
    let c = expected(color);
    let zpass = !tested || c[3] > 127;
    let depth_ok = if zpass {
        if variable_depth {
            data.depth[i] > 0. && data.depth[i] <= 1.
        } else {
            (data.depth[i] - 0.5).abs() < 1e-6
        }
    } else {
        data.depth[i] == 0.
    };
    if data.color[i] != [c[0], c[1], c[2], 255] || data.framebuffer_alpha[i] != c[3] || !depth_ok {
        return Err(format!("constant channel drift at{i}: source{color:?}, RGB{:?}, alpha{}, depth{}; expected{c:?}, depth permission{zpass}",data.color[i],data.framebuffer_alpha[i],data.depth[i]).into());
    }
    Ok(())
}
pub fn run(
    gpu: &mut OrderedAlphaGpu,
    device: &RenderDevice,
    queue: &RenderQueue,
    quick: bool,
) -> GpuResult<u64> {
    let texture = RawAlphaTexture {
        width: 1,
        height: 1,
        rgba: vec![128; 4],
    };
    let mut checks = 0;
    let below = f32::from_bits(128f32.to_bits() - 1);
    let alphas = if quick {
        vec![128., below, 64.]
    } else {
        vec![128., below, 0., 1., 64., 96.9, 127., 255.]
    };
    for size in [[64, 64], [512, 448]] {
        for &alpha in &alphas {
            for tested in [true, false] {
                let color = [73.3, 129.1, f32::from_bits(204f32.to_bits() - 1), alpha];
                for (extent, ws) in [(2., [0.75, 3.25, 11.5, 1.125]), (3., [1.1, 7.3, 0.9, 4.7])] {
                    let d = full_quad(extent, ws, color, tested);
                    let data = gpu
                        .render(
                            device,
                            queue,
                            size,
                            [17, 39, 73, 91],
                            std::slice::from_ref(&texture),
                            &[d],
                        )?
                        .readback(device, queue)?;
                    for i in 0..data.color.len() {
                        check_pixel(&data, i, color, tested, false)?;
                        checks += 1;
                    }
                }
                // One original synthetic source corner is behind the camera.
                let vertices = [
                    [-2.2, -1.7, 0.01, 1.3],
                    [2.7, -1.3, 0.01, 3.1],
                    [0.1, 1.2, 0.01, -0.7],
                ]
                .map(|clip| AlphaVertex {
                    clip,
                    uv: [0.5, 0.5],
                    color,
                })
                .to_vec();
                let d = AlphaDraw {
                    vertices,
                    texture: 0,
                    stage: stage(tested),
                    blend: ByteBlend::Overwrite,
                    cull_backfaces: false,
                    address: [TextureWrap::Clamp; 2],
                };
                let data = gpu
                    .render(
                        device,
                        queue,
                        size,
                        [17, 39, 73, 91],
                        std::slice::from_ref(&texture),
                        &[d],
                    )?
                    .readback(device, queue)?;
                let mut covered = 0;
                for i in 0..data.color.len() {
                    if data.color[i] != [17, 39, 73, 255] {
                        check_pixel(&data, i, color, tested, true)?;
                        covered += 1;
                    } else if data.framebuffer_alpha[i] != 91 || data.depth[i] != 0. {
                        return Err("constant fixture clear pixel wrote alpha/depth".into());
                    }
                    checks += 1;
                }
                if covered < 128 {
                    return Err("constant clipped fixture coverage unexpectedly empty".into());
                }
            }
        }
    }
    // Different triangles in one draw have different exact constants.
    for size in [[64, 64], [512, 448]] {
        let colors = [[12., 34., 56., 128.], [81., 99., 117., 64.]];
        let mut d = full_quad(2., [0.75, 3.25, 11.5, 1.125], colors[0], true);
        for v in &mut d.vertices[3..] {
            v.color = colors[1];
        }
        let data = gpu
            .render(
                device,
                queue,
                size,
                [17, 39, 73, 91],
                std::slice::from_ref(&texture),
                &[d],
            )?
            .readback(device, queue)?;
        let w = size[0] as usize;
        let h = size[1] as usize;
        for y in 0..h {
            for x in 0..w {
                let side = (2 * x + 1) * h + (2 * y + 1) * w;
                let boundary = 2 * w * h;
                let i = y * w + x;
                let candidates = if side == boundary {
                    &colors[..]
                } else {
                    &colors[usize::from(side < boundary)..usize::from(side < boundary) + 1]
                };
                if !candidates
                    .iter()
                    .any(|c| check_pixel(&data, i, *c, true, false).is_ok())
                {
                    return Err(format!(
                        "per-triangle constant metadata leaked across primitive at{x},{y}"
                    )
                    .into());
                }
                checks += 1;
            }
        }
    }
    Ok(checks)
}

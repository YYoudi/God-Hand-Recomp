//! Explicit all-resource uniform-state experiment; not recovered runtime material mapping.
use bevy::{
    asset::RenderAssetUsages,
    camera::CameraProjection,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use godhand_disc::{Disc, afs::AfsCatalog, fst::FstCatalog};
use godhand_engine::{
    byte_blend::ByteBlend,
    fragment_alpha::{AlphaComparison, AlphaFailure, AlphaStage},
    inspection_camera::{self, FAR, InspectionCamera, NEAR},
    placement::EulerOrder,
};
use godhand_lab::{alpha_gpu::*, offscreen, room};
use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};
struct Options {
    iso: PathBuf,
    room: String,
    camera: InspectionCamera,
    size: [u32; 2],
    blend: ByteBlend,
    failure: AlphaFailure,
    output: PathBuf,
    cull: bool,
    order: Option<EulerOrder>,
}
fn parse(args: Vec<OsString>) -> GpuResult<Options> {
    let mut values = BTreeMap::new();
    let mut args = args.into_iter();
    let mut cull = false;
    while let Some(arg) = args.next() {
        let key = arg.into_string().map_err(|_| "option must be UTF-8")?;
        if key == "--backface-culling" && !cull {
            cull = true;
            continue;
        }
        if ![
            "--iso",
            "--room",
            "--eye",
            "--target",
            "--fov",
            "--aspect",
            "--size",
            "--blend",
            "--alpha-failure",
            "--capture-png",
            "--rotation-order",
        ]
        .contains(&key.as_str())
        {
            return Err("unknown/duplicate control option".into());
        }
        let value = args.next().ok_or("control option requires value")?;
        if values.insert(key, value).is_some() {
            return Err("duplicate control option".into());
        }
    }
    let order = values
        .remove("--rotation-order")
        .map(|v| {
            v.into_string()
                .map_err(|_| "invalid rotation order")
                .and_then(|v| v.parse().map_err(|_| "invalid rotation order"))
        })
        .transpose()?;
    let mut value = |key: &str| -> GpuResult<OsString> {
        Ok(values
            .remove(key)
            .ok_or_else(|| format!("required control option {key}"))?)
    };
    let iso = value("--iso")?.into();
    let output: PathBuf = value("--capture-png")?.into();
    if output.extension().is_none_or(|e| e != "png") {
        return Err("new output must have .png extension".into());
    }
    let room = value("--room")?
        .into_string()
        .map_err(|_| "room must be UTF-8")?;
    let eye = inspection_camera::parse_vec3(value("--eye")?.to_str().ok_or("invalid eye")?)?;
    let target =
        inspection_camera::parse_vec3(value("--target")?.to_str().ok_or("invalid target")?)?;
    let fov = value("--fov")?.to_str().ok_or("invalid FOV")?.parse()?;
    let aspect = value("--aspect")?
        .to_str()
        .ok_or("invalid aspect")?
        .parse()?;
    let camera = InspectionCamera::new(eye, target, fov)?.with_projection_aspect(aspect)?;
    let size =
        inspection_camera::parse_view_size(value("--size")?.to_str().ok_or("invalid size")?)?;
    if size.iter().any(|v| !(64..=1024).contains(v)) {
        return Err("control viewport must be within64..1024".into());
    }
    let blend = match value("--blend")?.to_str() {
        Some("overwrite") => ByteBlend::Overwrite,
        Some("source-over") => ByteBlend::SourceOver,
        _ => return Err("blend must be overwrite or source-over".into()),
    };
    let failure = match value("--alpha-failure")?.to_str() {
        Some("keep") => AlphaFailure::Keep,
        Some("framebuffer-only") => AlphaFailure::FramebufferOnly,
        _ => return Err("failure must be keep or framebuffer-only".into()),
    };
    Ok(Options {
        iso,
        room,
        camera,
        size,
        blend,
        failure,
        output,
        cull,
        order,
    })
}
fn main() {
    if let Err(e) = run() {
        eprintln!("godhand-static-control: {e}");
        std::process::exit(1);
    }
}
fn run() -> GpuResult<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args == [OsString::from("--help")] {
        println!(
            "godhand-static-control --iso <personal ISO> --room <exact DAT path> --eye X,Y,Z --target X,Y,Z --fov 10..120 --aspect 0.25..4 --size W,H --blend overwrite/source-over --alpha-failure keep/framebuffer-only --capture-png <NEW PNG> [--backface-culling] [--rotation-order xyz/... ]\nNo-window all-resource UNIFORM-STATE CONTROL, not recovered materials. Fixed GT127, GEQUAL/unmasked depth, raw textures, nearest/clamp, source colors with reported neutral fallback. Storage ordering is NOT runtime ordering; no scene activation/fidelity claim."
        );
        return Ok(());
    }
    let o = parse(args)?;
    if o.output.exists() {
        return Err("refusing to replace existing capture".into());
    }
    let mut disc = Disc::open(&o.iso)?;
    disc.validate_godhand_usa()?;
    let fst = FstCatalog::read(disc.open_root_file("GODHAND.FST")?)?;
    let mut archive = disc.open_root_file("_DATA.AFS")?;
    let afs = AfsCatalog::read(&mut archive)?;
    fst.validate_archive(&afs)?;
    let file = fst.file(&o.room)?;
    let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
    let (view, summary) = room::load_layout_raw(&mut member, o.order, &[])?;
    let world_from_view = Transform::from_translation(Vec3::from_array(o.camera.eye()))
        .looking_to(Vec3::from_array(o.camera.forward()), Vec3::Y)
        .to_matrix();
    let projection = PerspectiveProjection {
        fov: o.camera.fov_radians(),
        near: NEAR,
        far: FAR,
        aspect_ratio: o.camera.projection_aspect().unwrap(),
        ..default()
    };
    let clip_from_world = projection.get_clip_from_view() * world_from_view.inverse();
    let bindings: BTreeMap<_, _> = view
        .textures
        .keys()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();
    let textures: Vec<_> = view
        .textures
        .into_values()
        .map(RawAlphaTexture::from)
        .collect();
    let mut draws = Vec::new();
    let mut missing_colors = 0;
    for primitive in view.mesh.primitives {
        if primitive.colors.is_none() {
            missing_colors += 1;
        }
        let vertices = primitive
            .indices
            .iter()
            .map(|&i| {
                let i = i as usize;
                let p = Vec3::from_array(primitive.positions[i]).extend(1.);
                AlphaVertex {
                    clip: (clip_from_world * p).to_array(),
                    uv: primitive.uvs[i],
                    color: primitive
                        .colors
                        .as_ref()
                        .map_or([128.; 4], |colors| colors[i].map(f32::from)),
                }
            })
            .collect();
        draws.push(AlphaDraw {
            vertices,
            texture: bindings[&primitive.material],
            stage: AlphaStage {
                enabled: true,
                comparison: AlphaComparison::Greater,
                reference: 127,
                failure: o.failure,
                depth_write_masked: false,
            },
            blend: o.blend,
            cull_backfaces: o.cull,
        });
    }
    validate_inputs(o.size, &textures, &draws)?;
    println!(
        "UNIFORM STATE CONTROL: {} nodes /{} buffers /{} triangles /{} images; missing-color neutral fallback on{} buffers. Blend {:?}; failure {:?}; GT127; GEQUAL/unmasked; cull {}; nearest/clamp; storage order. NO recovered material mapping or scene activation.",
        summary.nodes,
        summary.buffers,
        summary.indices / 3,
        textures.len(),
        missing_colors,
        o.blend,
        o.failure,
        o.cull
    );
    let (_app, device, queue) = offscreen::setup()?;
    let mut gpu = OrderedAlphaGpu::new(&device);
    let frame = gpu
        .render(&device, &queue, o.size, [0, 0, 0, 0], &textures, &draws)?
        .readback(&device, &queue)?;
    let extent = Extent3d {
        width: o.size[0],
        height: o.size[1],
        depth_or_array_layers: 1,
    };
    let bytes = frame.color.into_iter().flatten().collect();
    let image = Image::new(
        extent,
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
    .try_into_dynamic()?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&o.output)?;
    let format = bevy::image::ImageFormat::Png
        .as_image_crate_format()
        .ok_or("PNG unavailable")?;
    image.write_to(&mut file, format)?;
    println!(
        "Original offscreen renderer PNG captured: {}",
        o.output.display()
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn args() -> Vec<OsString> {
        "--iso personal.iso --room st1/candidate.dat --eye 0,1,4 --target 0,1,0 --fov 31 --aspect 1.3333333333 --size 512,448 --blend source-over --alpha-failure keep --capture-png original.png".split_whitespace().map(OsString::from).collect()
    }
    #[test]
    fn control_requires_explicit_state_and_finite_camera() {
        let good = parse(args()).unwrap();
        assert_eq!(good.blend, ByteBlend::SourceOver);
        assert!(!good.cull);
        assert_eq!(good.size, [512, 448]);
        for (key, value) in [
            ("--fov", "NaN"),
            ("--aspect", "0"),
            ("--size", "2048,448"),
            ("--blend", "auto"),
            ("--alpha-failure", "auto"),
            ("--capture-png", "old.jpg"),
        ] {
            let mut bad = args();
            let i = bad.iter().position(|v| v == key).unwrap();
            bad[i + 1] = value.into();
            assert!(parse(bad).is_err());
        }
    }
    #[test]
    fn duplicate_unknown_and_missing_control_options_are_refused() {
        for tail in [
            vec!["--fov", "45"],
            vec!["--unknown", "x"],
            vec!["--backface-culling", "--backface-culling"],
        ] {
            let mut bad = args();
            bad.extend(tail.into_iter().map(OsString::from));
            assert!(parse(bad).is_err());
        }
        assert!(parse(Vec::new()).is_err());
        let mut bad = args();
        bad.pop();
        assert!(parse(bad).is_err());
    }
}

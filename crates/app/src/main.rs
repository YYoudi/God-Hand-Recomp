use godhand_disc::Disc;
use godhand_disc::afs::AfsCatalog;
use godhand_disc::{fst::FstCatalog, tim2};
use std::{env, error::Error, path::PathBuf};

#[cfg(feature = "viewer")]
mod color_material;
use godhand_lab::room;
#[cfg(feature = "viewer")]
mod viewer;

fn main() {
    if let Err(error) = run() {
        eprintln!("godhand-lab: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let mut iso = None;
    let mut sandbox = false;
    let mut smoke_seconds = None;
    let mut catalog = false;
    let mut list = false;
    let mut texture = None;
    let mut inspect_textures = false;
    let mut room_path = None;
    let mut inspect_rooms = false;
    let mut inspect_mesh_path = None;
    let mut mesh_path = None;
    let mut layout_path = None;
    let mut inspect_layout_path = None;
    let mut rotation_order = None;
    let mut capture_png = None;
    let mut excluded_collections = Vec::new();
    let mut camera_eye = None;
    let mut camera_target = None;
    let mut camera_fov = None;
    let mut camera_aspect = None;
    let mut view_size = [1280, 720];
    let mut static_colors = false;
    let mut backface_culling = false;
    let mut collection = 1usize;
    let mut node = 0usize;
    while let Some(arg) = args.next() {
        if arg == "--iso" {
            iso = Some(PathBuf::from(args.next().ok_or("--iso needs a path")?));
        } else if arg == "--sandbox" {
            sandbox = true;
        } else if arg == "--static-colors" {
            static_colors = true;
        } else if arg == "--catalog" {
            catalog = true;
        } else if arg == "--list" {
            list = true;
        } else if arg == "--texture" {
            texture = Some(
                args.next()
                    .ok_or("--texture needs an exact FST path")?
                    .into_string()
                    .map_err(|_| "texture path must be UTF-8")?,
            );
        } else if arg == "--inspect-textures" {
            inspect_textures = true;
        } else if arg == "--room" {
            room_path = Some(
                args.next()
                    .ok_or("--room needs an exact FST path")?
                    .into_string()
                    .map_err(|_| "room path must be UTF-8")?,
            );
        } else if arg == "--inspect-rooms" {
            inspect_rooms = true;
        } else if arg == "--inspect-meshes"
            || arg == "--mesh"
            || arg == "--layout"
            || arg == "--inspect-layout"
        {
            let value = args
                .next()
                .ok_or("mesh option needs an exact FST path")?
                .into_string()
                .map_err(|_| "mesh path must be UTF-8")?;
            if arg == "--mesh" {
                mesh_path = Some(value);
            } else if arg == "--layout" {
                layout_path = Some(value);
            } else if arg == "--inspect-layout" {
                inspect_layout_path = Some(value);
            } else {
                inspect_mesh_path = Some(value);
            }
        } else if arg == "--exclude-collection" {
            let ordinal: usize = args
                .next()
                .ok_or("--exclude-collection needs an ordinal")?
                .to_str()
                .ok_or("invalid excluded ordinal")?
                .parse()?;
            if excluded_collections.len() >= 4096 {
                return Err("too many excluded collections".into());
            }
            excluded_collections.push(ordinal);
        } else if arg == "--rotation-order" {
            rotation_order = Some(
                args.next()
                    .ok_or("--rotation-order needs a sequence")?
                    .to_str()
                    .ok_or("invalid rotation order")?
                    .parse::<godhand_engine::placement::EulerOrder>()?,
            );
        } else if arg == "--collection" || arg == "--node" {
            let value: usize = args
                .next()
                .ok_or("selection needs an ordinal")?
                .to_str()
                .ok_or("invalid selection")?
                .parse()?;
            if arg == "--collection" {
                collection = value;
            } else {
                node = value;
            }
        } else if arg == "--backface-culling" {
            backface_culling = true;
        } else if arg == "--camera-eye"
            || arg == "--camera-target"
            || arg == "--camera-fov"
            || arg == "--camera-aspect"
            || arg == "--view-size"
        {
            let value = args.next().ok_or("camera/view option needs a value")?;
            let value = value.to_str().ok_or("invalid camera/view value")?;
            match arg.to_str().unwrap() {
                "--camera-eye" => {
                    camera_eye = Some(godhand_engine::inspection_camera::parse_vec3(value)?)
                }
                "--camera-target" => {
                    camera_target = Some(godhand_engine::inspection_camera::parse_vec3(value)?)
                }
                "--camera-fov" => camera_fov = Some(value.parse::<f32>()?),
                "--camera-aspect" => camera_aspect = Some(value.parse::<f32>()?),
                _ => view_size = godhand_engine::inspection_camera::parse_view_size(value)?,
            }
        } else if arg == "--capture-png" {
            capture_png = Some(PathBuf::from(
                args.next().ok_or("--capture-png needs a new file path")?,
            ));
        } else if arg == "--smoke-seconds" {
            let value = args.next().ok_or("--smoke-seconds needs a value")?;
            let seconds: u64 = value.to_str().ok_or("invalid duration")?.parse()?;
            if !(1..=120).contains(&seconds) {
                return Err("--smoke-seconds must be between 1 and 120".into());
            }
            smoke_seconds = Some(seconds);
        } else if arg == "--help" || arg == "-h" {
            println!(
                "godhand-lab --iso <original USA disc.iso> [--catalog] [--list] [--inspect-textures] [--inspect-rooms] [--room <FST path>] [--inspect-meshes <FST path>] [--inspect-layout <FST path>] [--exclude-collection N]... [--rotation-order xyz/xzy/yxz/yzx/zxy/zyx] [--layout <FST path> | --texture <FST path> | --mesh <FST path> [--collection N --node N] | --sandbox] [--smoke-seconds 1..120] [--capture-png <new PNG path>] [--camera-eye X,Y,Z --camera-target X,Y,Z [--camera-fov 10..120] [--camera-aspect 0.25..4]] [--view-size W,H] [--static-colors] [--backface-culling]\nViews require --features viewer. Mesh view is local; layout view applies explicit placement hypotheses, not verified runtime scene activation. No game lighting. Multiaxis placement requires an explicit rotation order. Game assets stay in your own ISO."
            );
            return Ok(());
        } else {
            return Err(format!("unknown argument: {}", arg.to_string_lossy()).into());
        }
    }
    if usize::from(sandbox)
        + usize::from(texture.is_some())
        + usize::from(mesh_path.is_some())
        + usize::from(layout_path.is_some())
        > 1
    {
        return Err("choose one viewer mode: --sandbox, --texture, --mesh or --layout".into());
    }
    if smoke_seconds.is_some()
        && !sandbox
        && texture.is_none()
        && mesh_path.is_none()
        && layout_path.is_none()
    {
        return Err("--smoke-seconds requires a viewer mode".into());
    }
    if (rotation_order.is_some() || !excluded_collections.is_empty())
        && layout_path.is_none()
        && inspect_layout_path.is_none()
    {
        return Err("placement options require --layout or --inspect-layout".into());
    }
    if layout_path.is_some() && inspect_layout_path.is_some() {
        return Err("choose --layout or --inspect-layout, not both".into());
    }
    if capture_png.is_some() && smoke_seconds.is_none_or(|v| v < 10) {
        return Err("--capture-png requires --smoke-seconds 10..120 and a viewer mode".into());
    }
    if (static_colors || backface_culling) && mesh_path.is_none() && layout_path.is_none() {
        return Err("--static-colors / --backface-culling require --mesh or --layout".into());
    }
    let camera_pose = match (camera_eye, camera_target) {
        (Some(eye), Some(target)) => {
            if mesh_path.is_none() && layout_path.is_none() {
                return Err("explicit camera requires --mesh or --layout".into());
            }
            let mut camera = godhand_engine::inspection_camera::InspectionCamera::new(
                eye,
                target,
                camera_fov.unwrap_or(45.),
            )?;
            if let Some(aspect) = camera_aspect {
                camera = camera.with_projection_aspect(aspect)?;
            }
            Some(camera)
        }
        (None, None) if camera_fov.is_none() && camera_aspect.is_none() => None,
        _ => {
            return Err(
                "camera eye and target must be provided together; FOV/aspect require a pose".into(),
            );
        }
    };
    if view_size != [1280, 720]
        && !sandbox
        && texture.is_none()
        && mesh_path.is_none()
        && layout_path.is_none()
    {
        return Err("--view-size requires a viewer mode".into());
    }
    if camera_pose.is_some() || view_size != [1280, 720] {
        println!("Requested diagnostic camera: {camera_pose:?}; physical viewport {view_size:?}");
    }
    let path = iso.ok_or("original ISO required: use --iso <path>")?;
    let mut disc = Disc::open(path)?;
    disc.validate_godhand_usa()?;
    println!("USA boot serial recognized. ISO opened read-only. No game code executed.");
    let count = disc.root_entries()?.len();
    println!("Volume: {} | root records: {}", disc.volume_id, count);
    if catalog {
        let index = AfsCatalog::read(disc.open_root_file("_DATA.AFS")?)?;
        println!(
            "AFS catalog: {} entries in {} bytes; metadata only, no extraction.",
            index.entries.len(),
            index.archive_size
        );
    }
    if list
        || inspect_textures
        || texture.is_some()
        || room_path.is_some()
        || inspect_rooms
        || inspect_mesh_path.is_some()
        || mesh_path.is_some()
        || layout_path.is_some()
        || inspect_layout_path.is_some()
    {
        let fst = FstCatalog::read(disc.open_root_file("GODHAND.FST")?)?;
        let mut archive = disc.open_root_file("_DATA.AFS")?;
        let afs = AfsCatalog::read(&mut archive)?;
        fst.validate_archive(&afs)?;
        println!(
            "FST: {} files, {} directories; AFS ordinals cover the archive exactly once.",
            fst.files.len(),
            fst.directories.len()
        );
        if list {
            for file in &fst.files {
                println!(
                    "{}\t{}\t{}",
                    file.ordinal, afs.entries[file.ordinal as usize].size, file.path
                );
            }
        }
        if inspect_rooms {
            let mut rooms = 0;
            let mut nested_catalogs = 0;
            let mut nested_sections = 0;
            for file in fst
                .files
                .iter()
                .filter(|f| f.path.starts_with("st") && f.path.ends_with(".dat"))
            {
                let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
                let result = room::inspect(&mut member, false, false)?;
                rooms += 1;
                nested_catalogs += result.nested_catalogs;
                nested_sections += result.nested_sections;
            }
            println!(
                "Room-candidate catalogs: {rooms} outer DAT, {nested_catalogs} nonempty nested SCP, {nested_sections} nested storage sections. Metadata only; no scene reconstruction."
            );
        }
        if let Some(path) = room_path {
            let file = fst.file(&path)?;
            let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
            let result = room::inspect(&mut member, true, true)?;
            println!(
                "Room candidate {path}: {} outer sections, {} nested sections, {} SCR collections, {} nodes; geometry and materials not decoded.",
                result.outer_sections, result.nested_sections, result.scr_collections, result.nodes
            );
        }
        if let Some(path) = inspect_mesh_path {
            let file = fst.file(&path)?;
            let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
            room::inspect_meshes(&mut member)?;
        }
        let display_layout = layout_path.is_some();
        if let Some(path) = layout_path.or(inspect_layout_path) {
            let file = fst.file(&path)?;
            let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
            let (view, summary) =
                room::load_layout(&mut member, rotation_order, &excluded_collections)?;
            println!(
                "Candidate layout {path}: {} collections, {} nodes, {} buffers, {} vertices, {} triangles, {} referenced images. Order: {rotation_order:?}; explicitly excluded: {excluded_collections:?}. S -> R -> T, unscaled translation hypothesis; runtime parity and resource activation unverified.",
                summary.collections,
                summary.nodes,
                summary.buffers,
                summary.vertices,
                summary.indices / 3,
                view.textures.len()
            );
            if display_layout {
                #[cfg(feature = "viewer")]
                viewer::run(
                    smoke_seconds,
                    None,
                    Some(view),
                    capture_png.clone(),
                    camera_pose,
                    view_size,
                    static_colors,
                    backface_culling,
                )?;
                #[cfg(not(feature = "viewer"))]
                return Err(
                    "rebuild with --features viewer to display the candidate layout".into(),
                );
            }
        }
        if let Some(path) = mesh_path {
            let file = fst.file(&path)?;
            let mut member = afs.open_entry(&mut archive, file.ordinal as usize)?;
            let view = room::load_mesh(&mut member, collection, node)?;
            println!(
                "Isolated mesh: {path}, SCP collection {collection}, node {node}, {} buffers, {} referenced textures; no scene transforms or game lighting.",
                view.mesh.primitives.len(),
                view.textures.len()
            );
            #[cfg(feature = "viewer")]
            viewer::run(
                smoke_seconds,
                None,
                Some(view),
                capture_png.clone(),
                camera_pose,
                view_size,
                static_colors,
                backface_culling,
            )?;
            #[cfg(not(feature = "viewer"))]
            return Err("rebuild with --features viewer to display the decoded mesh".into());
        }
        if inspect_textures {
            let mut accepted = 0;
            let mut refused = 0;
            for file in fst.files.iter().filter(|file| file.path.ends_with(".tm2")) {
                match tim2::decode(afs.open_entry(&mut archive, file.ordinal as usize)?) {
                    Ok(image) => {
                        accepted += 1;
                        println!(
                            "TIM2 {}: {}x{} decoded",
                            file.path, image.width, image.height
                        );
                    }
                    Err(error) => {
                        refused += 1;
                        println!("TIM2 {}: refused: {error}", file.path);
                    }
                }
            }
            println!(
                "TIM2 inspection: {accepted} decoded, {refused} refused. No files extracted; no reference-renderer parity claimed."
            );
        }
        if let Some(path) = texture {
            let file = fst.file(&path)?;
            let image = tim2::decode(afs.open_entry(&mut archive, file.ordinal as usize)?)?;
            println!(
                "Texture: {path}, {}x{} RGBA; isolated inspection view.",
                image.width, image.height
            );
            #[cfg(feature = "viewer")]
            viewer::run(
                smoke_seconds,
                Some(image),
                None,
                capture_png.clone(),
                camera_pose,
                view_size,
                static_colors,
                backface_culling,
            )?;
            #[cfg(not(feature = "viewer"))]
            return Err("rebuild with --features viewer to display the decoded texture".into());
        }
    }
    if sandbox {
        #[cfg(feature = "viewer")]
        viewer::run(
            smoke_seconds,
            None,
            None,
            capture_png.clone(),
            camera_pose,
            view_size,
            static_colors,
            backface_culling,
        )?;
        #[cfg(not(feature = "viewer"))]
        return Err("rebuild with --features viewer to open the synthetic sandbox".into());
    }
    Ok(())
}

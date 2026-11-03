use godhand_disc::Disc;
use godhand_disc::afs::AfsCatalog;
use godhand_disc::{fst::FstCatalog, tim2};
use std::{env, error::Error, path::PathBuf};

mod room;
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
    while let Some(arg) = args.next() {
        if arg == "--iso" {
            iso = Some(PathBuf::from(args.next().ok_or("--iso needs a path")?));
        } else if arg == "--sandbox" {
            sandbox = true;
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
        } else if arg == "--smoke-seconds" {
            let value = args.next().ok_or("--smoke-seconds needs a value")?;
            let seconds: u64 = value.to_str().ok_or("invalid duration")?.parse()?;
            if !(1..=120).contains(&seconds) {
                return Err("--smoke-seconds must be between 1 and 120".into());
            }
            smoke_seconds = Some(seconds);
        } else if arg == "--help" || arg == "-h" {
            println!(
                "godhand-lab --iso <original USA disc.iso> [--catalog] [--list] [--inspect-textures] [--inspect-rooms] [--room <FST path>] [--texture <FST path> | --sandbox] [--smoke-seconds 1..120]\nRoom options inspect metadata only. Texture and synthetic sandbox windows require --features viewer. Game assets stay in your own ISO."
            );
            return Ok(());
        } else {
            return Err(format!("unknown argument: {}", arg.to_string_lossy()).into());
        }
    }
    if sandbox && texture.is_some() {
        return Err("choose either --sandbox or --texture".into());
    }
    if smoke_seconds.is_some() && !sandbox && texture.is_none() {
        return Err("--smoke-seconds requires --sandbox or --texture".into());
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
    if list || inspect_textures || texture.is_some() || room_path.is_some() || inspect_rooms {
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
            viewer::run(smoke_seconds, Some(image))?;
            #[cfg(not(feature = "viewer"))]
            return Err("rebuild with --features viewer to display the decoded texture".into());
        }
    }
    if sandbox {
        #[cfg(feature = "viewer")]
        viewer::run(smoke_seconds, None)?;
        #[cfg(not(feature = "viewer"))]
        return Err("rebuild with --features viewer to open the synthetic sandbox".into());
    }
    Ok(())
}

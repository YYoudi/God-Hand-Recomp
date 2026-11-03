use godhand_disc::Disc;
use godhand_disc::afs::AfsCatalog;
use std::{env, error::Error, path::PathBuf};

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
    while let Some(arg) = args.next() {
        if arg == "--iso" {
            iso = Some(PathBuf::from(args.next().ok_or("--iso needs a path")?));
        } else if arg == "--sandbox" {
            sandbox = true;
        } else if arg == "--catalog" {
            catalog = true;
        } else if arg == "--smoke-seconds" {
            let value = args.next().ok_or("--smoke-seconds needs a value")?;
            let seconds: u64 = value.to_str().ok_or("invalid duration")?.parse()?;
            if !(1..=120).contains(&seconds) {
                return Err("--smoke-seconds must be between 1 and 120".into());
            }
            smoke_seconds = Some(seconds);
        } else if arg == "--help" || arg == "-h" {
            println!(
                "godhand-lab --iso <original USA disc.iso> [--catalog] [--sandbox [--smoke-seconds 1..120]]\nSandbox requires --features viewer. No game assets are loaded yet."
            );
            return Ok(());
        } else {
            return Err(format!("unknown argument: {}", arg.to_string_lossy()).into());
        }
    }
    if smoke_seconds.is_some() && !sandbox {
        return Err("--smoke-seconds requires --sandbox".into());
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
    if sandbox {
        #[cfg(feature = "viewer")]
        viewer::run(smoke_seconds)?;
        #[cfg(not(feature = "viewer"))]
        return Err("rebuild with --features viewer to open the synthetic sandbox".into());
    }
    Ok(())
}

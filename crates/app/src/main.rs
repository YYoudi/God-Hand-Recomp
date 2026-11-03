use godhand_disc::Disc;
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
    while let Some(arg) = args.next() {
        if arg == "--iso" {
            iso = Some(PathBuf::from(args.next().ok_or("--iso needs a path")?));
        } else if arg == "--sandbox" {
            sandbox = true;
        } else if arg == "--help" || arg == "-h" {
            println!(
                "godhand-lab --iso <original USA disc.iso> [--sandbox]\nSandbox requires a build with --features viewer. No game assets are loaded yet."
            );
            return Ok(());
        } else {
            return Err(format!("unknown argument: {}", arg.to_string_lossy()).into());
        }
    }
    let path = iso.ok_or("original ISO required: use --iso <path>")?;
    let mut disc = Disc::open(path)?;
    disc.validate_godhand_usa()?;
    println!("USA boot serial recognized. ISO opened read-only. No game code executed.");
    let count = disc.root_entries()?.len();
    println!("Volume: {} | root records: {}", disc.volume_id, count);
    if sandbox {
        #[cfg(feature = "viewer")]
        viewer::run();
        #[cfg(not(feature = "viewer"))]
        return Err("rebuild with --features viewer to open the synthetic sandbox".into());
    }
    Ok(())
}

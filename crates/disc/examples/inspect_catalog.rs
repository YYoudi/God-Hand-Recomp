//! Metadata-only reference-profile inspection. Does not start the game engine.
use godhand_disc::{Disc, afs::AfsCatalog, fst::FstCatalog};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: inspect_catalog <owned ISO9660 image>")?;
    let mut disc = Disc::open(path)?;
    let fst = FstCatalog::read(disc.open_root_file("GODHAND.FST")?)?;
    let afs = AfsCatalog::read(disc.open_root_file("_DATA.AFS")?)?;
    fst.validate_archive(&afs)?;
    println!(
        "{}: {} files, {} directories; all AFS ordinals covered once. Metadata only; edition not authenticated.",
        disc.volume_id,
        fst.files.len(),
        fst.directories.len()
    );
    Ok(())
}

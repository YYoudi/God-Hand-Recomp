//! Read-only room-candidate metadata inspection. No scene or mesh is reconstructed.
use godhand_disc::{dat::DatCatalog, scr::ScrCatalog};
use std::io::{self, Read, Seek};

#[derive(Default)]
pub struct Inspection {
    pub outer_sections: usize,
    pub nested_catalogs: usize,
    pub nested_sections: usize,
    pub scr_collections: usize,
    pub nodes: usize,
}

pub fn inspect(
    source: &mut (impl Read + Seek),
    parse_nodes: bool,
    verbose: bool,
) -> io::Result<Inspection> {
    let outer = DatCatalog::read(&mut *source)?;
    let mut result = Inspection {
        outer_sections: outer.sections().len(),
        ..Default::default()
    };
    for (ordinal, section) in outer.sections().iter().enumerate() {
        if verbose {
            println!(
                "DAT [{ordinal}] {}: {} bytes (storage span)",
                section.tag.escape_ascii(),
                section.size
            );
        }
        if section.tag != *b"SCP\0" || section.size == 0 {
            continue;
        }
        let mut scp = outer.open_section(source, ordinal)?;
        let children = DatCatalog::read(&mut scp)?;
        if children.sections().is_empty() {
            continue;
        }
        result.nested_catalogs += 1;
        result.nested_sections += children.sections().len();
        for (child_ordinal, child) in children.sections().iter().enumerate() {
            if verbose {
                println!(
                    "  SCP [{child_ordinal}] {}: {} bytes (storage span)",
                    child.tag.escape_ascii(),
                    child.size
                );
            }
            if parse_nodes && child.tag == *b"SCR\0" && child.size != 0 {
                let nodes = ScrCatalog::read(children.open_section(&mut scp, child_ordinal)?)?;
                result.scr_collections += 1;
                result.nodes += nodes.nodes.len();
                if verbose {
                    println!(
                        "    SCR: {} bounded nodes; MDB signatures matched; no mesh decoding",
                        nodes.nodes.len()
                    );
                }
            }
        }
    }
    Ok(result)
}

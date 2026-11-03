//! Read-only room-candidate metadata inspection. No scene or mesh is reconstructed.
use godhand_disc::{
    dat::DatCatalog,
    mdb::StaticMesh,
    scr::ScrCatalog,
    tim2::RgbaImage,
    tm3::{self, TextureBank},
};
use std::collections::BTreeMap;
use std::io::{self, Read, Seek};

#[derive(Default)]
pub struct Inspection {
    pub outer_sections: usize,
    pub nested_catalogs: usize,
    pub nested_sections: usize,
    pub scr_collections: usize,
    pub nodes: usize,
}

pub struct MeshView {
    pub mesh: StaticMesh,
    pub textures: BTreeMap<u16, RgbaImage>,
}

fn unique_section(catalog: &DatCatalog, tag: [u8; 4]) -> io::Result<usize> {
    let matches: Vec<_> = catalog
        .sections()
        .iter()
        .enumerate()
        .filter(|(_, s)| s.tag == tag)
        .map(|(i, _)| i)
        .collect();
    if matches.len() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected exactly one resource section for requested tag",
        ));
    }
    Ok(matches[0])
}

pub fn load_mesh(
    source: &mut (impl Read + Seek),
    collection: usize,
    node: usize,
) -> io::Result<MeshView> {
    let outer = DatCatalog::read(&mut *source)?;
    let scp_ordinal = unique_section(&outer, *b"SCP\0")?;
    let mut scp = outer.open_section(source, scp_ordinal)?;
    let children = DatCatalog::read(&mut scp)?;
    let section = children.sections().get(collection).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "SCR collection ordinal outside SCP",
        )
    })?;
    if section.tag != *b"SCR\0" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "selected SCP collection is not SCR",
        ));
    }
    let mesh = {
        let mut scr = children.open_section(&mut scp, collection)?;
        let nodes = ScrCatalog::read(&mut scr)?;
        StaticMesh::read(nodes.open_mdb(&mut scr, node)?)?
    };
    let bank_ordinal = unique_section(&children, *b"TM3\0")?;
    let mut bank_source = children.open_section(&mut scp, bank_ordinal)?;
    let bank = TextureBank::read(&mut bank_source)?;
    let mut textures = BTreeMap::new();
    for primitive in &mesh.primitives {
        if let std::collections::btree_map::Entry::Vacant(entry) =
            textures.entry(primitive.material)
        {
            let image =
                tm3::decode(bank.open_image(&mut bank_source, usize::from(primitive.material))?)?;
            entry.insert(image);
        }
    }
    Ok(MeshView { mesh, textures })
}

pub fn inspect_meshes(source: &mut (impl Read + Seek)) -> io::Result<()> {
    let outer = DatCatalog::read(&mut *source)?;
    let scp_ordinal = unique_section(&outer, *b"SCP\0")?;
    let mut scp = outer.open_section(source, scp_ordinal)?;
    let children = DatCatalog::read(&mut scp)?;
    let bank_ordinal = unique_section(&children, *b"TM3\0")?;
    let texture_count = {
        let mut bank_source = children.open_section(&mut scp, bank_ordinal)?;
        let bank = TextureBank::read(&mut bank_source)?;
        for i in 0..bank.entries().len() {
            tm3::decode(bank.open_image(&mut bank_source, i)?)?;
        }
        bank.entries().len()
    };
    let (mut meshes, mut buffers, mut vertices, mut triangles) = (0, 0, 0, 0);
    for (i, section) in children.sections().iter().enumerate() {
        if section.tag != *b"SCR\0" || section.size == 0 {
            continue;
        }
        let mut scr = children.open_section(&mut scp, i)?;
        let nodes = ScrCatalog::read(&mut scr)?;
        for node in 0..nodes.nodes.len() {
            let mesh = StaticMesh::read(nodes.open_mdb(&mut scr, node)?)?;
            meshes += 1;
            for p in mesh.primitives {
                if usize::from(p.material) >= texture_count {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "mesh material ordinal outside TM3 bank",
                    ));
                }
                buffers += 1;
                vertices += p.positions.len();
                triangles += p.indices.len() / 3;
            }
        }
    }
    println!(
        "Static mesh inspection: {meshes} meshes, {buffers} buffers, {vertices} vertices, {triangles} triangles, {texture_count} decoded bank images; materials in range. Runtime parity unverified."
    );
    Ok(())
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

//! Read-only metadata, isolated meshes and explicit room-layout experiments.
use godhand_disc::{
    dat::DatCatalog,
    mdb::StaticMesh,
    scr::ScrCatalog,
    tim2::RgbaImage,
    tm3::{self, TextureBank},
};
use godhand_engine::placement::{EulerOrder, Placement};
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
    #[cfg_attr(not(feature = "viewer"), allow(dead_code))]
    pub layout: bool,
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
    Ok(MeshView {
        mesh,
        textures,
        layout: false,
    })
}

#[derive(Debug, Default)]
pub struct LayoutSummary {
    pub collections: usize,
    pub nodes: usize,
    pub buffers: usize,
    pub vertices: usize,
    pub indices: usize,
}

impl LayoutSummary {
    fn add(&mut self, mesh: &StaticMesh) -> io::Result<()> {
        self.nodes += 1;
        self.buffers += mesh.primitives.len();
        self.vertices += mesh
            .primitives
            .iter()
            .map(|p| p.positions.len())
            .sum::<usize>();
        self.indices += mesh
            .primitives
            .iter()
            .map(|p| p.indices.len())
            .sum::<usize>();
        if self.nodes > 4096
            || self.buffers > 8192
            || self.vertices > 1_048_576
            || self.indices > 3_145_728
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "layout exceeds aggregate geometry budget",
            ));
        }
        Ok(())
    }
}

fn place_mesh(mesh: &mut StaticMesh, placement: &Placement) -> io::Result<()> {
    for primitive in &mut mesh.primitives {
        for point in &mut primitive.positions {
            *point = placement
                .point(*point)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
    }
    Ok(())
}

/// Places all resource nodes as a hypothesis; does not establish scene activation.
pub fn load_layout(
    source: &mut (impl Read + Seek),
    order: Option<EulerOrder>,
    excluded: &[usize],
) -> io::Result<(MeshView, LayoutSummary)> {
    let outer = DatCatalog::read(&mut *source)?;
    let scp_ordinal = unique_section(&outer, *b"SCP\0")?;
    let mut scp = outer.open_section(source, scp_ordinal)?;
    let children = DatCatalog::read(&mut scp)?;
    for &ordinal in excluded {
        if !children
            .sections()
            .get(ordinal)
            .is_some_and(|s| s.tag == *b"SCR\0" && s.size != 0)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excluded ordinal is not a nonempty SCR section",
            ));
        }
    }
    let mut summary = LayoutSummary::default();
    let mut mesh = StaticMesh {
        primitives: Vec::new(),
    };
    for (ordinal, section) in children.sections().iter().enumerate() {
        if section.tag != *b"SCR\0" || section.size == 0 || excluded.contains(&ordinal) {
            continue;
        }
        let mut scr = children.open_section(&mut scp, ordinal)?;
        let nodes = ScrCatalog::read(&mut scr)?;
        if summary.nodes + nodes.nodes.len() > 4096 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "layout exceeds aggregate node budget",
            ));
        }
        summary.collections += 1;
        for (node_ordinal, node) in nodes.nodes.iter().enumerate() {
            let decoded = (|| {
                let transform = Placement::new(node.scale, node.rotation, node.translation, order)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let mut decoded = StaticMesh::read(nodes.open_mdb(&mut scr, node_ordinal)?)?;
                summary.add(&decoded)?;
                place_mesh(&mut decoded, &transform)?;
                Ok::<_, io::Error>(decoded)
            })()
            .map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("collection {ordinal}, node {node_ordinal}: {e}"),
                )
            })?;
            mesh.primitives.extend(decoded.primitives);
        }
    }
    if summary.nodes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no SCR nodes remain in selected layout",
        ));
    }
    let bank_ordinal = unique_section(&children, *b"TM3\0")?;
    let mut bank_source = children.open_section(&mut scp, bank_ordinal)?;
    let bank = TextureBank::read(&mut bank_source)?;
    let mut textures = BTreeMap::new();
    let mut bytes = 0usize;
    for primitive in &mesh.primitives {
        if let std::collections::btree_map::Entry::Vacant(entry) =
            textures.entry(primitive.material)
        {
            let image =
                tm3::decode(bank.open_image(&mut bank_source, usize::from(primitive.material))?)?;
            bytes += image.pixels.len();
            if bytes > 256 * 1024 * 1024 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "layout exceeds aggregate texture budget",
                ));
            }
            entry.insert(image);
        }
    }
    Ok((
        MeshView {
            mesh,
            textures,
            layout: true,
        },
        summary,
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn word(b: &mut [u8], at: usize, value: u32) {
        b[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn dat(parts: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
        let mut b = vec![0; (4 + parts.len() * 8).next_multiple_of(16)];
        word(&mut b, 0, parts.len() as u32);
        for (i, (tag, bytes)) in parts.iter().enumerate() {
            let start = b.len() as u32;
            word(&mut b, 4 + i * 4, start);
            let tag_at = 4 + parts.len() * 4 + i * 4;
            b[tag_at..tag_at + 4].copy_from_slice(tag);
            b.extend(bytes);
            b.resize(b.len().next_multiple_of(16), 0);
        }
        b
    }

    fn fixture(ambiguous: bool, material: u16) -> Vec<u8> {
        // Original tiny triangle referenced by two differently translated nodes.
        let mut mdb = vec![0; 144];
        mdb[..4].copy_from_slice(b"mdb\0");
        word(&mut mdb, 4, 48);
        mdb[8] = 1;
        mdb[10] = 1;
        word(&mut mdb, 32, 64);
        word(&mut mdb, 64, 32);
        word(&mut mdb, 72, 56);
        mdb[84] = 3;
        mdb[86..88].copy_from_slice(&material.to_le_bytes());
        for (i, point) in [[0i16, 0, 0], [100, 0, 0], [0, 100, 0]].iter().enumerate() {
            for (axis, value) in point.iter().enumerate() {
                let at = 96 + i * 8 + axis * 2;
                mdb[at..at + 2].copy_from_slice(&value.to_le_bytes());
            }
            let flag: u16 = if i == 2 { 0 } else { 32768 };
            mdb[102 + i * 8..104 + i * 8].copy_from_slice(&flag.to_le_bytes());
        }
        let mut scr = vec![0; 256];
        scr[..4].copy_from_slice(b"scr\0");
        word(&mut scr, 8, 2);
        for (i, at) in [32usize, 144].into_iter().enumerate() {
            word(&mut scr, 16 + i * 4, at as u32);
            word(&mut scr, at, (256 - at) as u32);
            for axis in 0..3 {
                word(&mut scr, at + 16 + axis * 4, 1f32.to_bits());
            }
            word(&mut scr, at + 40, (10f32 * (i + 1) as f32).to_bits());
            if ambiguous && i == 1 {
                word(&mut scr, at + 28, 0.2f32.to_bits());
                word(&mut scr, at + 32, 0.3f32.to_bits());
            }
        }
        scr.extend(mdb);
        // Original 2x1 indexed TIM3 with a single synthetic palette.
        let mut image = vec![0; 129];
        image[..4].copy_from_slice(b"TIM3");
        image[4] = 4;
        image[6] = 1;
        word(&mut image, 16, 113);
        word(&mut image, 20, 64);
        word(&mut image, 24, 1);
        image[28] = 48;
        image[30] = 16;
        image[33] = 1;
        image[34] = 3;
        image[35] = 4;
        image[36] = 2;
        image[38] = 1;
        image[65..69].copy_from_slice(&[10, 20, 30, 128]);
        let mut bank = vec![0; 32];
        bank[..4].copy_from_slice(b"TM3\0");
        word(&mut bank, 4, 1);
        word(&mut bank, 16, 32);
        bank.extend(image);
        dat(&[(*b"SCP\0", dat(&[(*b"SCR\0", scr), (*b"TM3\0", bank)]))])
    }

    #[test]
    fn synthetic_layout_places_both_nodes_and_shares_one_texture() {
        let (view, count) = load_layout(&mut Cursor::new(fixture(false, 0)), None, &[]).unwrap();
        assert!(view.layout);
        assert_eq!(
            (
                count.collections,
                count.nodes,
                count.buffers,
                count.vertices,
                count.indices
            ),
            (1, 2, 2, 6, 6)
        );
        assert_eq!(view.textures.len(), 1);
        for (i, p) in view.mesh.primitives.iter().enumerate() {
            assert_eq!(p.positions[0], [10. * (i + 1) as f32, 0., 0.]);
            assert_eq!(p.positions[1], [10. * (i + 1) as f32 + 1., 0., 0.]);
            assert_eq!(p.indices, [0, 1, 2]);
            assert_eq!(p.uvs[0], [0., 1.]);
            assert_eq!(p.material, 0);
        }
        let isolated = load_mesh(&mut Cursor::new(fixture(false, 0)), 0, 1).unwrap();
        assert!(!isolated.layout);
        assert_eq!(isolated.mesh.primitives[0].positions[0], [0.; 3]);
    }

    #[test]
    fn ambiguous_node_fails_whole_layout_with_context_until_order_explicit() {
        let error = load_layout(&mut Cursor::new(fixture(true, 0)), None, &[])
            .err()
            .unwrap();
        assert!(error.to_string().contains("collection 0, node 1"));
        assert!(error.to_string().contains("ambiguous"));
        assert!(
            load_layout(
                &mut Cursor::new(fixture(true, 0)),
                Some(EulerOrder::Zyx),
                &[]
            )
            .is_ok()
        );
    }

    #[test]
    fn missing_material_and_aggregate_limits_fail_without_partial_view() {
        assert!(load_layout(&mut Cursor::new(fixture(false, 1)), None, &[]).is_err());
        for mut summary in [
            LayoutSummary {
                nodes: 4096,
                ..Default::default()
            },
            LayoutSummary {
                buffers: 8193,
                ..Default::default()
            },
            LayoutSummary {
                vertices: 1_048_577,
                ..Default::default()
            },
            LayoutSummary {
                indices: 3_145_729,
                ..Default::default()
            },
        ] {
            assert!(summary.add(&StaticMesh { primitives: vec![] }).is_err());
        }
    }

    #[test]
    fn explicit_exclusions_reject_wrong_tag_outside_ordinal_and_empty_selection() {
        for excluded in [&[1usize][..], &[99][..], &[0][..]] {
            assert!(load_layout(&mut Cursor::new(fixture(false, 0)), None, excluded).is_err());
        }
    }
}

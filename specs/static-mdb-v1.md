# Static MDB mesh and texture-bank binding, version 1

Date: 2026-11-03. Source: owned retail data probes, corroborated by independent
[Chihuahua MDB research](https://github.com/anasrar/chihuahua/blob/d27b348818163342f8c1f10841415b4e5240dbc1/pkg/mdb/main.go).
No original game instructions or translated game code consulted. Independent
tool source was read for format semantics; Rust is authored from this contract,
not a translation of a game function. MIT research attribution is in NOTICE.md.

## Supported static boundary

Each SCR MDB window ends at the next distinct MDB start, node-record start or
SCR end. Require the matching source length before opening that window. Never
let a stream consume another MDB or node metadata merely because it fits in SCR.
Within this window, accept lowercase MDB signature, flag zero at byte 30, and
one zero-translation bone. The u32 bone offset and u16 bone/buffer counts start
at byte 4. At byte 32, u32 buffer offsets are relative to MDB origin. Buffer count
is capped at 4,096; total decoded vertices at 1,048,576. Reject other mesh profiles.

Each 24-byte buffer descriptor has five relative u32 stream offsets (positions,
normals, UVs, colors, weights), then u16 vertex count and material ordinal. Required
position/UV regions start after the descriptor and fit the MDB. Check each optional
region when present. Reject weighted profiles and overlapping descriptor/data
regions; shared exact stream ranges are permitted only with the same encoding.

Static positions are signed i16 XYZ followed by u16 triangle emission flags. The
candidate scale is 0.01 world units per stored unit, pending runtime comparison.
Flag 32768 emits no face; flag 0 emits the previous two vertices plus the current
vertex; flag 1 reverses the first two. Reject unknown flags or emission before
vertex index 2. Preserve degenerate faces instead of silently changing topology.
UVs are signed i16 pairs divided by 4096; the candidate V convention adds 1.
Preserve out-of-range UVs and use repeat sampling in the inspection viewer.
Material is a zero-based ordinal in the selected sibling TM3 bank, checked before
rendering. No inferred name matching or substituted texture is allowed.

All 433 measured candidates have one zero-translation bone and flag zero. The
three samples contain 42,482 / 52,331 / 7,757 vertices, with only the three flag
values above; all material ordinals fit their 69 / 67 / 22-entry banks. A static
inspection view may omit lighting/vertex-color interpretation but must label that
limitation. SCR transform composition, animated meshes, runtime scene placement,
GS blending and in-game scale remain separate fidelity gates.

## Acceptance

Synthetic signed coordinates, both winding flags, non-emitting vertices, UVs,
material ordinals and malformed headers/limits/streams. Boundaries must stop a
mesh reading the next mesh's bytes. Real-data counts must match private probes.
Display an actual mesh and its referenced texture; retain the capture privately.

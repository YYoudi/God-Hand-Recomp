# First ISO-backed static mesh

Date: 2026-11-03. A small well-like prop from the retail data now renders in Bevy
with its referenced textures. This is an isolated asset inspection, not a recreated
game scene. The original ISO remains required and no game asset is published.

## Data path and measured scope

The original Rust implementation follows ISO -> FST -> AFS -> DAT/SCP -> SCR ->
bounded MDB, then binds each primitive's material ordinal to the sibling TM3 bank
and decodes that TIM3 image. Every MDB window ends before the next mesh or node
record. A stream merely fitting in its SCR parent is no longer sufficient.

| Retail candidate | Meshes | Buffers | Vertices | Emitted triangles | Decoded bank images |
| --- | ---: | ---: | ---: | ---: | ---: |
| r100 | 156 | 353 | 42,482 | 23,686 | 69 |
| r101 | 232 | 732 | 52,331 | 27,843 | 67 |
| r103 | 45 | 70 | 7,757 | 4,541 | 22 |

Rust counts match private data probes; all referenced material ordinals fit their
bank. All measured MDB headers use the static flag-zero profile with one
zero-translation bone. Their positions use signed 16-bit triples and explicit
triangle-emission/winding flags. The public generic pattern's float-position
layout is therefore insufficient for this static profile. Unsupported animation,
weighted profiles, unexpected flags and malformed ranges are refused.

The 158 measured textures use indexed TIM3. Selector zero has linear indices;
selector six uses GS transfer storage. The original direct address mapper handles
the observed 4-bit and 8-bit cases without a simulated 4 MiB VRAM buffer. Table-
based independent reference calculations corroborate complete synthetic images
at four size/type combinations, plus fixed coordinate cases and bijection checks.
No game image is used as a CI fixture.

## Actual visual check

The selected candidate is r100, SCP ordinal 23, SCR node 0. Its geometry and name
suggest a well assembly. Five buffers use five actual referenced textures. Bevy
displayed the masonry, wooden frame and attached wooden structure, and the
60-second smoke reached its deadline and exited normally. The private screenshot
records the actual window. An earlier arbitrary node rendered an ambiguous gray
shape; that view was not treated as proof of a recognizable gameplay object.

The inspection camera automatically frames local mesh bounds. It does not apply
SCR scene transforms. Materials are unlit, nearest-filtered with repeat UVs,
alpha blending and two-sided diagnostic rendering. Normals are computed for the
host mesh interface; original lighting, vertex colors, GS alpha tests and culling
are not reconstructed. Texture orientation, scale and appearance still require
comparison against a repeatable runtime scenario. A coherent isolated image does
not prove matching in-game rendering.

## Provenance and checks

Format semantics were informed by independent MIT
[Chihuahua research](https://github.com/anasrar/chihuahua/tree/d27b348818163342f8c1f10841415b4e5240dbc1/pkg),
including its static-mesh reader and table-based GS mapper. The new Rust decoder
is original contract-driven code; no game executable instructions, decompiled
functions or generated game C++ were used. See NOTICE.md and the versioned
[MDB](../specs/static-mdb-v1.md) / [TM3-TIM3](../specs/tm3-tim3-v1.md) contracts.
This records actual exposure; it does not claim formally separated observers.

38 synthetic Rust tests cover the existing readers and the new mesh/bank bounds,
winding, signed coordinates, UVs, unsupported profiles, indexed palette paths and
differential GS maps. The native linked viewer was checked separately from
headless parsing. Full inventory, probe source/results and actual mesh screenshots
stay under ignored local research directories.

## Next fidelity evidence

Associate the candidate room with a live retail reference scenario, inspect the
same landmark and verify transform scale/orientation, UV orientation and visible
materials. Establish SCR rotation composition before placing all nodes into a
room. Verify the prototype tools separately, without transferring prototype
mechanics or assuming that shared paths imply matching data. Full game logic,
collision, animation and combat remain unimplemented.

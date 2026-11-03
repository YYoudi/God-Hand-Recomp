# SCR node metadata, version 1

Date: 2026-11-03. Status: metadata reader validated on three retail candidates.
Provenance: public independent
[Chihuahua SCR research](https://github.com/anasrar/chihuahua/blob/d27b348818163342f8c1f10841415b4e5240dbc1/pkg/scr/scr.hexpat)
and private read-only data observations. Original Rust and synthetic fixtures;
no game executable instructions, decompilation or translated game source consulted.

## Contract and limits

The little-endian header has a lowercase SCR signature with zero terminator,
an opaque u32 word, a u32 node count, then four opaque bytes. A signed i32 offset
per node follows; each is relative to the SCR origin. Accept at most 4,096 nodes,
and require each complete 112-byte node record after the offset table and within
the selected SCR span. Reject overlapping node records. Preserve order and names
as eight raw bytes; names are not filesystem paths.

At a node's beginning, a signed i32 displacement selects an MDB signature relative
to that node's origin. Negative displacements are supported and occur in retail.
The target must lie outside the SCR header/table, within the SCR span and outside
all node records, with four bytes available for the lowercase MDB signature with
zero terminator. This proves a reference boundary only, not validity of a mesh.

After four opaque node bytes and eight name bytes, nine little-endian f32 values
represent candidate scale, Euler rotation and translation triples in that order.
Reject NaN/infinity. Preserve values without changing axes, degrees/radians,
rotation order or scale. Transform composition is a separate contract: public
field naming alone does not establish the game's camera or coordinate pipeline.
Remaining record bytes are opaque. Bound all reads; do not follow arbitrary
unvalidated pointers or interpret these data as executable code.

## Evidence and acceptance

Python inspection found 156 nodes in candidate r100, 232 in r101 and 45 in r103;
all referenced signatures matched MDB, with finite candidate transforms. Paths
are candidate identifiers, not confirmed in-game room names. Rust must reproduce
these counts. No geometry/topology/material/texture decoding or fidelity claim
follows from this check. Use synthetic tests for signed backward references,
bad offsets/signatures, overlapping nodes, truncation, count limits and nonfinite
transforms. Actual names, transforms and game bytes stay private.

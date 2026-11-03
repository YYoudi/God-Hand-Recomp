# Room storage and node metadata

Date: 2026-11-03. This increment establishes a checked room-data access boundary.
It does not reconstruct a room or verify which candidate is the starting room.

## Findings

All 94 retail stage-prefixed DAT files contain a count, an offset table and a
parallel tag table. A read-only probe falsified the first size-table hypothesis:
the second array is a sequence of four-byte identifiers, not section lengths.
Storage spans must be derived from adjacent offsets and the container end.
They include alignment padding, repeated identifiers and empty sections.

The outer offsets are aligned to 32 bytes. Their SCP children use 16-byte
alignment; enforcing the outer alignment on these children would reject 80
otherwise bounded catalogs. Of 94 SCP slots, 93 have a nonempty catalog and one
is a zero-count placeholder. Those children contain 1,708 storage sections.

Independent public [Chihuahua format research](https://github.com/anasrar/chihuahua/tree/d27b348818163342f8c1f10841415b4e5240dbc1/pkg)
corroborates the table layout and identifies the SCP/SCR/MDB and TM3/TIM3 resource
families. It is MIT-licensed tooling by Anas Rin, not a matching game decompilation.
The pinned patterns were read as format references after the initial table probes.
No parser source is vendored and no original game instructions were consulted.

The Rust reader follows this measured path without extraction:

```text
Owned retail ISO -> FST path -> bounded AFS member
  -> outer DAT catalog -> bounded SCP catalog
      -> SCR collection -> node record -> checked MDB signature reference
      -> TM3 candidate (texture-container decoding remains next)
```

## Verification

| Candidate | Outer sections | SCP sections | SCR collections | Validated nodes |
| --- | ---: | ---: | ---: | ---: |
| r100 | 40 | 29 | 28 | 156 |
| r101 | 23 | 47 | 46 | 232 |
| r103 | 38 | 6 | 5 | 45 |

Python and Rust agree on these counts. Each node's signed relative reference
resolves to an MDB signature inside its bounded SCR source. References commonly
point backward, so treating them as unsigned would produce false corruption.
Node records and reference targets are checked for range/overlap. Candidate
transform floats are finite and retained without unit, axis or rotation changes.
Names are opaque eight-byte metadata, not output paths.

Seven new synthetic Rust tests cover invalid tables, limits, alignment, descending
offsets, nested member isolation, counted metadata-only reads, signed references,
invalid signatures, overlapping nodes, truncation and nonfinite transforms.
There are now 29 Rust tests in the workspace. Private real-data checks inspected
all outer/nested catalogs and the three node samples above. No payload bytes,
names, transforms or game images are distributed by these tests or this report.

## What this does not establish

An MDB signature is not a valid mesh. Vertex buffers, topology, UVs, materials,
texture indices and transform composition remain to be specified and checked.
The public structural field labels are useful hypotheses, not proof of degrees,
radians, Euler order or scene behavior. Geometry has not been displayed by the
independent engine in this increment. Runtime room identity, palette/blending
fidelity and prototype debug controls remain unverified. These observations
apply to retail; this parser has not yet been validated on prototype room data.

## Next experiment

Within one of the checked SCR candidates, bound an MDB's header, descriptor
tables and vertex streams; cross-check counts and ranges on several nodes.
Establish triangle-strip/flag interpretation before drawing topology. Separately
validate the TM3 catalog and its referenced TIM3 textures, preserving material
ordinals. Then select one static element whose mesh, texture and transform can
be compared against an owned reference scenario. Keep raw captures private.

Local evidence is under recomp/experiments/room-2026-11-03/; exact paths and
commands are recorded in docs/local/overnight-checkpoint.md. These are ordinary
local probes and Rust checks, not canonical REA Evidence.

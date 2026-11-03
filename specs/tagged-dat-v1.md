# Tagged DAT section catalog, version 1

Date: 2026-11-03. Status: validated metadata boundary on owned retail samples.
Provenance: original read-only probes, then corroboration by
[Chihuahua's DAT pattern](https://github.com/anasrar/chihuahua/blob/d27b348818163342f8c1f10841415b4e5240dbc1/pkg/dat/dat.hexpat).
Chihuahua is independent MIT tooling by Anas Rin; no game executable or translated
game function was consulted. No third-party parser implementation is copied.

## Boundary

A little-endian u32 count is followed by count u32 offsets and count opaque
four-byte tags. Offsets are relative to this container's start. This profile
requires 16-byte alignment, nondecreasing offsets at or after the entire table,
and offsets no greater than source length. Each section spans to the next offset;
the final section spans to the source end. These are storage spans and may include
padding; they are not a claim of exact logical payload length. Equal offsets and
zero-sized sections are valid. Tags may repeat and retain their ordinal identity.
Never use tags as output filenames or execute their payloads.

There is no magic signature. Only invoke this reader for an explicitly selected
candidate context, not as a universal content detector. Zero count is accepted
as an empty catalog. Limit count to 65,536 before allocation. Length, additions
and relative windows use widened arithmetic. Read header/tables only; section
access is a bounded, read-only seekable view. Reject a changed backing length,
truncation, excessive count, table overlap, descending/unaligned/outside offsets
and invalid ordinals. No recursive automatic parsing or whole-container extraction.

## Measured samples and uncertainty

All 94 retail stage-prefixed DAT files satisfy this table layout. Their offsets
are also aligned to 32 bytes. Of these, 93 nonempty SCP sections contain a second
catalog with this layout, at 16-byte alignment. Requiring 32 bytes for inner
sections incorrectly rejects 80 samples; the first hypothesis that the second
array held sizes was also falsified. It contains tags. One SCP placeholder has
no nonempty catalog; it must not be fabricated into scene data.

Public research identifies SCP as a resource container with TM3 texture and SCR
node collections. Three owned retail room candidates were inspected further;
their nested SCR nodes refer to matching MDB signatures. In-game room selection,
geometry, texture association and runtime scene semantics remain to be verified.
Prototype applicability is not established by this retail-only experiment.

## Acceptance

Synthetic repeated tags, equal offsets, empty catalogs/sections and nested windows;
malformed size/count/offset/ordinal/backing-length cases. A counted input must prove
catalog reads do not consume payload bytes. Inspect all 94 retail catalogs and
their nonempty SCP children through the Rust ISO/FST/AFS path; retain full metadata
locally. No game data in tests or CI.

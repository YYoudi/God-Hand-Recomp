# D03 / G1.3 — bounded AFS catalog, version 1

Date: 2026-11-03. Scope: original generic format reader, not game logic.
Provenance: the format section of the author's
[cri-afs documentation](https://github.com/soyjxck/cri-afs#format) describes an
AFS signature, entry count and offset/size records. [AFSLib](https://github.com/MaikelChan/AFSLib)
documents variant/edge-case concerns. No implementation from either library is copied.
No original game executable or translated game C++ was consulted for this contract.

## Supported contract

An archive is a bounded seekable byte stream. At byte zero, require AFS followed
by a zero byte. Read a little-endian u32 count, then that many little-endian
u32 offset/size pairs. Offsets are relative to this archive, never to the ISO.
Little-endian is the supported subset, not a claim about every AFS variant.
Names, optional attribute tables, alignment, decompression and payload semantics
are outside version 1. An entry index is not an asset-type or room identifier.

## Safety and resource rules

- Reject more than 65,536 entries before allocating the catalog.
- Check that the full table fits the stream before reading its records.
- Check offset + size in widened arithmetic against the archive length.
- A nonempty payload cannot begin inside the header/table.
- Permit zero-length placeholders and aliased extents if in bounds.
- Do not require sorting or fixed alignment; do not infer filenames.
- Read header/table only: allocation depends on count, not archive length.
- The enclosing ISO file view rejects seeking outside its extent and caps each read.
  Reject multi-extent ISO files rather than treating them as contiguous storage.

## Acceptance

Synthetic tests cover a valid catalog, wrong signature, truncated header/table,
oversized count, out-of-range extent, payload/table overlap, zero/aliased entries
and attempted reads/seeks beyond an ISO extent. A counting source verifies that
catalog parsing does not read payload bytes. The owned-disc smoke reports aggregate
catalog size/count only and never extracts the archive or executes game code.

No claim is made about GODHAND.FST or prototype/retail layout equivalence. Unsupported
variants must fail with a useful error and lead to a separately reviewed contract
revision, not a silent heuristic copied from the game.

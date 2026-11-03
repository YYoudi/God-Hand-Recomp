# FST namespace and AFS lookup, version 1

Date: 2026-11-03. Status: experimentally validated for the two local profiles.
Author: current investigation session. No game executable, disassembly, translated
code or decompiler output was consulted. This is data-format research, not proof
of a formally separated clean room. Private evidence: structure-2026-11-03.

## Contract

The little-endian header is 16 bytes: FST signature with zero terminator, version
word representing 1.0, string-pool offset, record-table offset. This reader accepts
pool offset 16. The remaining table consists of 12-byte records (three u32 words).
Names are zero-terminated ASCII components at offsets relative to the string pool.
Directory child pointers are byte offsets relative to the record table. Root starts
at table offset zero; each directory listing ends at a (3, 0, 0) record.

Low byte 1 means directory (remaining flag bits zero), low byte 2 means file. File
word three is a zero-based AFS ordinal. Upper file flag bits remain opaque: they
must not substitute for the size from AFS. A zero directory pointer denotes an
empty directory in the prototype; it must never recurse to root. Other child
pointers must be aligned, in bounds and uniquely visited. A final all-zero record
is permitted as padding. Reject other unreachable records, cycles, aliases,
unterminated listings, unsafe names and duplicate full paths. An empty listing
is valid. Limits: 16 MiB FST, 65,536 records, depth 64, component length 255.
Expanded full-path text is also capped at 16 MiB.

Binding to AFS requires every ordinal exactly once, matching file/entry counts.
Reject mismatched, duplicated or missing ordinals. No executable payload is run.
All source windows remain read-only and bounded by their AFS entry.

## Evidence and uncertainty

Retail: 1,445 files, 49 directories; all AFS ordinals covered once. Prototype:
1,078 files, 62 directories; empty-directory pointers observed; all ordinals
covered once. The two profiles are distinct; path equality does not imply identical
content or behavior. The format's opaque flags and runtime loading semantics
remain unknown. Directory names and extensions are routing hints, not proof of
scene or gameplay semantics. REA 6.7.0 rejected FST as unsupported; observations
come from a separate read-only Python probe, not REA Evidence.

## Acceptance

Synthetic nested/empty directories, ordinal lookup, opaque flags; reject truncation,
bad header, invalid pointers, cycles/shared listings, malformed names, duplicate
paths, unreachable records and bad archive binding. Run the Rust reader over the
owned retail image; keep full file listings and prototype comparison local.

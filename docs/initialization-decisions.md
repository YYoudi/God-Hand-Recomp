# Initialization decisions

Append material evidence-driven changes here. Keep private dumps and raw logs out.

## 2026-11-03 — start with a bounded Rust foundation

- Evidence: source checks passed previously, but no Rust viewer had been linked and
  observed. The native backend's first fight does not validate Bevy execution.
- Decision: G1 requires a real synthetic-sandbox smoke test and safe catalog access.
- Next evidence: link/run the viewer, then specify bounded archive access.
- Limit: no game logic or game assets are implemented by this decision.

## 2026-11-03 — preserve the prototype CUE

- Evidence: the original CUE names a June 22 BIN, while the available file is labeled
  June 21. Its header matches ISO 9660 at 2048-byte sectors, matching the CUE track mode.
- Decision: generate a separate local CUE pointing at the existing BIN; preserve
  both originals. The inventory tool refuses unknown track layouts.
- Next evidence: boot the prototype in a reference emulator and verify debug controls.
- Limit: container validity is not runtime or debug-menu validation.

## 2026-11-03 — reduce compiler pressure after a native build failure

- Evidence: rustc exited with STATUS_ACCESS_VIOLATION while compiling ash in the
  first linked Windows GNU viewer build. Compilation used four jobs and debug info.
- Experiment: retry with one job and CARGO_PROFILE_DEV_DEBUG=0, retaining the log.
- Outcome: the retry linked the viewer in 4 minutes 23 seconds. The helper now uses
  one job and debug-info level 0 by default, with explicit overrides.
- Limit: the underlying cause is not established; do not diagnose hardware from this.

## 2026-11-03 — validate rendering rather than trusting exit code

- Evidence: the first native sandbox showed a magenta fallback and reported missing
  tonemapping LUTs despite returning success at its deadline.
- Decision: use explicit Tonemapping::None for the original synthetic scene.
- Outcome: the corrected scene visibly rendered a gray floor and orange capsule;
  the 45-second smoke reached its deadline and exited normally on Vulkan/RTX 5080.
- Limit: this validates our primitives, not game rendering or gameplay.

## 2026-11-03 — index the archive without extracting it

- Evidence: the original bounded catalog contract and synthetic malformed cases passed.
  Retail inspection found 1,445 AFS entries in a 1,541,885,952-byte archive.
- Decision: expose checked ISO file views and parse only the AFS header/table.
- Outcome: catalog access reads metadata without whole-archive allocation or extraction.
- Limit: no filenames, FST mapping, payload decoding or prototype/retail equivalence established.

## 2026-11-03 — follow real asset data rather than expand a placeholder scene

- Evidence: retail FST records cover all 1,445 AFS ordinals; prototype covers 1,078
  and introduces zero-pointer empty directories. REA refuses FST as unsupported.
- Decision: specify an original bounded namespace reader, then a narrow TIM2
  decoder from public format research and local header observations. No game
  executable, translated C++ or decompiler output is implementation input.
- Outcome: both profile catalogs validate in Rust; 34 of 35 retail top-level TIM2
  textures decode. A 4-bit portrait and an 8-bit interface-like image display in
  a separate ISO-backed Bevy inspection view. Full captures stay local.
- Adjustment: the interface-like texture appears to be a retained mockup. Count
  this as format validation, not an active retail screen or faithful menu recreation.
- Next evidence: bounded room-container sections, a measured static mesh and its
  texture association; reference-runtime comparisons before gameplay.
- Limit: one multi-palette font remains unsupported. Scene semantics, GS-rendering
  parity and prototype debug access remain unverified. Parallel agents wait for
  explicit contracts grounded by these experiments.

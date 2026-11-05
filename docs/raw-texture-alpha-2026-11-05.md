# Preserve texture alpha before converting it for display

Date: 5 November 2026. Status: raw-data prerequisite implemented and checked;
**no production retail renderer change or new fidelity claim**.

## Why this matters

The existing viewer decoder maps palette alpha0..128 into presentation0..255,
clamping higher values. That conversion is useful for inspection transparency,
but it is lossy and is not valid raw input to byte-alpha tests or blend arithmetic.
Reversing it cannot recover the discarded values. RGB expansion, palette index
ordering and transfer addressing do not need a new interpretation for this step.

The [contract](../specs/raw-texture-alpha-v1.md) was written before implementation.
Original TIM2/TIM3 `decode_raw` APIs now return a distinct `RawRgbaImage`, copying
all four palette bytes unchanged. `into_presentation` consumes that image using
the existing conversion; `decode` and `RgbaImage` retain their historical meaning.
The ordered GPU adapter accepts the raw type explicitly, without conversion.
No game program, translated function or game shader was implementation input.

## Bounded retail inspection

A new no-GPU diagnostic reads the user's ISO through validated FST/AFS ordinals,
exact room paths and unique DAT/SCP/TM3 sections. It compares every expanded raw
RGB byte and the expected legacy alpha-conversion relation. Existing per-image
limits apply; the command additionally caps16 paths,512 bank images,64Mi decoded
texels and120 seconds. It emits no success report until all selected rooms pass,
never extracts assets, and assigns no runtime state from alpha distributions.

The three established retail candidates passed: **158 bank images,1,894,912
texels**. The original ISO SHA-256 is unchanged. All raw RGB and normalized-alpha
relations match; existing normalized synthetic fixture outputs remain exact.
This is not a separate decoder oracle or a rendered baseline comparison: both
APIs deliberately share the unchanged index/address expansion.

Of these texels,19,980 have a different **unmodulated >127 predicate** when
presentation alpha is mistakenly treated as raw. Another3,172 contain original
alpha above128, which must remain representable. These are aggregate storage
observations, not numbers of incorrect screen pixels, recovered draw states,
texture-binding proofs or evidence that every resident resource is active.
Modulation, draw assignment and runtime passes must still be established.
Raw per-image logs and hashes remain private; no game texels become fixtures.

## Synthetic and actual GPU checks

- 72 distinct Rust tests (66 base plus6 viewer-only),25 Python tests, formatting,
locked viewer check and linked build pass.
- Original fixtures cover all256 alpha values, CSM1 ordering,4/8-bit nibbles,
linear TIM2/TIM3, both selector6 indexed widths and malformed/truncated inputs.
- The opt-in GPU checker now starts its exhaustive ramp at an original encoded
TIM2/CSM1 fixture, then passes the raw typed result to the shared GPU adapter.
Actual RTX5080 / Vulkan result:4,718,592 exhaustive alpha-stage pixel cases plus
573,442 spatial samples pass exact RGB/stored-alpha/depth checks.
- Inspector tests check bins, the conversion's threshold difference, malformed
images, RGB/alpha mismatches and missing/ambiguous section tags. Duplicate room
arguments are also refused before opening a disc.

CI uses original synthetic fixtures only, compiles the binaries and runs CPU
unit tests; GPU conformance and personal-ISO diagnostics are opt-in local actions.
No new GPU service, artifact upload, cache or paid Pages feature is introduced.

```sh
cargo run -p godhand-lab --bin godhand-alpha-inspect --locked -- \
  --iso "/path/to/personal.iso" --room st1/r100.dat \
  --room st1/r101.dat --room st1/r103.dat
cargo run -p godhand-lab --features viewer --bin godhand-alpha-check --locked
```

Windows validation uses the installed GNU toolchain, one job and no dev debug
info as in `scripts/dev.ps1`. The normal `godhand-lab` CLI remains the default.

## Next falsifiable step

Specify byte-domain blending and evidence-supported per-draw state, integrate
raw alpha and ordered writes as an explicit experiment, then compare both fixed
full-resource viewpoints. The production viewer still uses its unchanged
presentation path. Do not switch defaults, infer materials from opacity alone,
or claim sky activation from successful storage/CPU/component tests.

No new Reference / Bevy pair is fabricated for this reader increment. Existing
original comparison images are unchanged. Full rendering, general transforms,
camera motion and gameplay remain incomplete; prototype reference stays separate.
The autonomous goal is active, not complete.

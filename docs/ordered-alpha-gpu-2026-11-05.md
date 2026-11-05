# Ordered alpha writes: an offscreen GPU component

Date: 5 November 2026. Development resumed explicitly after the repository rename.
Status: synthetic component validated; **production retail viewer unchanged**.

## What changed

The rejected all-opaque control showed why one opaque/transparent flag is not
enough. The original [CPU permission model](../specs/fragment-write-permissions-v1.md)
now drives a reusable Rust/WGSL component in the app library. Its
[contract](../specs/ordered-alpha-gpu-v1.md) was written before implementation.

Each caller-ordered triangle has complementary passing/failing shader branches.
CPU-derived pipeline masks independently permit RGB, stored framebuffer alpha
and depth writes. The RGB target's presentation alpha remains opaque; stored
alpha lives in a separate data target. A zero data-alpha byte therefore cannot
hide valid RGB when presenting the result. Depth uses GreaterEqual, clear zero.
This component deliberately **overwrites RGB without blending** to isolate write
permissions. It bypasses material-phase sorting; it does not infer game draw state.

The shared implementation is `crates/app/src/alpha_gpu.rs`, with the original
`alpha_stage.wgsl`. The opt-in `godhand-alpha-check` executable calls that same
library, not a duplicated conformance shader. It creates no window and reads no
disc, game texture, game code or historical screenshot.

## Actual validation

- Locked build, formatting, workspace tests, viewer check and feature tests pass.
  67 distinct Rust tests:61 base plus6 viewer-only.25 Python tests pass.
- Actual local GPU: NVIDIA GeForce RTX 5080, Vulkan, Bevy0.20.1 / wgpu30.0.1.
- Full mode: **4,718,592 pixel cases**, covering every byte-alpha/reference pair,
  eight comparisons, four failure actions, both depth-write masks and disabled
  controls. Every RGB byte, stored alpha and depth value matches the CPU oracle.
- **573,442 additional spatial samples** check behind/front occlusion, equal-depth
  writes, caller order including color-only updates and overlapping mixed-outcome
  primitives within a single draw, byte-domain modulation and
  saturation, screen-linear color interpolation with unequal clip W, adjacent
  floating-point values at the127/128 boundary, and clipped/straddling geometry.
- Quick mode also passed; it is a selected-reference regression, not exhaustive
  alpha/reference coverage. No frame-rate or game-fidelity result is inferred.

Finite zero/negative clip W is delegated to GPU clipping, not rejected wholesale.
Input budgets and malformed/nonfinite buffers are checked before submission;
readback mapping has finite timeouts. Private receipts retain source/executable,
lockfile and log hashes. No ISO or extracted material was used in this increment.
CI compiles the executable and runs CPU tests only; it never runs the GPU suite.
The existing free public runner and main:/docs Pages configuration are unchanged.

## Run without game data

```sh
cargo run -p godhand-lab --features viewer --bin godhand-alpha-check --locked
cargo run -p godhand-lab --features viewer --bin godhand-alpha-check --locked -- --quick
```

Windows local validation uses the installed GNU toolchain, one job and no dev
debug info, as in `scripts/dev.ps1`. The normal CLI remains the package default.

## A failed hypothesis retained

Review found that two branch-wide batches per draw can reorder overlapping
triangles with different alpha outcomes. A new synthetic regression failed
against that implementation. The corrected adapter submits both branches for
each triangle before advancing to the next primitive; both primitive orders
then pass. This is a correctness-first diagnostic path with up to two commands
per triangle, not an optimized renderer or a performance result.

## What this does not prove

This is **not** a GS renderer, a fix for retail sky overpainting, recovered scene
activation or gameplay. There is no blending, fog, lighting, destination-alpha
test, scissor or full framebuffer mask implementation. Raster precision,
clipping edge rules and complete GS arithmetic are not validated by these tests.
There is no new retail replay or screenshot pair: synthetic checks must not be
presented as a Reference / Bevy game comparison. Existing originals are preserved.

The current retail texture reader normalizes/clamps alpha for presentation.
That is **not raw input** to this component. Preserve and corroborate raw alpha,
specify byte-domain blending and explicit per-draw state, then integrate only as
an opt-in experiment and compare both fixed full-resource viewpoints. Do not
assign runtime materials from source opacity alone, silently change defaults,
or use manual collection exclusions as scene activation.

The autonomous goal remains active. General transforms, camera motion,
actors/HUD, full rendering and gameplay remain incomplete; prototype behavior
stays separate. Original project code is MIT; this is a provenance protocol,
not legal certification or a formal independent-team clean room.

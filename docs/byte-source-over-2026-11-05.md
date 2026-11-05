# Byte-source-over control at both fixed retail viewpoints

Date: 5 November 2026. Status: original component validated; actual all-resource
controls rendered. **Uniform states are not recovered game materials.**

## What was observed, specified and implemented

Existing data-only draw observations joined to geometry-consistent UV candidates
show the same enabled source-over selector combination on176 first-view and28
second-view buffers. Those counts overlap, are not complete coverage and do not
prove texture identities, activation or native draw order. Raw trace/program
labels and per-resource mappings remain private; no game instructions, generated
functions or game shaders were implementation inputs.

Independent [ps2tek hardware research](https://psi-rockin.github.io/ps2tek/) informs
the original [byte-input contract](../specs/byte-source-over-v1.md): source alpha
uses128 as unity, signed negative differences round down, and RGB is clamped
after the weighted update. This is not normalized host alpha blending. The
bounded RGBA8 model does not establish wider source intermediates or full GS
precision, other blend equations, fog or complete framebuffer behavior.

Original Rust and WGSL implement that explicit subset. RGB is overwritten by
computed byte results, not a host blend state. Each blended triangle reads a
separate snapshot of the previous primitive's RGB; passing/failing branches use
independent RGB/stored-alpha/depth masks. Data alpha never becomes presentation
transparency. [WGSL shift semantics](https://www.w3.org/TR/WGSL/#shift-expr) and
actual signed-product GPU checks support the shader arithmetic.

## A negative control that failed

A deliberately stale whole-draw destination snapshot produces the wrong color
when two overlapping primitives in one draw have different alpha outcomes.
The synthetic GPU regression fails as expected; restoring a snapshot before
each primitive passes. Both branches for a primitive share one snapshot and
complementary coverage. No currently writable attachment is sampled.
This is a correctness-first feedback path, not a performance claim.

## Actual validation

- 78 distinct Rust tests:70 base plus8 viewer-only;25 Python tests, formatting,
locked viewer check and linked build pass. Existing normalized reader fixtures
remain exact; a new raw-layout test checks geometry, bindings and conversion.
- CPU exhausts16,777,216 source/destination/alpha byte triples against a separate
weighted-rational integer oracle.
- Actual RTX5080 / Vulkan:16,777,216 blend pixel cases plus163,840 feedback,
write-mask, ordering and culling samples pass RGB/stored-alpha/depth checks.
Previous4,718,592 alpha-stage cases and573,442 spatial samples still pass.
- The real-ISO control runs below use no window and complete normally; original
ISO SHA-256 is unchanged. Each run contains156 nodes,353 buffers,23,686 triangles
and67 referenced images. All source color streams are present: zero neutral
fallback buffers. Four captures cover Keep and framebuffer-only failures at both
unchanged512x448 poses, with4:3 projection and explicit back-face culling.

CI runs only original synthetic CPU tests and compiles the optional GPU tools.
Personal ISO data, traces and raw textures are never CI fixtures or artifacts.
The existing free public runner and main:/docs GitHub Pages service are unchanged.

## Actual original captures, including the failures

The selected pairs below use framebuffer-only alpha failures. Both Keep controls
are retained privately; they also exhibit ground holes. The original native
reference PPMs and renderer PNGs are unchanged; reference PNGs are labeled
lossless previews. No cropping, retouching, camera refit or object movement.

| Fixed viewpoint | Reference | Bevy uniform-state control |
| --- | --- | --- |
| First | [PNG preview](dashboard/media/edace008cb069c5c1a80cb803c1bf71aab90a7740ad57ee2cb3f91d3941cc9f5.png) / [unchanged PPM](dashboard/media/033e25087ed4fe723f77b608804cb4af404c25c6e8671f4a743cbf18ad355dad.ppm) | [Original renderer PNG](dashboard/media/b46740c4f8440a7c24823d583f039c2f5b76d2a97a707ffd3e2f4a86683bfd71.png) |
| Second | [PNG preview](dashboard/media/91c07de4b15b05b38f9cd1b1e455edd4c2cef7815aff7e28d10a0a7796e1f2af.png) / [unchanged PPM](dashboard/media/6b4e4957d85122eb5166a6d4e068a5a685314522ad03492008de9e229fd2a83e.ppm) | [Original renderer PNG](dashboard/media/f5cd907a90a07cf448e29163b78ff70991a6d9497ca723bce9fdf6f3bff01449.png) |

The street/buildings are visible with every resource included, unlike the earlier
all-blend inspector baseline. **Blue gaps crossing the ground remain obvious**;
background detail, surface colors/filtering and shading also differ. Neither
uniform failure mode is accepted as a final material policy. Actors/HUD and
runtime game logic are absent. The reference is the third-party native backend,
not hardware or a PCSX2 cross-check; prototype work stays separate.

### Before / After means named iterations, not recovered fidelity

Second viewpoint, same full resource set and unchanged camera:

- Before: [all-blend inspector, revision f83f076](dashboard/media/d14c9554c493b040c7c76454da8e7358ccf80a2c83f4af2efbe8160333378e39.png).
- After: [explicit byte-source-over control, uncommitted capture sources](dashboard/media/f5cd907a90a07cf448e29163b78ff70991a6d9497ca723bce9fdf6f3bff01449.png). The registry records that provenance; private receipts retain exact source/executable hashes.

This changes several factors together: raw alpha, byte-domain RGB feedback,
alpha write permissions and ordering policy. It does NOT isolate depth or prove
all resources should be active. The earlier first-view inspection also excluded
sky collections, so it is not advertised as an equivalent all-resource before.
All previous originals and rejected opaque controls remain preserved.

## Reproduce an explicit control, not a shipping preset

```sh
cargo run -p godhand-lab --features viewer --bin godhand-static-control --locked -- \
  --iso "/path/to/personal.iso" --room st1/r100.dat \
  --eye -4.4560604,1.3398599,35.0116218 \
  --target -2.3145737,0.6408461,25.2686544 --fov 31 \
  --aspect 1.3333333333 --size 512,448 --backface-culling \
  --blend source-over --alpha-failure framebuffer-only \
  --capture-png "/new/path/control.png"
```

State is uniform and caller-selected: GT127, unmasked GEQUAL depth,
nearest/clamp sampling, source colors and storage order. No scene exclusions
are available in this tool. Missing colors would use reported neutral128 for
this diagnostic only; undefined multiaxis placement is refused unless an order
is explicitly supplied. Output must be new. The normal viewer remains unchanged.
Windows uses the installed GNU toolchain, one job and no dev debug info as in
scripts/dev.ps1. The GPU checker still needs no disc or game images.

## Next falsifiable investigation

Compare data-only runtime vertex-alpha/texture/pass inputs against identifiable
source ground resources at the same two poses. Do not guess that the holes are
fixed by forcing alpha, changing culling, deleting resources or moving the camera.
Establish material/ordering coverage before replacing the uniform control.
Full GS source intermediates, filtering/fog, scene activation, general transforms,
camera motion, actors/HUD and gameplay remain incomplete. Goal active, not complete.

## Subsequent addressing experiment

[Explicit Repeat](texture-addressing-2026-11-05.md) restores texture detail while
keeping the same source-over controls. The original Clamp commands remain byte-
identical; the blue ground gaps persist. Neither addressing nor disabling culling
provides a complete hole/material fix. Original images above remain unchanged.

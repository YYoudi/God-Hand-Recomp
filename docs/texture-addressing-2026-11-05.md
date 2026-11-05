# Repeat sampling restores detail, not every ground gap

Date: 5 November 2026. Engine capture revision:
`bb7aedf2805822b550c5e84d0c116e02c348df99` (committed before the new captures).
Status: isolated addressing comparison, **not recovered per-draw materials**.

## Observed discrepancy and contract

The original MDB contract keeps signed UVs, adding one to V; the normal inspector
uses Repeat. The separate ordered control initially used Clamp. Adding an integer
UV period preserves Repeat sampling but not Clamp: an out-of-range coordinate can
select an edge texel rather than the intended interior texture detail.

Existing data-only GS state joins show wrapU=0/wrapV=0 on176 first-view and28
second-view candidate buffers. Independent [gsKit register constants](https://github.com/ps2dev/gsKit/blob/8ef73d05f022d14b3ded7c70cba670fe333ed61a/ee/gs/include/gsCore.h)
identify0 as Repeat and1 as Clamp. Only those enum meanings were consulted, not
a game implementation or rasterizer. Raw source digests/mappings remain private.
Candidate counts overlap; they are not full active-material or texture-identity
coverage. The [contract](../specs/texture-addressing-v1.md) was written before code.

## Original implementation and synthetic checks

Each explicit GPU draw now has independent Clamp/Repeat addressing on U/V, with
four bounded nearest samplers. Changing sampler binding does not change shader
math, texture bytes, RGB/alpha/depth permissions, culling or submission order.
`--address clamp/repeat` opts the static control into one mode on both axes;
omitting it preserves the prior Clamp default. The normal inspector stays Repeat.
Region/mirror modes are refused, not approximated.

80 distinct Rust tests (70 base plus10 viewer-only),25 Python tests, formatting,
locked viewer check and linked build pass. Actual RTX5080 / Vulkan validates
147,456 original addressing samples: an8x4 texture, negative/out-of-range UVs,
independent axes, exact integer boundaries, integer-shift equivalence under
Repeat and non-equivalence under Clamp, mixed sampler bindings/order and raw
transparent-border alpha/depth/source-over feedback. A forced-Clamp implementation
fails the new GPU regression as intended; correct source is restored and rebuilt.
All earlier alpha/blend suites still pass. CI compiles GPU tools and runs original
CPU fixtures only; no game pixels, ISO or raw runtime traces are fixtures/artifacts.

## Real retail controls, without fitting

Replay all four previous Clamp render argument sets with new output paths: every PNG remains
**byte-identical**, not merely visually similar. Then add only `--address repeat`
for Keep and framebuffer-only variants at both fixed poses. Every render still
contains156 nodes,353 buffers,23,686 triangles and67 images, without exclusions,
object edits, camera changes or missing-color fallbacks. The original ISO SHA-256
is unchanged. Selected original framebuffer-only controls follow below.

| Fixed viewpoint | Reference | New Bevy control |
| --- | --- | --- |
| First | [Reference preview](dashboard/media/edace008cb069c5c1a80cb803c1bf71aab90a7740ad57ee2cb3f91d3941cc9f5.png) / [unchanged original PPM](dashboard/media/033e25087ed4fe723f77b608804cb4af404c25c6e8671f4a743cbf18ad355dad.ppm) | [Original Repeat PNG](dashboard/media/467b8596967c777e11a8a2cb3ef3edf55614ce50ad1c4fd43655cdd3435755a0.png) |
| Second | [Reference preview](dashboard/media/91c07de4b15b05b38f9cd1b1e455edd4c2cef7815aff7e28d10a0a7796e1f2af.png) / [unchanged original PPM](dashboard/media/6b4e4957d85122eb5166a6d4e068a5a685314522ad03492008de9e229fd2a83e.ppm) | [Original Repeat PNG](dashboard/media/8f7e4953c9ceeb7f09ba0a4dbc7b8a8038aa6fb27692a95b97c673b63a74ebc8.png) |

Reference PPMs and new renderer PNGs are unchanged originals. The reference PNG
is a labeled lossless preview, not a replacement original. The reference is the
third-party native backend, not hardware or a PCSX2 cross-check. Prototype stays
separate. No desktop pixels, crop, repaint, image warp or post-capture correction.

## Named Clamp / Repeat iterations

The Clamp controls were captured from the preceding uncommitted engine sources,
then recorded in code revision `fb0f307`; their original registry provenance is
unchanged. They have now also been reproduced byte-identically from the committed
new engine. Repeat captures name exact revision `bb7aedf` above. Before / After
here means these named test iterations, not that Bevy is better than the game.

| View | Before: retained Clamp original | After: explicit Repeat original |
| --- | --- | --- |
| First | [Clamp control](dashboard/media/b46740c4f8440a7c24823d583f039c2f5b76d2a97a707ffd3e2f4a86683bfd71.png) | [Repeat control](dashboard/media/467b8596967c777e11a8a2cb3ef3edf55614ce50ad1c4fd43655cdd3435755a0.png) |
| Second | [Clamp control](dashboard/media/f5cd907a90a07cf448e29163b78ff70991a6d9497ca723bce9fdf6f3bff01449.png) | [Repeat control](dashboard/media/8f7e4953c9ceeb7f09ba0a4dbc7b8a8038aa6fb27692a95b97c673b63a74ebc8.png) |

Facade, terrain and mountain texture detail returns instead of stretched edge
samples. **The blue ground gaps remain in both views.** Therefore the hypothesis
that Clamp caused ALL holes is rejected. Repeat addresses one real input problem,
not complete ground coverage or overall fidelity. Sky detail/shading, filtering,
some surface colors and actors/HUD still differ or are absent.

Two additional Repeat controls remove only back-face culling. Both fail visibly:
a nearby rear surface occludes the street. Those unchanged failed originals stay
private; dropping culling is not a material/activation fix. Neither forcing alpha,
flipping all triangle winding nor removing resources follows from these results.

## Reproduce the addressing-only experiment

Use the existing [byte-source-over control command](byte-source-over-2026-11-05.md)
at either established camera, with a new output path, and append:

```text
--address repeat
```

Omit it, or use `--address clamp`, to retain the earlier mode. Blend and alpha
failure remain explicit caller choices. Nearest sampling, uniform state and storage
order are still diagnostic restrictions, not recovered runtime ordering or
per-asset policies. Free public CI and main:/docs GitHub Pages are unchanged.

## Next falsifiable step

Trace identifiable ground fragments with clipping-aware correspondence, then
corroborate their texture/alpha/pass inputs. Existing vertex observation records
contain position/STQ, not RGBA: they cannot verify runtime vertex alpha directly.
Absent exact-corner UV matches do not prove inactivity; clipping can create
intermediate vertices. Source projections alone do not establish native culling.

Retain the failed ground/no-cull controls. Complete material/ordering coverage,
scene activation, full GS precision, general transforms, camera motion and gameplay
remain open. The autonomous goal stays active, not complete.

## Subsequent cause isolated

The [constant-channel correction](constant-channels-2026-11-05.md) later removes
these recorded bands at both poses without changing addressing or geometry.
A constant128 interpolant was truncating to127 and skipping depth writes. The
Repeat/Clamp images above remain unchanged historical evidence, not current output.

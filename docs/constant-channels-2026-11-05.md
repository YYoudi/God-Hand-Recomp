# Preserve constant alpha: the recorded ground bands disappear

Date: 5 November 2026. Capture engine revision:
`befc14d444ba0446664f514192780efdd5f7a8c6`, committed before the new images.
Status: a generic numerical defect fixed in the ordered renderer; **not complete
material/scene reconstruction or gameplay**.

## A causal check, not another guessed visibility policy

Repeat restored texture detail but left blue ground bands. To distinguish missing
ground from a later overwrite, an ignored original Rust probe reused the public
reader/renderer and the same pinned dependency graph. Both complete-frame probe
PNGs first matched the official originals byte-for-byte. Source-buffer selection,
submission-order controls and raw RGB/stored-alpha/depth reporting were then added
only to that private diagnostic. Root source and Cargo.lock were not modified by
the probe; all407 registry version/checksum entries matched the public lockfile.

The isolated ground texture has raw alpha128 everywhere, and every expanded
ground vertex has exact f32 alpha128. Nevertheless, some covered pixels stored127
and zero depth. Neighboring pixels stored128 and positive depth. GT127 therefore
rejected those supposed opaque pixels for depth writes; later sky-colored draws
overpainted them. Reordering sky before ground hid the color symptom but retained
the incorrect alpha/depth, so it was not accepted as a pass-policy fix.

The defect was in **our float-to-byte input path**: a mathematically constant
interpolated128 could drift just below128 during clipping/rasterization, then
`floor` made it127. This is not a claim that WGSL interpolation violates its
floating-point specification, nor evidence that the game's own program is wrong.
No original game instructions, translated functions, shaders or backend rasterizer
code were consulted. Raw probes, source selections and technical dumps stay private.

## A general invariant, specified before implementation

The [contract](../specs/constant-vertex-channels-v1.md) preserves a channel only
when all three source vertices have exactly the same f32 value. The CPU writes
that reference and a four-bit equality mask into each GPU vertex of the triangle;
[WGSL flat interpolation](https://www.w3.org/TR/WGSL/#interpolation) transports it
without interpolation. Both provoking choices have the same reference because it
is repeated at all three vertices. Only masked channels select the exact value.

There is **no epsilon, forced alpha128, material override, pass reordering,
resource exclusion, geometry/texture edit or camera fitting**. Genuinely varying
channels keep the existing screen-linear path. An exact input immediately below128
still truncates below128. Fractional constants are preserved before the existing
floor/clamp/modulation; they are not rounded into opacity.

Internal GPU vertex stride grows from40 to64 bytes; public AlphaVertex inputs are
unchanged. The normal interactive inspector uses a separate existing path and is
not silently replaced. The ordered control intentionally changes its formerly
faulty pixels; all original before images are retained.

## Synthetic and actual validation

- 81 distinct Rust tests (70 base plus11 viewer-only),25 Python tests, formatting,
locked viewer check and linked build pass. CPU tests check explicit little-endian
serialization, per-triangle masks and exact fractional/subthreshold references.
- An invented oversized clipping quad reproduces the old failure immediately:
source alpha128, stored127 and no depth. No source game geometry or pixels were
used to construct that regression.
- Actual RTX5080 / Vulkan after the fix:11,440,128 constant-channel/clipping/
threshold sample checks pass, including unequal W, a behind-camera source vertex,
constant fractional/adjacent-threshold values and different constants in one draw.
All prior alpha, byte-blend, addressing, ordering and varying-channel checks pass.
- Four full-resource official renders (Keep/framebuffer-only at both fixed poses)
complete normally. Independent original-renderer probes reproduce the fixed full
PNGs byte-for-byte and verify stored alpha128 with positive ground depth at the
previously failed pixels. Stored alpha here is separate data, not PNG transparency.
- Original ISO SHA-256 unchanged. No prototype work. CI compiles the tools and
runs original CPU fixtures only; no game pixels, disc data or raw traces enter CI.

## Unchanged original reference and renderer captures

All156 nodes,353 buffers,23,686 triangles and67 referenced images remain included;
no missing-color fallbacks. Same512x448 output,4:3 projection, camera, source data,
Repeat, culling, source-over, GT127/framebuffer-only failure, unmasked GEQUAL depth
and storage order as the preceding iteration. Only constant-channel handling changes.

| Fixed pose | Reference | New ordered control |
| --- | --- | --- |
| First | [Reference preview](dashboard/media/edace008cb069c5c1a80cb803c1bf71aab90a7740ad57ee2cb3f91d3941cc9f5.png) / [unchanged PPM](dashboard/media/033e25087ed4fe723f77b608804cb4af404c25c6e8671f4a743cbf18ad355dad.ppm) | [Original fixed PNG](dashboard/media/8660693e4cf764ca6886bc944aa7d9918cf1c82649ead6cdfa600004f24e429d.png) |
| Second | [Reference preview](dashboard/media/91c07de4b15b05b38f9cd1b1e455edd4c2cef7815aff7e28d10a0a7796e1f2af.png) / [unchanged PPM](dashboard/media/6b4e4957d85122eb5166a6d4e068a5a685314522ad03492008de9e229fd2a83e.ppm) | [Original fixed PNG](dashboard/media/acda57e361027f2f86607f3b73ffe7952c0be3caa29f75daab5b659f48f11062.png) |

The recorded blue ground bands are absent in both new frames. These are actual
unretouched renderer PNGs, not patched screenshots. Reference PPMs are unchanged;
the linked reference PNGs are labeled lossless previews. The reference is the
third-party native backend, not hardware or PCSX2. Prototype remains separate.

| View | Before: exact Repeat iteration | After: exact constant-channel iteration |
| --- | --- | --- |
| First | [Before: bb7aedf](dashboard/media/467b8596967c777e11a8a2cb3ef3edf55614ce50ad1c4fd43655cdd3435755a0.png) | [After: befc14d](dashboard/media/8660693e4cf764ca6886bc944aa7d9918cf1c82649ead6cdfa600004f24e429d.png) |
| Second | [Before: bb7aedf](dashboard/media/8f7e4953c9ceeb7f09ba0a4dbc7b8a8038aa6fb27692a95b97c673b63a74ebc8.png) | [After: befc14d](dashboard/media/acda57e361027f2f86607f3b73ffe7952c0be3caa29f75daab5b659f48f11062.png) |

The before/after labels name code iterations, not that Bevy is an improvement over
the game. Both engine revisions were committed before their respective captures.
All earlier Clamp, Repeat and rejected controls remain preserved.

## Remaining gates

This fixes the observed bands at these two poses, **not every renderer defect**.
Uniform state and storage order are still controls, not recovered native material
or visibility assignment. Ground overlays/shadows and transparent ordering,
texture/color composition, filtering/fog and sky detail remain incomplete. Native
RGBA is not present in the retained vertex trace. Wider GS/fixed-point precision,
general transforms, original camera motion, actors/HUD and gameplay are unverified
or absent. No overall fidelity percentage is inferred from the test counts.

Next corroborate ground-layer order and per-draw texture/color/pass coverage at
both established views; do not replace the numerical fix with a guessed global
state policy. Free public CI and main:/docs GitHub Pages are unchanged. Goal active.

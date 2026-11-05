# Preserve exact constant vertex channels, version 1

Date: 5 November 2026. Status: CPU/GPU regression fixed; controlled retail comparison pending.
Provenance: original Rust/WGSL renderer, original synthetic fixtures and bounded
read-only retail render diagnostics. No game instructions, translated functions,
original shaders or third-party rasterizer code are implementation inputs.

## Observed failure and invariant

An isolated ground control has raw texture alpha128 at every texel and exact
vertex alpha128 on every expanded vertex. Some covered pixels store127 and fail
GT127, leaving zero depth; later sky color then overwrites them. Reordering passes
hides the visible symptom but leaves the wrong stored alpha/depth. Pixel reports,
source/executable hashes and subset controls stay private. This does not recover
native RGBA, per-draw materials or complete visibility.

The byte-input contract requires a channel identical at all THREE vertices of a
triangle to remain exactly that input value throughout clipping/rasterization,
before existing floor/clamp/modulation. Do not let floating interpolation drift
cross a byte threshold for a mathematically constant source. Constant fractional
values remain fractional until the existing truncation; an input immediately
below128 MUST remain below128. Never add a global epsilon, round arbitrary
vertex channels, force alpha128 or flatten genuinely varying channels.

## Original GPU representation

Keep public AlphaVertex inputs unchanged. For each consecutive triangle, compute
an exact-equality four-bit channel mask and retain its first vertex's four f32
color values. Repeat that metadata at all three GPU vertices. Fragment inputs for
that mask/reference use flat interpolation, so provoking-vertex/clipping choice
cannot change the reference. Select flat values only on masked channels; preserve
existing screen-linear interpolation on the other channels. Existing per-primitive
order, alpha comparisons, RGB/stored-alpha/depth masks, blending and sampler state
remain unchanged. No per-game identifiers or geometric special cases.

GPU vertex serialization becomes64 bytes: clip/UV/color remain at0/16/24; flat
color at40, mask u32 at56, zero padding at60. All values explicitly little-endian.
Existing finite/triangle/vertex/texture limits remain; vertex storage grows from
40 to64 bytes per expanded vertex. This is a numerical fix in the ordered adapter,
not a silent change to the normal inspector or a new game material policy.

## Acceptance

Add invented unequal-W/clipped triangles and oversized full-screen quads, constant
integer/fractional/adjacent-threshold colors, and mixed per-triangle constants in
one draw. Verify RGB, raw stored alpha and depth permissions at covered pixels;
include behind-camera source vertices without importing game geometry. First
observe the regression on the old implementation, then test the fix. Keep all
previous alpha/blend/address/interpolation tests, including varying channels and
intentional subthreshold floats. CI contains original synthetic fixtures only.

After CPU/GPU validation, compare BOTH full-resource poses using the same Repeat
and alpha-failure controls, no resource exclusions, object/camera refits or global
culling changes. Retain failed originals and report any remaining differences.
Unchanged diagnostic parameters can legitimately produce corrected pixels now;
never claim old faulty output is unchanged. Wider GS/fixed-point interpolation,
native color composition, filtering/fog, ordering/activation and gameplay remain
separate gates; do not claim full fidelity from this constant-channel invariant.

The flat interpolation mechanism follows the primary [WGSL interpolation
specification](https://www.w3.org/TR/WGSL/#interpolation); exact source constants
are copied to every triangle vertex, so no provoking-vertex assumption is needed.

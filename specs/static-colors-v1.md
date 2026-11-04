# Static MDB colors and modulation experiment, version 1

Date: 2026-11-04. Status: raw stream reading and an explicit rendering experiment,
not recovered per-resource GS state or full lighting. Prerequisites: static-mdb-v1,
scr-placement-v1 and inspection-camera-v1.

## Provenance and evidence

Existing independent MIT format research labels the fourth MDB stream as colors
but leaves its rendering interpretation unimplemented. Read-only observations
find four bytes per vertex in all 1,155 buffers of the three candidates; observed
channels are 0..128, with variable alpha. Raw data and texture extracts stay private.
The complete r100 SCP storage span matches the reference snapshot byte-for-byte.
Other portions of its surrounding DAT differ; do not claim the entire DAT matches
or that asset residency establishes which resources are active.

The native backend's existing passive GS draw trace was used without changing
its source or rendering. The recaptured reference framebuffer remains identical.
The trace observes modulation, other texture functions, fog, alpha tests/blending,
filtering and multiple targets/passes. A source stream is not the final lit vertex
color. No game instructions or translated functions were inspected. Opaque VU
program labels in private traces are not used as implementation input or published.

Hardware formula reference: [ps2tek, GS Textures](https://psi-rockin.github.io/ps2tek/),
independent primary hardware research. Modulate RGB multiplies texture and vertex
byte channels then divides by 128 with truncation. The experiment assumes final
clamping to 0..255. It is not an implementation of decal/highlight modes, fog,
lighting, the GS alpha pipeline, mip selection or postprocessing.

## Reader contract

Preserve the descriptor's optional color stream at +12 as raw RGBA-candidate
u8 quadruples, one per position. Keep absent streams explicitly absent; do not
invent colors in the disc layer. Existing bounds, overlap and total-vertex checks
apply. Never normalize or mutate the source data. Reject truncated/overlapping
color regions as before. Preserve all byte values, not only the observed 0..128.

## Explicit viewer experiment

Default inspection rendering remains unchanged. `--static-colors` enables only
for mesh/layout modes. It samples raw UNORM texture RGB (not hardware sRGB
decoding), interpolates source vertex colors in screen space, truncates/clamps
RGB using the /128 modulation formula, then converts the resulting sRGB-coded
bytes to linear for Bevy's sRGB output attachment. Do not multiply gamma-coded
GS colors directly into Bevy's linear PBR output. Missing streams use a visibly
documented neutral [128,128,128,128] fallback in the viewer only.

Alpha is explicitly diagnostic: multiply the texture reader's normalized alpha
by interpolated source A/128 and clamp, using ordinary host alpha blending.
The texture reader already maps/clamps GS alpha to display opacity; this cannot
implement every original alpha-test/blend/overbright case. Two-sided rendering,
nearest/repeat sampling and manual collection exclusions remain labeled limits.
This mode does not silently infer TFX=0 for every original draw.

## Acceptance

Synthetic fixtures test raw color order/count, absence, byte extremes and bad
ranges. CPU tests cover neutral 128, truncation, black, saturation, sRGB transfer
and why linear-space multiplication differs. A private synthetic GPU swatch
must check actual output bytes against the CPU oracle; it is not a game scene
or a public fidelity result. Build and capture the real retail scene with the
previous camera fixed, preserve earlier originals, and compare against the same
reference. Record improvements and regressions, not an invented quality score.

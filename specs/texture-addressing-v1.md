# Explicit texture addressing, version 1

Date: 5 November 2026. Status: CPU/GPU component validated; controlled retail comparison pending.
Provenance: original reader/renderer code, original synthetic sampling fixtures,
and previously retained data-only GS register observations. No game instructions,
translated functions or shaders are implementation inputs.

## Evidence and hypothesis

The MDB contract preserves signed UVs and uses V=rawV/4096+1. The normal inspector
uses Repeat; the initial separate ordered control uses Clamp. An integer UV
shift preserves Repeat sampling, not Clamp sampling. Never silently normalize
or clamp source UVs to hide this distinction.

Geometry-consistent UV candidates show GS wrapU=0/wrapV=0 on176 first-view and28
second-view buffers, with no multi-state buffers in that join. Counts overlap;
coverage and texture identity are not complete. Independent gsKit register
constants identify0 as Repeat and1 as Clamp:
[gsCore.h, pinned](https://github.com/ps2dev/gsKit/blob/8ef73d05f022d14b3ded7c70cba670fe333ed61a/ee/gs/include/gsCore.h).
Only enum meanings were consulted, not a game implementation or rasterizer.

The tested hypothesis is that Clamp interacts with out-of-range ground UVs,
selecting edge alpha instead of interior texels. This is NOT yet a recovered
material assignment or an explanation of every remaining visible difference.

## API and invariant

Add distinct Clamp/Repeat address modes per U/V axis to each explicit GPU draw.
Use bounded four-entry nearest-sampler cache; changing addressing is bind state,
not an extra shader/material sort key. Do not reorder draws or change RGB blend,
alpha/depth masks, culling, geometry, source colors or texture bytes.
Keep all existing constructors/control defaults explicitly Clamp; normal viewer
Repeat is unchanged. Add opt-in --address clamp/repeat to the static control,
applying the caller's chosen mode to both axes. Reject other/region modes; do not
silently approximate region clamp/repeat, mirror or filtering/mipmap semantics.

For nearest sampling, texel coordinates follow floor(uv*dimension): Repeat wraps
indices modulo the texture size (including negative coordinates); Clamp selects
the nearest edge index. Test normalized integer boundaries and independent axes.
Do not infer full GS precision, STQ/UV quantization or complete sampler state from
these host-component checks. Finite-input and existing image/vertex budgets apply.

## Acceptance and real comparison

Original no-disc GPU fixtures cover non-square textures, negative/out-of-range
UVs, four axis combinations, integer-offset equivalence under Repeat and an
explicit non-equivalence control under Clamp. Verify RGB, raw stored alpha and
depth, not only a visually plausible tile. Mixed-address draws in one frame and
submission permutations test cache/binding correctness. A synthetic transparent
border must expose the difference through alpha tests and source-over feedback.
No game pixels/data in fixture sets or CI; CI runs CPU helpers only.

Run the previous exhaustive alpha/blend suites and source/control parse tests.
Replay BOTH previous real Clamp poses with unchanged commands: original output
must stay byte-identical. Then vary ONLY explicit addressing at both fixed poses,
with Keep and framebuffer-only variants. Preserve all failed originals, original
ISO and prototype. Publish selected unchanged Reference / Bevy pairs and named
Clamp/Repeat iterations, with source revision, provenance and visible limitations.
Uniform state/storage order remain controls until per-draw coverage is validated;
no gameplay, full rendering or scene-activation gate is completed by this feature.

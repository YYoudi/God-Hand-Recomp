# Ordered GPU alpha-stage adapter, version 1

Date: 5 November 2026. Status: component contract, not a retail material mapping.
Prerequisites: fragment-write-permissions-v1.md and static-colors-v1.md.
Provenance: original Rust/WGSL implementation from the CPU permission contract,
pinned Bevy/wgpu public APIs, and independent ps2tek register documentation.
No game program, translated function, shader or backend rasterizer implementation
is an input. Existing raw runtime observations stay private.

## Ordered draws and targets

Provide a GPU component receiving a caller-ordered list of textured triangles,
explicit AlphaStage per draw, and homogeneous clip-space vertices/UV/raw vertex
colors. Do NOT route these draws through implicit opaque/transparent material
sorting. For each triangle in caller primitive order, render its passing branch then its
failure branch with complementary shader discard. Splitting a whole overlapping
draw into two branch-wide batches is forbidden: earlier failed fragments could
overpaint later passed primitives. Skip a branch only if its CPU write permissions
are all false. Keep comparison enabled/disabled, reference and failure explicit.
Do not infer per-asset state or scene activation.

Use separate targets: Rgba8Unorm for byte-coded RGB (its unused alpha remains1),
R8Unorm for raw framebuffer-alpha data, and Depth32Float with clear0 and
GreaterEqual. Pipeline RGB/alpha-target/depth-write masks come from the CPU
contract. Framebuffer alpha is DATA, never presentation transparency. Rendered
RGB must not change merely because that data alpha is zero. Readback exposes
raw RGBA by combining RGB with the separate data-alpha byte; it is not an image
for browser compositing. No post-capture alteration of historical screenshots.

The initial adapter has no blending, fog, lighting, destination-alpha test,
scissor, framebuffer bit masks, multisampling or hidden sort. This deliberately
isolates alpha-stage permissions; it is NOT GS blend replay. RGB writes overwrite
byte-domain UNORM values. Passing/failing fragments still undergo the explicit
depth comparison. Exact equal depths pass. Cull disabled for component fixtures.

## Fragment input

Sample a raw RGBA8 UNORM texture with nearest/clamp sampling; restore bytes by
round(sample*255). Vertex colors are byte-domain f32 interpolated in screen
space, truncated/clamped to u8. Modulate each texture/color component with the
existing integer /128 rule and final0..255 clamp. The resulting alpha BYTE is
the test input. This API requires raw alpha: the current retail texture reader's
normalized/clamped presentation alpha is NOT an equivalent input. Retail wiring
must first preserve/corroborate raw alpha; do not silently pass converted images.

## Safety and acceptance

Refuse zero/oversized targets, malformed textures/triangles, nonfinite vertices,
colors outside0..255, unsupported budgets and inconsistent
readback lengths. Finite clipW may be zero or negative: delegate valid clipping
to the GPU; never reject an entire real draw just because a vertex is behind
the camera. Test fully clipped and near-plane-straddling triangles.
Bound width/height64..1024, draws4096, vertices1,048,576,
texture bytes64MiB per input, draw-state allocations bounded. Poll GPU mapping
with an explicit finite timeout; propagate failure, never report missing output
as a pass. Use validated shader creation, no unsafe unchecked shader submission.

The opt-in no-disc conformance executable must use THIS adapter, not a second
shader implementation. Render all256 alpha values against all256 references,
all8 comparison modes, all4 failure actions, both depth-write masks, and disabled
control cases. Compare every RGB byte, stored alpha and depth value to the CPU
oracle. Test equality and later behind/in-front geometry to expose wrong depth
writes. Include byte-modulation and interpolated-alpha boundary fixtures.

No GPU execution in CI: CI compiles this executable and runs synthetic CPU tests;
actual local GPU results are recorded separately. No ISO or game pixels in either
fixture set. After component validation, preserve the production baseline and
wire evidence-supported retail state/alpha handling, then compare both real
full-resource views. This component alone is not goal completion.

Within-draw ordering is a correctness-first diagnostic path with up to two GPU
draw commands per triangle; no batching or performance claim is made.

Raw decoder integration, 5 November 2026: TIM2/TIM3 now offer a distinct raw
image API; the GPU adapter accepts it explicitly without conversion. The
synthetic exhaustive ramp exercises an original encoded TIM2/CSM1 input.
Production retail textures still use the unchanged presentation path; this
does not supply blending or evidence-supported per-draw state assignment.

Extension, 5 November 2026: byte-source-over-v1.md adds explicit per-draw byte
blending with safe previous-primitive RGB snapshots and explicit culling.
Overwrite remains available with unchanged conformance results. The no-blend
statements above describe the initial component, not the later optional subset.

Numerical correction, 5 November 2026: constant-vertex-channels-v1.md preserves
exactly equal per-triangle source channels through flat metadata, avoiding byte
threshold drift during clipping. The internal GPU vertex stride is now64bytes;
public inputs and nonconstant interpolation remain unchanged. Earlier failed
render originals are retained rather than described as unchanged output.

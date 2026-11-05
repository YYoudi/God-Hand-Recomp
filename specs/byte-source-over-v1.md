# Byte-domain source-over diagnostic, version 1

Date: 5 November 2026. Status: byte-input component validated; retail uniform controls remain incomplete.
Provenance: data-only passive GS register observations, existing geometry-consistent
UV candidates and independent [ps2tek hardware research](https://psi-rockin.github.io/ps2tek/).
No game instructions, translated functions or shaders are implementation inputs.

## Evidence and boundary

The first-view176 and second-view28 candidate buffers share enabled blending,
source/destination/source-alpha/destination selectors, PABE0, FBA0 and COLCLAMP1.
Counts overlap across views; UV joins are candidates, not texture identity or
complete mapping/ordering/scene activation. Private source digests and summaries
are retained. This supports investigating byte source-over, NOT assigning it to
all assets or assuming the full-resource renderer is recovered.

For raw RGBA8 inputs only, define each RGB output component as
clamp(destination + floor((source-destination)*source_alpha/128),0,255).
Use signed integer arithmetic; negative fractions round down, not toward zero.
Alpha128 is unity; above128 extrapolates before final clamp. This is not normalized
host source-over, not premultiplied alpha and not an sRGB/linear conversion.
Framebuffer alpha is source alpha only when alpha-stage write permissions permit;
it is NOT the host presentation opacity. Depth and alpha-failure masks remain
those in fragment-write-permissions-v1.md. Overwrite remains explicit and unchanged.

This bounded byte-input contract does not establish9bit source intermediates,
other selector equations, COLCLAMP0/wrap, PABE/FBA, destination-alpha tests,
fog/filtering, full raster precision or all GS behavior. Do not silently clamp
an unsupported wider source and call it conformance. Reserve those as open work.

## GPU ordering and feedback safety

Add explicit Overwrite / ByteSourceOver per draw to the shared original adapter.
Do not enable host blending. For each blended triangle in caller order, copy
current RGB to a distinct sampled snapshot BEFORE beginning its render pass.
Load RGB, stored-alpha and depth attachments; run complementary pass/failure
branches for that primitive before advancing. Both branches read the same snapshot
and have complementary coverage. Never sample a currently writable attachment,
reuse a whole-draw stale destination, reorder triangles or use stored data-alpha
as presentation transparency. The no-blend path must retain baseline results.
Full-size per-triangle feedback copies are a correctness-first diagnostic design,
not a performant renderer; retain existing input bounds and finite mapping polls.

## Acceptance

Original CPU tests exhaust all256 source/destination/alpha bytes against an
independent weighted-integer rational oracle, including negative and saturated
results. Actual opt-in GPU tests exhaust source/destination bytes at every alpha,
check all RGB/stored-alpha/depth values and exercise gradients, multiple layers,
equal/behind depth, alpha failure masks and overlapping mixed-outcome triangles
within one draw. No game data or pixels in fixtures/CI; CI executes CPU tests only.

After component validation, retail integration needs explicit state/mapping and
ordering evidence and both unchanged full-resource viewpoints. No production
viewer default changes or new fidelity claim follow from this component alone.

## Bounded retail state control (not material recovery)

Expose a separate no-window static-control executable requiring explicit personal
ISO, exact room, camera/viewport, blend overwrite/source-over, alpha-failure
keep/framebuffer-only and NEW PNG destination. Use existing layout placement,
all resources in storage order, raw textures and source colors. Fixed experimental
alpha test GT127, depth GEQUAL/unmasked; nearest/clamp sampling. Explicit optional
back-face culling; missing color streams use reported neutral128 diagnostic color.
This deliberately applies a UNIFORM caller-selected state, NOT recovered per-draw
materials or scene ordering. It is a falsifiable all-resource control motivated
by candidate observations, not a shipping default or acceptance substitute.

Reuse the bounded layout reader with separate raw output, preserving normalized
API/defaults; synthetic fixtures verify geometry/texture associations unchanged.
Reuse the actual GPU component and no-window initialization. CPU projection uses
the existing tested Bevy inspection camera contract. Encode the framebuffer RGB
bytes directly to a new PNG without desktop capture, cropping or postprocessing.
The source render alpha remains opaque while stored GS alpha stays separate.
Any real runs retain failures and compare both fixed poses without fitting objects
or moving the camera. Report resource counts, explicit state/order/filter limits
and missing colors. No claim of texture identity, activation, full GS precision,
performance, original camera behavior or full fidelity is implied.

Signed shader shifts follow [WGSL shift expressions](https://www.w3.org/TR/WGSL/#shift-expr);
actual GPU conformance checks negative products independently of host blend modes.

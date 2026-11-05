# Fragment alpha write permissions, version 1

Date: 5 November 2026. Status: original GPU-free alpha-stage model; not connected
to Bevy draw submission yet. No claim of full GS replay or recovered game materials.

## Provenance and observed reason

Independent hardware reference: [ps2tek, GS Tests and Pixel Control](https://psi-rockin.github.io/ps2tek/).
Register semantics, not a game implementation, inform the contract. No game
instructions, translated functions or shaders are implementation inputs.

The existing passive observations, joined to UV-candidate buffers, show enabled
alpha blending, unmasked depth writes and an alpha comparison greater than127:
176 candidate buffers in the first view and28 in the second. Failure either keeps
both buffers unchanged or permits framebuffer-only writes. Counts are per-view,
not disjoint resource coverage. UV matches are candidates, not texture identities.
Raw traces, program labels and per-buffer mappings stay private.

The public inspector currently gives every material AlphaMode::Blend. In pinned
Bevy0.20.1 this disables depth writes. A private controlled copy of our app changes
only the two material choices to Opaque: full-resource geometry reappears in both
views, while cutouts/shadows regress. All-opaque is a rejected control, NOT a
shipping policy or evidence that all sky resources are active.

## Alpha-stage behavior

Model unsigned eight-bit source alpha and reference. Support the eight named
comparison methods (never, always, less, less-or-equal, equal, greater-or-equal,
greater, not-equal). No floating normalization or gamma conversion is involved.

When testing is disabled or passes, permit RGB and framebuffer-alpha writes;
permit depth only when the caller's depth-write mask is off. On failure:

- Keep: no writes.
- FramebufferOnly: RGB and framebuffer alpha, never depth.
- DepthOnly: depth only, subject to its write mask.
- RgbOnly: RGB only; no framebuffer alpha or depth.

Return separate Boolean permissions. These are NOT final pixel updates: scissor,
depth/destination-alpha tests, framebuffer component masks, blending arithmetic
and pass order are separate gates. Do not infer which failure mode belongs to a
source asset from these generic functions. Do not use the private observation
mapping as an asset table in the public engine.

## Acceptance and next GPU gate

Use original synthetic integers only. Exhaust all alpha/reference combinations
for comparison partitions/counts and verify threshold endpoints. Test every fail
action, disabled and passing tests, depth-mask precedence, and the observed
127/128 boundary with both Keep and FramebufferOnly. Inputs must be unmodified.

Before wiring a GPU pass: specify ordering, byte-alpha interpretation, discarded
fragments, equality/depth behavior and host-vs-GS blend-color differences. Validate
with original synthetic overlapping geometry and known color/depth outcomes,
then both existing retail viewpoints without camera/object fitting. Preserve old
captures/default behavior. Do not ship an all-opaque or blindly source-opacity
heuristic as recovered per-draw state.

# Candidate room placement experiments

Date: 4 November 2026. The Rust/Bevy inspection viewer can place real retail
resource nodes. This is a diagnostic layout, **not a reconstructed playable room
or a visual-fidelity pass**. Each user still provides their own ISO.

## Implemented and checked

Original GPU-free affine math follows the
[versioned contract](../specs/scr-placement-v1.md): component scale, explicit
fixed-axis right-handed radian rotations, then unscaled translation. All six
rotation sequences are testable. Without a specified sequence, multiaxis nodes
fail rather than inherit an arbitrary library Euler convention. Runtime
handedness, translation units and general rotation order remain hypotheses.

Synthetic integration fixtures include a two-node shared-mesh resource and an
original indexed image. They check translations, unchanged UVs/indices/materials,
texture sharing, ambiguous-node context, invalid selections and aggregate limits.
No game pixels or data serve as test fixtures. The only third-party source
consulted for game-format semantics remains the independent MIT research already
credited in NOTICE.md; no game instructions or translated functions were read.

| Candidate | Nodes | Buffers | Vertices | Triangles | Referenced images |
| --- | ---: | ---: | ---: | ---: | ---: |
| r100 | 156 | 353 | 42,482 | 23,686 | 67 |
| r101 | 232 | 732 | 52,331 | 27,843 | 66 |
| r103 | 45 | 70 | 7,757 | 4,541 | 22 |

These match the earlier geometry counts. Referenced images are fewer than the
complete bank inventories (69/67/22); unused bank entries are not decoded again.
An additional candidate fails at its multiaxis node without an explicit order,
as intended. This does not prove that choosing one order would match the game.

## Visual results and failed attempts

1. A 120-second window smoke exited normally. Its desktop/window capture was
   occluded by an unrelated application, so it was rejected as evidence and
   never published. No unrelated application was controlled or closed.
2. A direct renderer PNG of all r100 resources showed an oversized white sky
   shape and a tiny town. A successful exit was not mistaken for a useful scene.
3. Explicitly excluding resource collections 27/28 produced a readable town
   overview: 148 nodes, 345 buffers, 42,018 vertices, 23,288 triangles, 64 referenced
   images. No filename heuristic or automatic visibility rule hides resources.
   The selected 30-second smoke saved its framebuffer PNG and exited normally.

The new `--capture-png` uses the renderer's own screenshot event, without desktop
pixels or image edits. PNG support adds codec dependencies to Cargo.lock. Missing
captures, early closure and save errors fail the bounded run. Existing capture
destinations are refused. The captured development build is labeled uncommitted;
private receipts preserve source and binary hashes rather than inventing a commit.

## Reference comparison

A fresh headless ps2-native-recomp run reloaded the existing first-fight snapshot
and captured frame 4501. The original output is P6 PPM, not PNG. The published
comparison retains the unchanged raw PPM and provides an explicitly labeled
lossless PNG preview; a local pixel-by-pixel conversion check passed. Bevy's PNG
is its original framebuffer capture. Neither image is an extracted texture sheet.

The pair in the [journal](https://yyoudi.github.io/God-Hand-Recomp/dashboard/#comparison-2026-11-04-retail-street-candidate-layout)
is **unmatched**: street-level reference versus candidate overhead inspection.
Exact camera alignment and scene association remain unverified. The reference
is a third-party renderer, not hardware or an independent PCSX2 cross-check.
It does not establish original scene activation, lighting, vertex colors, alpha
tests, culling, animation, collision, actors, HUD or gameplay. Materials remain
unlit/two-sided diagnostics. No visual score or FPS claim is made.

## Next evidence

Identify the same landmark in both views, add reproducible camera parameters,
and test placement/UV/material hypotheses from that comparable viewpoint. Treat
the sky rendering and runtime active-resource selection as unresolved work, not
as exclusions to bake into the game. Keep the prototype reference separate.

# Second-view check of unchanged retail placement

Date: 4 November 2026. This is another viewpoint, **not a before/after engine
change**. The renderer is the already published revision f83f076. Earlier
comparisons and their original files remain unchanged.

## Controlled reference and provenance

Resume the existing retail state recorded after analog movement and attack-button
inputs during the earlier native evaluation. The prototype is not used. Run the
unmodified native backend once without observation and once with its existing
passive GS vertex observer. Both newly rendered frame-6203 PPMs are byte-identical.
The draw log labels the corresponding observed frame 6202; preserve this numbering
distinction. This is a third-party backend reference, not console or PCSX2 evidence.
No original instructions, translated functions or game shaders were consulted.
Raw state, trace, program labels, source mesh/UV data and matrices remain private.

Repeat the existing UV-candidate matching and common projective fitting method
from the [first-view investigation](observed-projection-2026-11-04.md). Source
r100 geometry and SCR placements do not change; no object is fitted individually.

## Independent viewpoint and holdout

614 uniquely located candidate triangles span 19 resources and yield 566
deduplicated world/screen point candidates. Train on 419 points; reserve 147
points from **four whole resources**, not random triangles from those resources.

| Geometry observation check | Result |
| --- | --- |
| Training points within one pixel | 419 / 419 |
| Held-out points within one pixel | 147 / 147 |
| Held-out median residual | 0.0255 pixels |
| Held-out 95th-percentile residual | 0.0397 pixels |

These are geometric residuals to passive GS output, **not image-fidelity scores**.
UV uniqueness is not texture identity. Shared source/matching assumptions still
limit inference; this is not a formally separated clean-room claim.

Translation controls are useful here: omitting source translations, or multiplying
them by 0.01 a second time, still gives 139 training inliers but **zero of the
147 held-out points** within one pixel, with held-out medians around 160 pixels.
A locally plausible fit is not enough; keep the withheld-resource test.

The inferred camera has turned substantially and moved from the first pose.
It again suggests roughly 31-degree vertical FOV and 4:3 projection on 512x448
output. The centered Y-up inspection approximation has median residual 0.0377
pixels and 95th percentile 0.0621 across the candidates. It does not implement
camera-follow motion, collision or gameplay behavior.

All 19 matched resources again have **unit scales and zero rotations**. This
supports the static translation/local-coordinate interpretation in two views,
not general Euler order, nonuniform scales, parent hierarchy or other rooms.
An inherited private reporting label initially counted an absent saloon resource
as a fifth holdout. Corrected before publication: four actual resources; point
counts and all residuals were unchanged.

## Actual Bevy result and failed full-resource controls

The [new reference / Bevy pair](https://yyoudi.github.io/god-hand-rs/dashboard/#comparison-2026-11-04-second-view)
contains the unchanged frame-6203 PPM, its clearly labeled lossless PNG preview,
and the original Bevy framebuffer PNG. The background wall and near right
masonry share the observed framing, without moving any object or editing images.
The new pose uses the same explicit aspect, back-face culling and source-color
experiment as the first comparison; collections 27/28 remain manually excluded.

Full-resource default/color control renders were also tried. Their sky layers
**overpaint the street**. Both original failures remain local; no successful sky
or scene reconstruction is claimed. Those manual exclusions are diagnostic,
not recovered scene activation or a rendering fix. Material/depth/alpha/pass state
and which resources are active need further investigation.

Sky, dynamic actors, HUD, some props and full lighting/fog/filter/blending/pass
semantics are absent or wrong. This is not a playable independent game.

## Validation and next work

The actual bounded Bevy runs exit at ten seconds. A locked-build PowerShell
recapture is byte-identical to the initial color capture. The published Bevy
image records revision f83f076, not a later documentation commit. All three retail
layout candidates were re-inspected with the unchanged readers: 433 nodes,
1,155 buffers, 102,570 vertices and 56,070 triangles in total. Synthetic checks
remain 57 base Rust plus 3 additional viewer-only tests (60 distinct), and
25 Python tests. Game data is not supplied to tests or CI.

Next separate the sky/activation and per-pass depth/alpha issues before expanding
the scene. General rotation/scale semantics and camera motion remain independent,
unverified work; the wider geometry/scenes roadmap task is not marked complete.

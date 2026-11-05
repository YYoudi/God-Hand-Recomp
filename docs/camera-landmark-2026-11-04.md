# A common static landmark, not a recovered gameplay camera

Date: 4 November 2026. The comparison now identifies the yellow saloon sign and
its supporting facade in both the retail reference and a Bevy street-level view.
The [earlier unmatched overview](placement-2026-11-04.md) is retained unchanged.
This is a useful static-landmark check, not a whole-scene or camera-fidelity pass.

## Instrumentation and provenance

The original [camera contract](../specs/inspection-camera-v1.md) defines a finite
Y-up look-at pose, explicit vertical FOV and physical viewport. GPU-free parsing,
basis and projection tests complement the existing placement tests. There is no
new camera-follow, collision, animation or gameplay implementation.

A private original probe reuses the existing bounded ISO/FST/AFS/DAT/SCR/MDB/TM3
readers to inspect texture associations and primitive bounds. It identifies the
saloon sign's material and supporting mesh; names alone were not used to invent
the association. Texture inspection files and the private contact sheet are
excluded from Git. They are not reference screenshots and cannot be published
under the comparison-image exception. No game instructions, translated functions,
decompilation or prototype observations were consulted.

## Actual experiments

All four linked Windows/Vulkan runs captured original 512x448 framebuffer PNGs
and exited normally at their 10-second limits:

| Trial | Result |
| --- | --- |
| 01 | Wrong side of the building: rear facade and well. Rejected for reference alignment. |
| 02 | Correct facade direction, but a near projecting sign obstructs the camera. Preserved as a failed framing trial. |
| 03 | Saloon front and neighboring red wall identified. Sign is too low in the frame. |
| 04 | Manually adjusted horizon and bearing put the common sign near its reference position. Other depths still disagree. |

The published inspection uses:

```text
--layout st1/r100.dat --exclude-collection 27 --exclude-collection 28
--camera-eye 1.3,2.6,16 --camera-target 3.55,-1,-11.3
--camera-fov 45 --view-size 512,448 --smoke-seconds 10
--capture-png <new private PNG path>
```

These are manually chosen inspection parameters, not recovered game constants.
The target is a point on an extended viewing ray, not a claimed game target or
player position. No object placement was changed to fit the screenshot. Images
were not cropped, warped, repainted or AI-corrected. Near/far 0.01/2000 and the
viewport scale-factor override are diagnostic choices. Matching framebuffer
dimensions does not establish the PS2 display pixel aspect or original FOV.

## What the comparison establishes

The [new pair](https://yyoudi.github.io/god-hand-rs/dashboard/#comparison-2026-11-04-saloon-landmark)
is labeled **same-landmark**, not same-frame. The reference remains the original
retail frame 4501 from ps2-native-recomp at cba42f57: it is a third-party rendering,
not hardware or PCSX2. Its raw original PPM remains downloadable, with a labeled
lossless PNG preview. The Bevy image is a new original framebuffer PNG from the
uncommitted camera-development build; private receipts retain exact commands,
timestamps and source/binary hashes. Historical captures were not relabeled.

The sign's visible letters can be compared, but reference actors occlude part
of them. A yellow-pixel ROI is not the full sign bounding box, a calibration
oracle or an image-quality score. Fitting one facade cannot validate camera
intrinsics, units, depth placement, general Euler order or runtime scene activation.

Neighboring-wall parallax, surface colors, shadow/alpha state, sky, actors and
HUD still differ. Collections 27/28 remain explicitly excluded; materials are
still unlit/two-sided diagnostics. These discrepancies are not hidden by editing
the images. Prototype behavior has not been substituted for retail.

## Validation and next experiment

52 synthetic Rust tests, 24 Python tests, formatting and Bevy compilation pass.
Five CLI cases reject incomplete poses, orphan FOV, vertical/nonfinite cameras
and oversized viewports before opening a disc. Actual PNG dimensions were checked
separately from compilation. A fifth 10-second run through scripts/dev.ps1
repeated the final pose with byte-identical PNG output on this setup; this
checks local inspection repeatability, not fidelity to the reference. The source disc remains read-only.

Hold this camera fixed and compare additional depth-separated static landmarks.
Investigate discrepancies in placement/material interpretation and active-resource
selection rather than fitting each object independently or declaring general
fidelity from one manually aligned sign.

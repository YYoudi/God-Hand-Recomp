# Reproducible inspection camera, version 1

Date: 2026-11-04. Status: comparison instrumentation, not God Hand camera logic.
Provenance: original projective geometry and Bevy camera conventions; owned
retail framebuffer observations and bounded static-data placement probes.
No executable instructions, translated functions or decompilation consulted.

## Contract

Expose explicit `--camera-eye X,Y,Z`, `--camera-target X,Y,Z`,
`--camera-fov DEGREES` and `--view-size WIDTH,HEIGHT` for mesh/layout inspections.
Eye and target must occur together. FOV without a pose is an error. Default
vertical FOV is 45 degrees, near/far 0.01/2000 diagnostic units. These defaults
are engineering choices, not recovered game constants. Without a pose retain
the existing automatic overview camera. Never change object placement to make
an image fit the reference.

Use a right-handed, Y-up look-at basis. Reject nonfinite positions, coincident
eye/target and directions nearly parallel to the up axis (horizontal magnitude
less than 1e-6 times total direction length). FOV must be finite and within
10..120 degrees. Validate differences/normalization using f64 intermediates;
the renderer must use a finite normalized direction rather than subtracting
extreme f32 endpoints again. Preserve explicit eye values. Parsed vectors must
contain exactly three finite comma-separated f32 values, without ignored fields.

Viewport dimensions are physical pixels: 64..4096 per axis and at most 4,194,304
pixels. Use scale-factor override 1.0 so OS scaling does not change captures.
Matching the native 512x448 framebuffer is not proof of PS2 display pixel aspect
or original FOV. Record the actual captured dimensions, pose/FOV and scene
selection locally and in the public comparison description.

## CPU projection oracle

Expose a GPU-free projection of a finite world point to pixel coordinates.
Right = normalize(forward cross world-up); up = right cross forward.
Use the explicit vertical FOV and viewport aspect. The eye-depth interval is
[0.01,2000]; return no point outside it, including behind-camera points. Keep
offscreen coordinates unclamped so failed alignment remains measurable.
No fitting, camera solving or screen-error minimization is implied.

## Acceptance and limits

Synthetic tests cover exact parsing, invalid poses/FOV/viewport, cardinal camera
bases, screen center/right/up, near/far/behind rejection, aspect ratio and
unclamped offscreen output. Compare CPU projection with the same camera convention
used by Bevy, not an unrelated Euler-angle API. Build and inspect actual captured
frames at the requested dimensions. Preserve failed trials and previous originals.

Identify a repeatable landmark against the retail reference. Manual pose matching
is an observation aid: do not call it the recovered game camera, general runtime
transform order or pixel-perfect material/lighting parity. Keep unmatched/partial
comparisons labeled and keep the prototype separate.

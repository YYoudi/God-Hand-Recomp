# SCR placement experiments, version 1

Date: 2026-11-04. Status: explicit diagnostic hypotheses, not verified runtime
composition. Prerequisites: scr-nodes-v1.md, static-mdb-v1.md and tm3-tim3-v1.md.
Provenance: read-only owned retail data; independent Chihuahua MIT field layout
research already recorded in NOTICE.md. Its SCR exporter omits these node
transforms, so it is **not** evidence for their composition. No executable
instructions, translated game C++ or decompilation consulted.

## Evidence and unresolved questions

The three previously validated candidates contain unit scale, zero rotation,
and two Y-only rotations in r100. A wider private probe finds non-unit scales
and three multiaxis candidates too. Five other records fail the existing 112-byte
node profile; do not silently relax that profile or call all rooms validated.
Repeated angles near +/-1.570796 support radians. They do not determine handedness,
Euler order, runtime scene activation or parent transforms. Translation values
are plausible in the decoder's existing 0.01-scaled mesh space, but matching
runtime scale remains an experiment, not an established unit conversion.

## GPU-free transform contract

Provide an original finite-input affine experiment: scale local XYZ componentwise,
then apply fixed-axis right-handed rotations in radians, then add translation.
Do not multiply SCR translations by the mesh decoder's 0.01 again. The caller
must label this unit interpretation as a hypothesis. Y is height; no extra
handedness/axis flip, parent hierarchy or guessed matrix is inserted.

Support all six **application sequences** XYZ, XZY, YXZ, YZX, ZXY, ZYX explicitly.
For example XYZ means scale -> Rx -> Ry -> Rz -> translation, equivalently
T * Rz * Ry * Rx * S on column vectors. These labels are not an implicit mapping
to a library's intrinsic Euler convention. Without an explicit order accept only
zero or single-axis rotations, where these six sequences agree; refuse multiaxis
data instead of guessing. A user-specified order remains a hypothesis.

Positive quarter turns send +Y to +Z for X, +Z to +X for Y, and +X to +Y for Z.
Zero, nonuniform and negative scales retain their literal affine meaning;
do not reinterpret them as visibility flags. Calculate intermediates in f64,
reject nonfinite input and any result not representable as finite f32. Never
clamp or silently return an identity transform on failure. Preserve primitive
indices, UVs and material ordinals; reflection/singular-normal interpretation
is a separate rendering gate. Synthetic point results use 1e-5 tolerance.

## Room-candidate inspection

`--layout <exact FST path>` places every supported static SCR node in its DAT's
single SCP catalog using the experiment above. `--rotation-order xyz` (etc.) is
an explicit hypothesis; absent it, ambiguous rotations fail the whole load with
collection/node context. No unsupported node is silently skipped. This is a
candidate resource layout, not a claim that all resources are simultaneously
active in the game. Do not infer hierarchy from flat resource order.

Repeated `--exclude-collection N` is a manual diagnostic resource selection,
never an inferred visibility rule. Validate each ordinal is a nonempty SCR;
report the list and resulting counts. Reject an empty node selection. The first
all-resource r100 overview is dominated by sky resources. An explicitly logged
27/28 exclusion helps inspect geometry but does not prove correct scene activation.

All existing bounded readers stay in force. Cap total nodes at 4,096, primitive
buffers at 8,192, decoded vertices at 1,048,576, indices at 3,145,728, and referenced
decoded texture bytes at 256 MiB. These are engineering safety budgets, not PS2
limits. Decode each referenced texture once, check bank ordinals, and keep game
files in memory only. Materials and viewer remain diagnostic unlit/two-sided.
The isolated `--mesh` mode stays in local coordinates and is unchanged semantically.

`--capture-png <new file>` with a 10..120-second smoke captures the renderer's
own primary framebuffer halfway through the bounded run, without desktop pixels.
Use Bevy's screenshot event and lossless PNG encoding, not a desktop crop or an
image generator. Refuse an existing destination/non-PNG extension. Return failure
if saving fails or the asynchronous capture has not finished at the smoke deadline.
No image comparison or successful pixels are implied by a normal smoke exit.

## Acceptance and fidelity gates

Synthetic tests must exercise identity, each signed quarter turn, nonuniform
scale-before-rotation, translation-after-rotation, all six noncommuting sequences,
negative/zero scale, ambiguous-order rejection, and nonfinite/overflow rejection.
Room integration tests use only original synthetic containers/meshes/textures and
must check totals, translated coordinates, material sharing and whole-load errors.
Re-run the three real candidates and check their existing mesh/texture counts.
Link and visually inspect a bounded Bevy layout run, retaining its actual capture.

Compare identifiable static landmarks against a separately labeled retail
reference. Record agreements and contradictions in orientation/relative placement,
framing, units, UVs and materials. Original matched reference/Bevy screenshots
follow docs/progress-tracking.md. Do not mark runtime transform order, full room
identity, material parity, visibility or gameplay done on the basis of passing
math tests or a recognizable overview alone.

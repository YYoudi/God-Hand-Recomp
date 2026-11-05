# Session context â€” independent engine

## Current priority — owner decision, 2026-11-05

The final objective is a faithful, modifiable PC port and later remaster, not
necessarily Rust/Bevy. The independent-engine rendering goal is **paused, not
achieved**. Preserve its source, pending edits, tests and original evidence; do
not start new Rust features. Evaluate LasVola/godhand-recomp first through a real
local build and modification experiment, then LucasPicoli/god-hand-decomp as a
separate matching-decompilation/comprehension project. Follow
[the evaluation workflow](../docs/port-evaluation-workflow.md). Keep all third-party
checkouts, generated game code, ELF/assets/dumps and GPL experiments in ignored
research workspaces, never in the public MIT engine. Record exposure explicitly;
any future component using decompiled code is not an independent reimplementation.
No automatic republication of third-party content. No delegation. Public journal
and resources stay English, factual and on free GitHub Pages.


Reply to the owner in French; maintain public resources in English.
God Hand USA / SLUS_215.03: **independent Rust + Bevy 0.20.1 engine**, MIT for
original code. The paused branch's 2026-11-03 decision was original public source without
translated PS2 code. A personal ISO is required to launch.

Read AGENTS.md, docs/workflow.md, docs/clean-room.md, docs/architecture.md and ROADMAP.md.
Historical prompts 01â€“05 concern local research only.

First-milestone execution plan: docs/initialization-plan.md; adaptive decisions:
docs/initialization-decisions.md. Use scripts/dev.ps1. Preserve the separate retail
and prototype profiles in docs/reference-profiles.json. The original prototype CUE
names an absent BIN; scripts/reference_inventory.py can prepare an ignored local
CUE without editing the originals. Prototype debug controls remain unverified.

A third-party ps2-native-recomp trial reached the first fight and resumed a runtime
snapshot. See docs/native-evaluation-2026-11-03.md and docs/native-backend-outputs.md.
Game artifacts stay local; selected comparison screenshots now follow
docs/progress-tracking.md. This backend result is not Bevy-engine progress.

Foundation: crates/disc (read-only ISO), crates/engine (original simulation),
crates/app (CLI and optional Bevy sandbox). The game is not playable in this engine.
Y is height. CI fixtures are synthetic only. Never import decompiled or translated code.

The original floor/capsule scene has now rendered on Windows/Vulkan with a bounded
smoke exit. The bounded AFS catalog reader reports 1,445 entries on the local retail
ISO. FST ordinals now map all retail entries (49 directories) and all 1,078 prototype
entries (62 directories). Narrow TIM2 decoding accepts 34 of 35 top-level textures;
real 4/8-bit textures have displayed in Bevy, separate from the sandbox. No game
scene or in-game visual parity is established. Read docs/game-structure-2026-11-03.md,
specs/fst-catalog-v1.md and specs/tim2-indexed-v1.md. Do not start subagents until
the next scene contracts and tests establish meaningful independent work boundaries.
New room-data boundary: DAT/SCP catalog access covers 94/93 retail containers and
1,708 nested sections. SCR metadata on three candidates validates 156/232/45 nodes
and signed MDB references; static geometry decoding is described below. Continue from
docs/room-containers-2026-11-03.md and docs/local/overnight-checkpoint.md. Source
research uses Chihuahua's independent MIT format patterns, not original game code.
Static MDB and TM3/TIM3 decoding now validate 433 meshes / 158 bank images on the
same three candidates. A real textured well-like prop has rendered in Bevy from
the ISO. See docs/static-mesh-2026-11-03.md. Scene placement, lighting, runtime
parity and prototype debug access remain open; the mesh view is unlit/local.

After each session, update docs/dashboard/data.json with actual changes, checks
and limitations. Run python scripts/dashboard.py. Capture visual journal changes.
Check staged files with python scripts/check_public_tree.py.
Publish through free Pages from main:/docs without paid external hosting.

Owner update, 2026-11-04: read docs/progress-tracking.md and the current
`progress` / `comparisons` fields in docs/dashboard/data.json on every resume.
Maintain docs/local/progress-checkpoint.md at each meaningful boundary, with
verified revision, commands/results, evidence, failures and the next experiment.
Publish modest English updates after features and substantial findings. Keep
CPU tests, linked builds, actual rendering and fidelity validation distinct.
Selected original reference and Bevy PNG pairs are now authorized for public
GitHub/Pages comparison, unchanged with hashes, timestamps, runtime/revision,
profile, same-subject alignment and visible limitations. This replaces the
old journal-only image restriction, not the ISO/code/asset exclusions. MIT does
not cover game imagery and this is not a rights-holder/legal certification.
Do not invent a comparison or conceal mismatched camera/state. Register and
prepare pairs with scripts/comparisons.py; publish only approved media.

Placement increment, 2026-11-04: specs/scr-placement-v1.md defines explicit
S -> R -> T experiments and six fixed-axis application sequences. Unspecified
multiaxis order is refused, never guessed. `--layout` / `--inspect-layout` place
real resource nodes; repeated `--exclude-collection N` is explicit diagnostics,
not inferred game visibility. Three candidates reproduce 433 nodes/meshes.
r100 with collections 27/28 manually excluded rendered 148 nodes and 23,288
triangles; automatic overview, unlit/two-sided, no scene activation or runtime
parity established. `--capture-png` captures the renderer without desktop pixels
halfway through a bounded smoke; destination must be new. Read
specs/scr-placement-v1.md and docs/placement-2026-11-04.md before extending.
The public first-fight reference / candidate overview pair is explicitly
UNMATCHED. Reference PPM is downloadable unchanged; its PNG is a labeled lossless
preview. Do not turn this into a matched fidelity claim on resume.

Common-landmark increment, 2026-11-04: explicit diagnostic camera eye/target/FOV
and physical viewport are implemented under specs/inspection-camera-v1.md.
Read docs/camera-landmark-2026-11-04.md. A second public pair identifies the same
yellow saloon sign/facade at street level, manually fitted; it is NOT original
camera behavior or overall fidelity. The earlier unmatched overview is retained.
52 Rust synthetic tests; original 512x448 renderer captures and bounded exits.
Keep private texture inspection extracts/contact sheet out of Git. Next compare
additional depth-separated landmarks without tuning the camera to each one;
SCR/general rendering semantics and scene activation remain unresolved.

Source-color increment, 2026-11-04: read specs/static-colors-v1.md and
docs/source-colors-2026-11-04.md. MDB colors are preserved raw/optional.
--static-colors is an opt-in byte-domain modulation experiment, not full GS state
or lighting; default output was verified unchanged. 56 Rust / 25 Python tests;
private synthetic GPU probes check seven RGB samples and screen-linear color
interpolation. Do not supply game pixels/data to synthetic tests or CI.
The complete r100 SCP geometry/texture span matches the reference snapshot;
the WHOLE DAT does NOT match (opaque runtime-mutated sections). Residency does
not establish every resource is active. Native draw observations are frame4502,
paired traced/control frame4503; gallery reference stays4501. No original game
instructions/translated functions were inspected. Raw snapshots, traces, opaque
program labels, texture extracts and synthetic probe images remain PRIVATE.
Some color-mode shadows are excessively dark; never mark rendering parity done
or silently switch the default based on this experiment. Camera/placement and
original runtime material/light/fog/alpha/filter/pass semantics remain open.


Observation-backed projection increment, 2026-11-04: read
specs/inspection-camera-v1.md and docs/observed-projection-2026-11-04.md.
The earlier one-facade manually fitted camera is historical, not a validated pose.
Passive GS UV-candidate matching now supports one common projection across
whole-resource holdouts (862/865 points within1px; geometric residual, NOT image
fidelity). All86 matched resources have unit scales and ZERO rotations: this
cannot verify general Euler order or nonuniform scaling. --camera-aspect separates
projection aspect from framebuffer aspect; --backface-culling is explicit only.
The first new-pose two-sided captures were fully occluded; retain them privately.
A texture-mirror identity attempt failed and must not be used as binding evidence.
Defaults stay unchanged, including two-sided inspection. Source colors, sky,
lighting/fog/alpha/filter/passes, scene activation, camera motion and gameplay
remain incomplete. 57 base Rust +3 additional viewer-only tests;25 Python tests.
Run viewer-only synthetic adapter tests as well as locked base tests/checks.
Next validate a second retail viewpoint without per-object fitting. Private
raw data and opaque trace program labels remain excluded from publication.


Second-view validation, 2026-11-04: read docs/second-view-2026-11-04.md.
The same engine revision f83f076 predicts147/147 candidate points from FOUR
held-out resources after reference movement/turning; no per-object refits.
19 resources are unit-scale/zero-rotation: no general Euler/scale validation.
Trace6202/control capture6203 agree byte-for-byte. Actual Bevy replay agrees.
Full-resource default/color controls still overpaint the street with sky layers;
manual27/28 exclusions are NOT scene activation or a fix. Retain failed originals
privately; inspect material/depth/alpha/pass semantics next. Sky, actors/HUD,
complete rendering, camera motion and gameplay remain absent/incomplete.
60 distinct Rust /25 Python synthetic tests. Public pairs have raw originals;
no game data, dumps, translated code or trace labels enter CI/public engine.


Native-rendering goal, 2026-11-05: baseline5d35247; follow
docs/native-rendering-workflow.md and docs/render-pass-investigation-2026-11-05.md.
Goal ACTIVE: GPU integration and both-view validation remain required. Do not
mark complete after the CPU oracle or all-opaque control. specs/fragment-write-permissions-v1.md defines original byte-alpha comparisons and separate RGB/alpha/
depth permissions; crates/engine/src/fragment_alpha.rs is NOT wired to Bevy yet.
64 distinct Rust /25 Python tests. Private all-opaque controls restore geometry
but break shadows/cutouts/sky; reject as shipping behavior. Mode changes also
change queue/blending, not depth alone. Existing GS candidate states combine
alpha test/blending/depth writes; per-asset failure actions remain unassigned.
Next specify synthetic GPU discard/write paths and byte-vs-linear blend issues,
then real full-resource comparisons at both fixed poses. Keep historical images,
raw traces/mappings private, prototype separate, no delegation and no paid service.


Repository rename, 2026-11-05: public repository is now YYoudi/god-hand-rs;
Pages uses https://yyoudi.github.io/god-hand-rs/. Local workspace directory is
unchanged. The owner subsequently explicitly resumed the native-rendering goal;
keep it ACTIVE and work in background with milestone-only notifications.
Preserve historical screenshot originals.

Ordered GPU alpha increment, 2026-11-05: read specs/ordered-alpha-gpu-v1.md and
docs/ordered-alpha-gpu-2026-11-05.md. Shared app-library component and opt-in
godhand-alpha-check executable validate separate RGB/data-alpha/depth writes
without a disc or window. 67 distinct Rust /25 Python synthetic tests; actual
Vulkan exhaustive and spatial checks pass. No blending or production retail
viewer integration yet. Raw alpha is required: converted texture alpha is NOT
equivalent. No new retail fidelity or scene-activation result. Goal remains active.

Raw texture alpha increment, 2026-11-05: read specs/raw-texture-alpha-v1.md and
docs/raw-texture-alpha-2026-11-05.md. TIM2/TIM3 decode_raw returns a distinct
RawRgbaImage; existing decode remains presentation-normalized. Never invert the
lossy conversion or reinterpret RgbaImage. Bounded alpha inspector verified158
retail bank images /1,894,912texels with unchanged ISO; no asset extraction.
72 distinct Rust /25 Python tests; original synthetic encoded TIM2 now feeds
exhaustive GPU conformance. No blending/material mapping/production integration
or new retail fidelity claim. Next specify byte-domain blending/state and both
full-resource views, not another all-opaque shortcut. Goal remains active.

Byte-source-over/control increment, 2026-11-05: read specs/byte-source-over-v1.md
and docs/byte-source-over-2026-11-05.md. Original byte-input blend math and
per-primitive destination snapshots pass actual exhaustive Vulkan checks; a
stale whole-draw snapshot negative control fails as expected.78 distinct Rust
(70base+8viewer-only),25Python tests. Shared raw layout reader and separate
no-window godhand-static-control rendered all156r100nodes at BOTH fixed poses,
no exclusions/camera/object edits. Street visible, but BLUE GROUND HOLES and
material/sky errors persist. Uniform state/storage order are explicit controls,
NOT recovered materials, native draw order, scene activation or a shipping fix.
Both Keep/framebuffer-only failures retained; selected unchanged framebuffer-only
Reference/Bevy originals published. Default inspector unchanged. Next isolate
vertex-alpha/texture/pass inputs for identifiable ground resources before
changing material assignment. Do not hide failed holes or claim full GS9bit
intermediates from RGBA8 arithmetic tests. Goal remains ACTIVE; no delegation.

Texture addressing increment, 2026-11-05: read specs/texture-addressing-v1.md and
docs/texture-addressing-2026-11-05.md. Per-axisClamp/Repeat and explicit --address
clamp/repeat; defaults unchanged.80distinctRust (70base+10viewer-only),25Python;
147,456actualGPUaddress samples and previousalpha/blend suitespass. Four default
Clamp replays byte-identical; four Repeat variants restore texture detail, but
BLUE GROUND GAPS PERSIST. Two no-cull controls fullyocclude the street: rejected.
SelectedRepeat originals captured from committedenginebb7aedf; previousClamp
originals retained.176/28candidatebuffers showwrap0/0, notcomplete materialidentity
or activation. Existingtrace hasSTQ/position but NO RGBA; cannotprove runtime
vertexalpha from it. Next clipping-aware ground-fragment correspondence and
texture/alpha/pass mapping; absence ofexactcornerUVmatches isNOTinactiveproof.
No globalwindingflip/forcedalpha/deletingresources as guessedfixes. GoalACTIVE.

Constant-channel increment, 2026-11-05: read specs/constant-vertex-channels-v1.md
and docs/constant-channels-2026-11-05.md. This SUPERSEDES the current-gap claim
for new ordered-control captures only: the observed blue bands disappear inBOTH
unchanged fullresource poses. Rootcause was sourcealpha128 driftingto127 inour
clipped float-to-byte path, failingGT127 andnotwritinggrounddepth; later sky
thenoverpainted. Original isolatedRustprobe baseline wasbyte-identical before
isolation; no originalgameinstructions/shadercode consulted. NewinventedGPU
regression failsbeforefix andpassesafter. Per-triangle exact equalitymask+flat
reference preserves ONLYconstantchannels; noepsilon/forcedalpha, varyingchannels
unchanged. GPUvertexstride64; AlphaVertex publicinputsunchanged.81Rust
(70base+11viewer-only),25Python;11,440,128actualGPUconstant checks+prior suitespass.
Neworiginals from committedenginebefc14d, samecamera/resources/culling/order;
ISOunchanged, oldfailuresretained. This isNOTcomplete materials/activation or
fullGS precision; normalinteractiveinspector unchanged. Next nativegroundlayer
order/texture-color-pass coverage, sky/shadows/filtering. GoalACTIVE/no delegation.

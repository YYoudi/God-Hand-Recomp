# AGENTS.md — God Hand independent engine lab

Reply to the user in French. Maintain public resources and the journal in English.
Read docs/workflow.md, docs/clean-room.md and docs/architecture.md before changing
the engine. The user's 2026-11-03 decision supersedes the initial pipeline:
**independent Rust + Bevy reimplementation**, original public MIT code, personal ISO required.

## Scope

- crates/disc: read-only ISO 9660 reader, without game-code execution.
- crates/engine: original logic testable without a disc or GPU.
- crates/app: CLI and optional Bevy sandbox (--features viewer).
- specs/: behavioral contracts with provenance, without disassembly.
- docs/dashboard/data.json: single source of truth for roadmap and journal.
- scripts/dashboard.py: generates ROADMAP.md and journal HTML; --check detects drift.
- scripts/check_public_tree.py: validates the Git index before publication.

MIT covers our own code only. Never copy game disassembly, decompiled pseudocode,
translated C++ or incompatibly licensed code into the engine. Translation or renaming
does not make code clean room. Do not claim legal certification or a formal separation
that has not occurred. Document provenance.

## Persistent progress and comparison evidence — owner decision, 2026-11-04

Read docs/progress-tracking.md on every resume, together with the active goal,
Git state, docs/dashboard/data.json and docs/local/progress-checkpoint.md.
Maintain the local checkpoint at every useful boundary: last proven revision,
exact commands/results, private artifact paths, unknowns, failed hypotheses,
pending processes and the next falsifiable experiment. Public progress, tasks,
journal and comparison registry remain in docs/dashboard/data.json.
Distinguish observed, inferred, hypothetical, unknown and implemented/unverified;
CPU, build, rendered integration and reference fidelity are separate gates.

The owner now explicitly requests publication of selected original reference
screenshots AND their Bevy counterparts, like the Giants comparison workflow.
This supersedes the old journal-only image restriction, not the ISO/asset/code
exclusions. Use reviewed, same-subject Reference / Bevy pairs, unchanged original
screenshots (PNG, or backend PPM with a labeled lossless PNG preview), original-resolution downloads, SHA-256 filenames, timestamps, runtime and
revision provenance, alignment and visible limitations. Do not invent a matched
pair when a reference is missing. Before / After means named iteration revisions,
not that Bevy is better than the original. Keep retail/prototype separate.
Only entries validated in `comparisons` may publish to docs/dashboard/media/.
Use scripts/comparisons.py for byte-preserving preparation; never sweep private
evidence folders. This is the owner's publication request, not rights-holder
permission or legal certification; game imagery is not MIT. No game pixels in
synthetic test fixtures. Do not import Giants code, goals or app permissions.

## Workflow

For the first Rust foundation, follow docs/initialization-plan.md and record material
adjustments in docs/initialization-decisions.md. Use scripts/dev.ps1 for Doctor,
Check, Build, Inspect, Catalog and bounded Smoke actions. It defaults to one job
and no dev debug info after a Windows compiler crash; explicit overrides are available.
Reference profiles are in docs/reference-profiles.json. The prototype debug tools
are reported leads, not locally verified. Never substitute prototype behavior for retail.

The Windows/Vulkan sandbox has rendered original primitives and exited at its smoke
deadline. Read-only AFS catalog access is implemented under specs/afs-catalog-v1.md;
retail inspection reports 1,445 entries without extraction. FST mapping is now
validated on retail (1,445 files/49 directories) and prototype (1,078/62).
A bounded TIM2 subset decodes 34 of 35 top-level retail textures, refuses the
multi-palette font, and displays real ISO-backed images in a separate inspection
view. No scene/gameplay or reference-renderer parity is established. Read
docs/game-structure-2026-11-03.md and the FST/TIM2 contracts before extending this.
Work remains sequential until contracts justify the user's later parallel phase.
Room data now has bounded DAT/SCP catalogs: 94 retail outer containers and 93
nonempty SCP children (1,708 sections). SCR metadata validates 156/232/45 nodes
on candidates r100/r101/r103, including signed backward MDB references. Read
docs/room-containers-2026-11-03.md, specs/tagged-dat-v1.md and specs/scr-nodes-v1.md.
Actual room identities, runtime transform composition and visual parity remain
unverified; static mesh/topology/texture-bank scope is documented below. Public independent Chihuahua patterns are credited;
do not substitute matching decompilation repositories for independent implementation.
Static MDB flag-zero decoding and TM3/TIM3 banks now pass on the three candidates:
433 meshes, 1,155 buffers, 102,570 vertices, 56,070 triangles and 158 bank images.
An actual well-like prop (r100, SCP ordinal 23, node 0; five buffers/five textures)
has displayed in Bevy with a normal 60-second smoke exit. Read
docs/static-mesh-2026-11-03.md and the static MDB / TM3-TIM3 contracts. This is
unlit local-coordinate inspection, not scene reconstruction or runtime fidelity.
GS transfer addressing has independent synthetic table-oracle checks. Geometry
exposure includes independent MIT tool source for format semantics, no game code.

1. Specify observable behavior, provenance and acceptance criteria before implementation.
2. Synthetic fixtures only; never give game data to CI.
3. cargo test --workspace --locked; for Bevy, cargo check -p godhand-lab --features viewer --locked.
4. Update tasks, journal, actual validations and limitations in docs/dashboard/data.json.
   Mark tasks done only after verification; never invent FPS or gameplay results.
5. Capture the actual journal after visual changes. Publish selected original
   reference/Bevy pairs under docs/progress-tracking.md; other captures stay local.
   Register date, provenance, hashes and limitations before staging.
6. python scripts/dashboard.py; explicitly stage public files;
   python scripts/check_public_tree.py; python scripts/dashboard.py --check.
7. After pushing main, verify CI and Pages. Free Pages from main:/docs on github.io,
   standard public runners, no custom persistent artifacts or caches.

## Existing private workspace

God Hand (USA).iso, extraction/, GH-Prototype/, ghidra/, PS2Recomp/, recomp/, tools/,
downloads/, pcsx2-v2.8.2-windows-x64-Qt/ and .zcode/ are ignored. Never modify original
discs or extractions. Before prototype/diff work, read unpublished local
docs/prototype-tcrf-notes.md. Keep that research separate from implementation.
Historical prompts 01–05 do not describe the public-engine workflow.

God Hand and Bevy use **Y as height**. Both ELFs are stripped. An MCP entry in
.zcode/config.json does not guarantee availability in Codex; inspect session tools.

Machine audit, 2026-11-03: Rust 1.98, MSVC and GNU toolchains installed; gcc, CMake
and Ninja via G:/Tools/WinLibs/mingw64/bin. Historical vcvars64 path is absent.
Ghidra 12.1.2 is present. Local PS2Recomp is clean at e5636981; binaries are present
but not revalidated. Local docs/toolchains.lock.json pins research backends only.

Local evaluation: ps2-native-recomp cba42f57 built and observed through the first
fight. Runtime snapshot saved/reloaded; scripted inputs tested.
See docs/native-evaluation-2026-11-03.md and docs/native-backend-outputs.md.
Artifacts stay in recomp/experiments/native-2026-11-03/ and the ignored backend.
Local launcher: recomp/Play-GodHand.cmd (ISO required). This does not describe
Rust/Bevy compatibility. No original game code was consulted to implement an
independent-engine feature during this trial.

Keep this file, README and prompts/00-contexte-maitre.md consistent.
Report local artifacts with absolute paths. Do not delegate without explicit instruction.

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

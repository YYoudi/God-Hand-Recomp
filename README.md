# God Hand Â· Independent Engine Lab

## Current priority — owner decision, 2026-11-05

The final objective is a faithful, modifiable PC port and later remaster, not
necessarily Rust/Bevy. The independent-engine rendering goal is **paused, not
achieved**. Preserve its source, pending edits, tests and original evidence; do
not start new Rust features. Evaluate LasVola/godhand-recomp first through a real
local build and modification experiment, then LucasPicoli/god-hand-decomp as a
separate matching-decompilation/comprehension project. Follow
[the evaluation workflow](docs/port-evaluation-workflow.md). Keep all third-party
checkouts, generated game code, ELF/assets/dumps and GPL experiments in ignored
research workspaces, never in the public MIT engine. Record exposure explicitly;
any future component using decompiled code is not an independent reimplementation.
No automatic republication of third-party content. No delegation. Public journal
and resources stay English, factual and on free GitHub Pages.


An independent **Rust + Bevy** engine reimplementation for **God Hand (USA, PS2)**.
Original code is public under MIT. Each user must provide their own ISO.
No game code, source models, textures, audio or level files are distributed.
Selected rendered reference/Bevy screenshots may illustrate progress; game imagery
remains owned by its rights holders and is not covered by MIT.

**Status: early foundations. The game is not playable in this engine.**
The read-only ISO/FST/AFS readers can locate and decode a narrow TIM2 texture subset.
The texture inspection view reads your ISO; the sandbox contains original primitives only.

- [Development journal](https://yyoudi.github.io/god-hand-rs/)
- [Roadmap](ROADMAP.md) Â· [Workflow](docs/workflow.md)
- [Progress tracking and original reference/Bevy comparisons](docs/progress-tracking.md)
- [First Rust milestone and adaptive workflow](docs/initialization-plan.md)
- [First game-data map and next fidelity experiments](docs/game-structure-2026-11-03.md)
- [Room-container and node metadata findings](docs/room-containers-2026-11-03.md)
- [First static mesh and texture-bank validation](docs/static-mesh-2026-11-03.md)
- [Candidate placement experiments and unmatched reference comparison](docs/placement-2026-11-04.md)
- [Clean-room method](docs/clean-room.md) Â· [Architecture](docs/architecture.md)
- [PS2 recompilation research](docs/research-2026-11-03.md)
- [Contributing](CONTRIBUTING.md) Â· [Licensing and scope](NOTICE.md)
- [Native backend outputs and their use](docs/native-backend-outputs.md)

## Getting started

Install Rust **1.98+** and the [Bevy prerequisites](https://bevy.org/learn/quick-start/getting-started/setup/).
Bevy is pinned to **0.20.1**; Cargo.lock pins its dependencies.
Tests require neither a GPU nor game data.

```sh
cargo test --workspace --locked
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso"
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --catalog
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-textures
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-rooms
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --room "st1/r100.dat"
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-meshes "st1/r100.dat"
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --mesh "st1/r100.dat" --collection 23 --node 0
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --texture "etc/subscr/face_hm00.tm2"
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --sandbox
```

Startup requires an ISO whose SYSTEM.CNF references SLUS_215.03.
This recognizes a structure and serial number; it does not certify authenticity.
The sandbox displays a floor and a capsule, controlled with WASD or arrow keys.
It is not a scene from the game.
The separate texture view supports single-picture 4/8-bit indexed TIM2 with one
RGBA32 palette. Other layouts are refused. Rendering an isolated asset does not
establish a faithful game screen; reference-runtime comparison remains open.
Room inspection validates storage sections and SCR node/MDB references. The narrow
static MDB and TM3/TIM3 readers can display an isolated textured asset from the ISO.
The mesh view uses local coordinates. Candidate resource layouts now apply
explicit affine hypotheses; multiaxis order is never guessed. Game scene
activation, lighting and reference-runtime parity remain open. Candidate paths still need
to be associated with a repeatable in-game scenario.

Windows: use a GNU toolchain with `cargo +stable-x86_64-pc-windows-gnu ...`
and MinGW on PATH, or install Visual C++ Build Tools and the Windows SDK for MSVC.

## Public source and local data

| Public | Excluded from Git |
| --- | --- |
| crates/: reader, original logic, Bevy application | ISO, BIOS, ELF, IRX, AFS |
| specs/: contracts with provenance | Translated game code, game binaries, dumps |
| docs/: architecture, roadmap, journal | Extracted data, Ghidra projects, saves |
| scripts/: tracking and validation tools | Research backends, unselected captures |
| Reviewed original reference/Bevy screenshot pairs | Source assets, dumps and private capture receipts |

Static recompilation remains **local research**. Its generated code does not feed
the public engine. Translating MIPS into C++ does not establish a clean-room implementation.

A [local ps2-native-recomp trial](docs/native-evaluation-2026-11-03.md) reached the
first fight and resumed a runtime snapshot. This is a third-party backend result;
our independent engine remains at the foundation stage.

## Free public journal

GitHub Pages publishes **main:/docs** on the free github.io domain.
The static journal uses no paid API or external hosting service.
CI uses standard public Ubuntu runners without caches or custom artifact uploads.

Update docs/dashboard/data.json, run `python scripts/dashboard.py`, capture visual
journal changes, then check staged files with `python scripts/check_public_tree.py`.
Public resources are in English; conversations with the project owner remain in French.

Progress is recorded after each feature or substantial finding, not estimated as
a percentage. Public tasks, journal and comparison metadata live in
`docs/dashboard/data.json`; private resumable checkpoints retain exact commands
and evidence. Visual comparisons publish the original reference and Bevy files
side by side with full-resolution links, provenance and explicit differences.
Missing or unmatched reference evidence is reported, never manufactured.

Inspect a placement experiment (not a fidelity-validated scene):

```sh
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-layout "st1/r100.dat"
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --layout "st1/r100.dat" --exclude-collection 27 --exclude-collection 28 --smoke-seconds 30 --capture-png "/new/private/capture.png"
```

Manual resource exclusions are logged diagnostics, not game visibility rules.
The first public reference/overview comparison is explicitly unmatched. The raw
native PPM screenshot is preserved alongside its lossless PNG viewing preview.

For reproducible **inspection** camera poses, use `--camera-eye X,Y,Z`,
`--camera-target X,Y,Z`, optional `--camera-fov DEGREES`, and `--view-size W,H`.
The [saloon comparison](docs/camera-landmark-2026-11-04.md) identifies a common
static landmark; its manually fitted pose is not the original gameplay camera.

An optional `--static-colors` mode for mesh/layout inspection consumes raw source
vertex colors using a byte-domain modulation experiment. It is **not complete
game lighting or GS state replay** and remains disabled by default. See the
[source-color evidence and before/after limits](docs/source-colors-2026-11-04.md).


The [latest static-projection observation](docs/observed-projection-2026-11-04.md)
compares whole-resource holdouts and renders the street with explicit inspection
aspect/culling. It does not establish camera motion, general SCR composition or
full rendering fidelity; previous original comparisons are retained.

The [second-view check](docs/second-view-2026-11-04.md) supports unchanged static
placement with whole-resource holdouts. Full-resource sky controls still fail;
this is not completed scene reconstruction or general runtime transform parity.


The [new autonomous rendering goal](docs/native-rendering-workflow.md)
starts from the sky-overpainting failure. Its [first investigation](docs/render-pass-investigation-2026-11-05.md)
adds original CPU alpha-stage write permissions, now consumed by the separate
ordered GPU control; complete runtime integration remains pending.
All-opaque diagnostic captures are rejected controls, not completed native rendering.

Repository: [god-hand-rs](https://github.com/YYoudi/god-hand-rs). The native-rendering goal has been explicitly resumed by the owner.

The [ordered GPU alpha component](docs/ordered-alpha-gpu-2026-11-05.md) passes
synthetic offscreen Vulkan checks. The normal interactive inspector is unchanged;
a byte-blend subset and all-resource controls exist, but complete runtime
materials and rendering remain open.

[Raw texture alpha](docs/raw-texture-alpha-2026-11-05.md) is preserved separately
from presentation transparency; the existing viewer output policy is unchanged.
The bounded inspector reads personal ISO data without extracting assets.

The [byte-blend control](docs/byte-source-over-2026-11-05.md) now renders all
resources at both fixed inspection poses without a window. The street appears,
but its original ground-gap controls exposed a numeric bug, now corrected below.
Material/sky errors remain. This is an explicit uniform-state
experiment, not recovered game materials, scene activation or playable content.

[Explicit Repeat addressing](docs/texture-addressing-2026-11-05.md) restores
texture detail in the all-resource control. Addressing alone did not fix the
ground gaps; previous Clamp and Repeat originals remain preserved.

The [constant-channel correction](docs/constant-channels-2026-11-05.md) removes
the observed blue ground bands at both unchanged full-resource poses. It fixes
our numeric input path, not complete material/scene activation. Sky/shadows,
full precision, camera behavior and gameplay remain incomplete.

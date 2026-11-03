# God Hand · Independent Engine Lab

An independent **Rust + Bevy** engine reimplementation for **God Hand (USA, PS2)**.
Original code is public under MIT. Each user must provide their own ISO.
No game code, models, textures, audio or levels are distributed.

**Status: early foundations. The game is not playable in this engine.**
The read-only ISO/FST/AFS readers can locate and decode a narrow TIM2 texture subset.
The texture inspection view reads your ISO; the sandbox contains original primitives only.

- [Development journal](https://yyoudi.github.io/God-Hand-Recomp/)
- [Roadmap](ROADMAP.md) · [Workflow](docs/workflow.md)
- [First Rust milestone and adaptive workflow](docs/initialization-plan.md)
- [First game-data map and next fidelity experiments](docs/game-structure-2026-11-03.md)
- [Room-container and node metadata findings](docs/room-containers-2026-11-03.md)
- [Clean-room method](docs/clean-room.md) · [Architecture](docs/architecture.md)
- [PS2 recompilation research](docs/research-2026-11-03.md)
- [Contributing](CONTRIBUTING.md) · [Licensing and scope](NOTICE.md)
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
Room inspection validates storage sections and SCR node/MDB references. It does
not yet decode geometry or reconstruct a room. Candidate paths still need to be
associated with a repeatable in-game reference scenario.

Windows: use a GNU toolchain with `cargo +stable-x86_64-pc-windows-gnu ...`
and MinGW on PATH, or install Visual C++ Build Tools and the Windows SDK for MSVC.

## Public source and local data

| Public | Excluded from Git |
| --- | --- |
| crates/: reader, original logic, Bevy application | ISO, BIOS, ELF, IRX, AFS |
| specs/: contracts with provenance | Translated game code, game binaries, dumps |
| docs/: architecture, roadmap, journal | Extracted data, Ghidra projects, saves |
| scripts/: tracking and validation tools | Research backends, gameplay captures |

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

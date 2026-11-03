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

1. Specify observable behavior, provenance and acceptance criteria before implementation.
2. Synthetic fixtures only; never give game data to CI.
3. cargo test --workspace --locked; for Bevy, cargo check -p godhand-lab --features viewer --locked.
4. Update tasks, journal, actual validations and limitations in docs/dashboard/data.json.
   Mark tasks done only after verification; never invent FPS or gameplay results.
5. Capture the actual journal after visual changes. Gameplay captures stay local.
   Register date and provenance in the manifest before staging.
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

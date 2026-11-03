# Session context — independent engine

Reply to the owner in French; maintain public resources in English.
God Hand USA / SLUS_215.03: **independent Rust + Bevy 0.20.1 engine**, MIT for
original code. The user's 2026-11-03 decision is original public source without
translated PS2 code. A personal ISO is required to launch.

Read AGENTS.md, docs/workflow.md, docs/clean-room.md, docs/architecture.md and ROADMAP.md.
Historical prompts 01–05 concern local research only.

First-milestone execution plan: docs/initialization-plan.md; adaptive decisions:
docs/initialization-decisions.md. Use scripts/dev.ps1. Preserve the separate retail
and prototype profiles in docs/reference-profiles.json. The original prototype CUE
names an absent BIN; scripts/reference_inventory.py can prepare an ignored local
CUE without editing the originals. Prototype debug controls remain unverified.

A third-party ps2-native-recomp trial reached the first fight and resumed a runtime
snapshot. See docs/native-evaluation-2026-11-03.md and docs/native-backend-outputs.md.
Game artifacts and captures stay local; this is not Bevy-engine progress.

Foundation: crates/disc (read-only ISO), crates/engine (original simulation),
crates/app (CLI and optional Bevy sandbox). The game is not playable in this engine.
Y is height. CI fixtures are synthetic only. Never import decompiled or translated code.

The original floor/capsule scene has now rendered on Windows/Vulkan with a bounded
smoke exit. The bounded AFS catalog reader reports 1,445 entries on the local retail
ISO; FST and payload decoding remain unimplemented. Read specs/afs-catalog-v1.md.

After each session, update docs/dashboard/data.json with actual changes, checks
and limitations. Run python scripts/dashboard.py. Capture visual journal changes.
Check staged files with python scripts/check_public_tree.py.
Publish through free Pages from main:/docs without paid external hosting.

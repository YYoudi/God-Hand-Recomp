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
ISO. FST ordinals now map all retail entries (49 directories) and all 1,078 prototype
entries (62 directories). Narrow TIM2 decoding accepts 34 of 35 top-level textures;
real 4/8-bit textures have displayed in Bevy, separate from the sandbox. No game
scene or in-game visual parity is established. Read docs/game-structure-2026-11-03.md,
specs/fst-catalog-v1.md and specs/tim2-indexed-v1.md. Do not start subagents until
the next scene contracts and tests establish meaningful independent work boundaries.

After each session, update docs/dashboard/data.json with actual changes, checks
and limitations. Run python scripts/dashboard.py. Capture visual journal changes.
Check staged files with python scripts/check_public_tree.py.
Publish through free Pages from main:/docs without paid external hosting.

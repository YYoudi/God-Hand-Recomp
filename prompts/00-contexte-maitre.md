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

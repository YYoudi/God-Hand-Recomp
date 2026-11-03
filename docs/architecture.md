# ADR 001 — Rust + Bevy independent engine

Date: 2026-11-03. Status: adopted at the user's request.

## Decision

Use Rust for the original engine and Bevy 0.20.1 for ECS, rendering and input.
The engine does not load or execute the MIPS program or link its translation.
Game data will be read from the user's own ISO.

PS2Recomp and ps2-native-recomp remain local research references, not public-engine
dependencies. Their God Hand compatibility is not progress in our independent engine.

| Layer | Responsibility | Foundation |
| --- | --- | --- |
| godhand-disc | Read-only ISO 9660 access, bounds checks | Root directory, SYSTEM.CNF, identification |
| Formats | AFS/FST, geometry, textures, skeletons, audio | To be specified |
| godhand-engine | Original GPU-free testable simulation | Generic X/Z movement |
| Bevy application | Window, rendering, camera, input | Optional floor/capsule sandbox |
| Game | Scenes and observable behavior | Not implemented |

The 60 Hz step and sandbox speed are provisional engineering choices, not measurements
of the game. Y is height. Sample input intent per rendered frame and consume it on
the fixed step. Future input edges must be buffered.

## Work required

Formats and scripts, scenes, rendering, animation, collisions, combat, AI, progression,
audio, saves and full-level validation. Bevy does not supply God Hand's logic.
No completion date or gameplay coverage is promised.

## Alternatives

PS2Recomp (GPL-3.0): translation and game-specific handlers; experimental runtime.
ps2-native-recomp (MIT): practical recompilation route, locally tested through the
first fight. PCSX2: behavioral reference, not the Bevy engine.
See research-2026-11-03.md, native-evaluation-2026-11-03.md and
native-backend-outputs.md for evidence, outputs and boundaries.

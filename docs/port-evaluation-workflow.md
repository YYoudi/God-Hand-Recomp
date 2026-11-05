# Faithful PC port and remaster evaluation

Owner priority change: 2026-11-05. No new Rust features during this evaluation.
The earlier rendering objective is paused, not completed. Existing uncommitted
Rust diagnostic work stays preserved locally, separately from the new research.

## Execution objective

Find the most faithful, maintainable and demonstrably modifiable PC codebase.
Start with [LasVola/godhand-recomp](https://github.com/LasVola/godhand-recomp).
Audit [LucasPicoli/god-hand-decomp](https://github.com/LucasPicoli/god-hand-decomp)
separately as a possible comprehension complement, not a presumed PC port.

1. Record Git state, previous goal and owned processes. Finish or log engaged
   checks only. Preserve original ISO, prototype, Rust sources/tests/captures.
2. Pin each checkout revision, nested dependencies and licenses in an ignored,
   isolated research workspace. Distinguish author claims and local observations.
3. Audit actual build configuration and generation path before running it. Record
   tool versions, commands, source/build modifications and output identities.
   No ISO/ELF/generated game code/extracted asset/dump belongs in the public repo.
4. Build LasVola from the owner's untouched original USA ISO. Test startup, first
   scene, fighting/input, in-game saving/loading and any separate runtime snapshot
   facility. A snapshot is not a memory-card save; log each independently.
5. Trace a resource loading/rendering path in that separate project. Demonstrate
   a minimal original external texture replacement with identical-view unchanged
   captures before/after, or reproduce/document the precise blocking condition.
   Merely booting the game does not prove remasterability.
6. Inspect LucasPicoli's actual C/C++, generated outputs, assembly fallbacks,
   matching measurements and toolchain. Attempt relevant builds where feasible.
   An identical target ELF does not establish complete matching C or a PC build.
7. Compare fidelity, stability, modification cost, asset replacement, rendering
   control, maintainability and license obligations. Recommend upstream work,
   separate GPL fork, hybrid workflow or independent work based on reproduced facts.
8. Update dashboard data, roadmap and English journal on substantial findings.
   Only reviewed unchanged screenshot pairs may publish under the comparison
   protocol; preserve original source/revision/runtime provenance. Never quietly
   label third-party runtime screenshots as Bevy independent-engine progress.
   Stage only original documentation/reviewed evidence; verify privacy, CI/Pages.

## Provenance boundary

MIT covers our original engine/tools only. Do not copy GPL or decompiled game
code into it. Audit/source exposure must be recorded; future affected components
cannot claim independence. A GPL continuation needs a distinct project/fork and
its corresponding license/source notices. A repository license does not grant
rights-holder permission for the game. Never automatically upload third-party
checkout contents, generated code or binaries. Keep prototype separate.

## First deliverable

An actually evaluated local build plus a reproducible original modification (or
precisely evidenced blocker), exact pinned dependencies/licenses, and an adapted
roadmap. Unknown/unreproduced gates stay open; screenshots and authors' README
claims are not substitutes for builds, input tests or persistence checks.

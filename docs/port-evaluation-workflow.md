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

## Continuation direction — 2026-11-06

The owner accepted LasVola as the PC continuation base, not another Rust rewrite.
Prepare a distinct GPL continuation with a publication allowlist, pinned upstream
source acquisition and no generated game material. Audit tracked upstream files
before choosing a public fork/mirror; a dependency-driven original tooling repo
is preferable to accidentally republishing excluded material. Keep this MIT lab
intact. First extension: named external resources with original synthetic tests,
retail restoration controls and explicit resolution/format limits.

Stop autonomous attempts to finish combat stages. Persistence can use a legitimate
owner-provided save when available; a prototype debug shortcut may assist prototype
research but cannot establish retail saving, progression or fidelity.

Prototype work is a separate experiment: read the owner's saved HTML, identify the
actual MODE1/2048 disc/boot ELF, preserve originals and test generation/build with
prototype-specific metadata. Never bypass retail hash checks and run retail code
against the prototype disc. Distinguish discovery errors, generated C++, successful
object compilation, linked executable, actual boot and debug-menu operation.
A precise blocker is an acceptable prototype experiment result, not a playable claim.

### Separate public continuation tooling

[YYoudi/god-hand-pc](https://github.com/YYoudi/god-hand-pc) contains original GPL
bootstrap tooling, not a mirror of upstream or generated game code. It fetches a
pinned dependency into ignored local storage, can register an existing clean
checkout and verifies the retail ISO read-only. Its 62 original synthetic tests
and public CI pass. The named-resource metadata planner was tested with the
personal retail disc and bridged to the evaluated private adapter: replacement
and restoration were observed in separate fresh native boots. The public runtime
bridge and full fresh-disc build pipeline remain pending; this is not a finished
installer or remaster. See the named-resource experiment report. The journal
stays on this existing free Pages site.

The isolated personal-disc setup is now undergoing a separate fresh-build check; see [fresh setup trials](retail-setup-2026-11-06.md). Preparation and generation pass; the first stale-target failure is preserved, and the corrected workflow explicitly reconfigures after generation. Do not substitute those gates for a linked executable or observed fresh boot.

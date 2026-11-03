# G1 — first validated Rust foundation

Local gates are verified; the current public revision's validation is tracked in
GitHub Actions. This is the first bounded implementation goal, not a promise to
recreate the complete game in one pass. The compatibility target is the USA retail
build. Public resources remain English; owner-facing conversation remains French.

## Deliverable and completion criteria

G1 completes when all of the following are supported by recorded evidence:

1. A reproducible local command builds and links the Rust/Bevy viewer.
2. The native synthetic sandbox opens, renders original primitives and exits normally
   through its bounded smoke-test path. A compile check alone does not satisfy this.
3. Retail ISO access exposes a bounded AFS header/catalog reader without extracting
   the whole archive. Invalid counts, extents and truncated inputs have synthetic tests.
4. Retail and prototype observation profiles are explicit, with original images
   preserved and version-specific observations kept separate.
5. Checks, provenance, limits and current decisions are reflected in the public
   repository and journal; private game data and translations remain excluded.

First textured scenes, original animation, combat and full-game parity belong to
later milestones. G1 remains active until its criteria are actually met.

## Executable development loop

From the repository root on Windows:

```powershell
powershell -NoProfile -File scripts/dev.ps1 -Action Doctor
powershell -NoProfile -File scripts/dev.ps1 -Action Check
powershell -NoProfile -File scripts/dev.ps1 -Action Build
powershell -NoProfile -File scripts/dev.ps1 -Action Inspect
powershell -NoProfile -File scripts/dev.ps1 -Action Catalog
powershell -NoProfile -File scripts/dev.ps1 -Action Smoke -Seconds 20
```

The helper selects the installed stable Windows GNU toolchain when available.
Use -Toolchain to override it and -Iso for another local path. Check requires no
game data. Inspect and Smoke require the retail ISO; they never execute PS2 code.
Smoke renders our original synthetic scene and exits after 1–120 seconds measured
from its first application update. A smoke timeout is not a gameplay benchmark.

The local helper defaults to one compilation job and debug-info level 0 after the
initial Windows compiler failure. Override with -Jobs and -DebugInfo when needed;
the helper restores the caller's environment. CI is a separate Linux source check,
not evidence of a Linux graphical run.

To prepare the prototype reference without modifying its original CUE:

```powershell
python scripts/reference_inventory.py --prepare-prototype-cue
```

The inventory goes to ignored docs/local/. The corrected CUE goes to ignored
recomp/references/. Optional --hash computes original-image fingerprints read-only.
The current local CUE names an absent file; its available BIN has a valid ISO 9660
descriptor at the expected 2048-byte sector position. This is a container check,
not proof of a running prototype or its authenticity.

## Work order and gates

| Gate | Work | Required evidence |
| --- | --- | --- |
| G1.1 | Inventory toolchains and original references | Missing/mismatched inputs reported; originals untouched |
| G1.2 | Build and smoke-test the Rust viewer | Linked binary, renderer observation, normal bounded exit |
| G1.3 | Specify generic ISO-entry and AFS bounds | Public contract, provenance and synthetic malformed cases |
| G1.4 | Implement read-only archive catalog access | No whole-archive allocation; owned-disc smoke report; tests pass |
| G1.5 | Review and publish the foundation | Accurate task state, CI, journal and file-boundary checks |

Local evidence recorded on 2026-11-03: linked Windows GNU viewer; Vulkan rendering
of original primitives; normal 45/60-second smoke exits; 14 Rust synthetic tests;
12 Python guard/inventory tests; retail catalog with 1,445 entries. The retail ISO
hash still matches the pre-native-evaluation baseline. Prototype debug access,
FST interpretation and game assets remain unvalidated/unimplemented. Use Actions
for the current revision's CI result, not this historical list.

At each gate, record the command, toolchain/version, outcome and limitation. Do not
advance merely because a file was generated. Keep detailed logs and images local.
Use docs/initialization-decisions.md for changes to the execution strategy.

## Observation → contract → implementation → comparison

1. Pick one small question: a file-table bound, a visible movement interval, a camera
   response or a transition. Identify its acceptance criterion before coding.
2. Select a reference profile and a reproducible scenario. Record the version,
   input sequence, runtime and measurement method. Check a third-party backend's
   surprising behavior against PCSX2 or a console before treating it as game behavior.
3. Write a behavioral/format contract with uncertainty and exposure disclosure.
   Use specs/contract-template.md. Never supply generated game C++ as that contract.
4. Implement a small original unit with synthetic fixtures. Keep rendering, data
   access and simulation boundaries explicit; all image/archive reads stay bounded.
5. Compare outputs. Preserve failed hypotheses and revise the contract when evidence
   warrants it. Record whether the issue belongs to the spec, engine or reference.
6. Update the journal and roadmap, run the relevant checks, review the public index,
   commit and verify CI/Pages. Avoid unrelated features during an unresolved gate.

## The prototype's role

See reference-profiles.json. Local notes report a June 2006 prototype with L2 + R3
debug access, Room Jump, collision/hitbox flags and Player Warp. These are leads to
verify, not features already exercised in this project. The saved local research
page is the current source; online retrieval was unavailable during preparation.

After verifying the debug controls in a reference emulator, use them to reach a
scene, isolate collisions or inspect camera framing. Record every debug setting:
invulnerability, warps or disabled collision can invalidate a timing comparison.

Prototype observations carry profile_id=prototype-june-2006. Retail observations
carry profile_id=retail-usa. Prototype room IDs, assets, scripts, timings, combat
rules and incomplete content must not silently become retail requirements. Confirm
each transferred assumption against retail, or label it prototype-only/unknown.
Both use Y as height; sandbox speed and its 60 Hz step remain provisional choices.

Binary diffing and internal-code research stay in the private research track. A
component's implementer must disclose such exposure and must not claim a formal
clean-room separation that has not occurred. Keep generated code out of the engine.

## Adapting the plan

For each material change, record: the failed hypothesis, evidence, chosen adjustment,
next bounded experiment and what would make us revert it. Prefer one variable at
a time. A compiler crash does not prove an engine defect; a prototype mismatch does
not prove a retail bug. Installation of needed tools is authorized inside the
repository's ignored area or G:/Tools, without changing original game files.

When G1 is complete, define G2 around one independently specified asset loaded from
the retail ISO into a Bevy scene, with a comparison protocol. Do not mark the full
roadmap or the game's recreation complete at that point.

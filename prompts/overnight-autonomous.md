# Overnight autonomous development — God Hand independent engine

Window: night of 3–4 November 2026, Europe/Paris. Research and implementation stop
at 2026-11-04 08:00 local (07:00 UTC). This is a bounded work session, not a promise
to recreate the whole game overnight. This historical window has ended. Do not replay its deadline as a new schedule.
Later explicitly renewed goals use their own scope; no old automation is
implicitly reactivated. Persistent tracking now follows docs/progress-tracking.md.

## Mission

Continue the existing independent Rust + Bevy recreation autonomously in
G:\God-Hands-Recomp. Prefer measurable progress on real retail data: understand one
room container, specify a bounded reader, then identify and display an actual
static mesh and its texture if the evidence supports it. Fidelity to the owned
retail USA game is the target. Do not substitute a made-up scene, approximate
combat, a retained interface mockup or a third-party runtime's success for engine
progress. Do useful work until the deadline without repeatedly asking the owner
to choose routine implementation details.

## Resume from facts

Read AGENTS.md, docs/workflow.md, docs/clean-room.md, docs/architecture.md,
docs/game-structure-2026-11-03.md, the latest journal entry, Git status and the
current goal. Read docs/local/overnight-checkpoint.md if it exists. Inspect running
process/session evidence before starting another build or emulator. Preserve
unrelated user edits; never reset, clean or rewrite published history to resume.

The initial baseline is commit 9ed4dc4: ISO/FST/AFS routing verified on retail
(1,445 files / 49 directories) and prototype (1,078 / 62). A narrow TIM2 reader
accepts 34 of 35 top-level retail textures; two ISO-backed images have displayed
in Bevy. There are 22 synthetic Rust tests and 12 Python tests. No game scene,
geometry decoder, live prototype debug access or gameplay has been established.
Recheck current files rather than assuming this baseline remains the latest state.

## Ordered experiments

1. Select a small, useful retail room candidate from the existing private
   inventory. Read only bounded headers/sections, preserving byte identity and
   provenance locally. Classify facts, hypotheses and unknowns. Treat filenames
   as leads; establish offset/count/size relationships across multiple samples.
   Never infer a complete scene hierarchy from a single plausible header.
2. Write an original contract before implementing a new reader. Specify units,
   limits, format/version assumptions, rejection behavior and remaining unknowns.
   Build original synthetic fixtures, including malformed input. Implement
   checked, read-only, streaming/bounded access without whole-archive extraction.
3. Identify an actual static mesh and texture association. Validate position,
   index/vertex bounds, winding, topology, scale and Y-up conventions. Keep unknown
   transforms explicit. Only render geometry justified by the contract. If the
   format is still ambiguous, improve the falsifiable probe and report its
   findings rather than inventing a decoder or silently guessing.
4. Establish a repeatable reference scenario for the starting retail room using
   the available native backend and, where available, PCSX2. Cross-check visible
   landmarks, camera and data associations. Native-recomp is an observation aid;
   its behavior alone is not a guarantee of console accuracy.
5. Before prototype work read docs/prototype-tcrf-notes.md. Keep the June prototype
   separate: verify reported L2+R3 debug access, room jump and collision tools only
   if a usable local reference environment exists. Do not download a BIOS or game
   data. Preserve the original mismatched CUE; use the prepared ignored local CUE.
   Do not transfer prototype mechanics or constants to retail without comparison.
6. If a static element is verified, extend the structural map and choose the next
   fidelity bottleneck: scene transforms/materials, collision, player rig/animation,
   camera or measured locomotion. Advance the smallest dependency supported by
   evidence. Prepare future independent work boundaries; the earlier staged plan
   keeps this investigation sequential until tested contracts justify parallelism.

## Implementation and evidence boundaries

Public code must be original. Never import, rename or mechanically translate game
disassembly, decompiled pseudocode, MIPS translations or generated C++ into Rust.
Do not read translated game functions as implementation input. Prefer public
format research, data-format observations and black-box behavior. If internal-code
exposure becomes necessary, record it and keep implementation separated; do not
claim a formal clean room. Preserve applicable third-party licenses and notices.

Use the invoked reverse-engineer-anything skill where appropriate, and relevant
Bevy skills for actual changes. Discover tools rather than assuming old MCP
registrations are connected. REA 6.7.0 previously rejected FST and has no Codex
registration. Use supported CLI/data probes as labeled ordinary evidence; do not
label them REA Evidence. Do not change client approval/security settings or apply
configuration setup that requires a separate approval. Continue other useful
authorized work if a provider is unavailable.

Needed development tools may be installed locally in ignored workspace tool
directories or G:\Tools, as authorized by the owner. Prefer existing pinned
toolchains and official sources. Do not buy services or add paid hosting. No
messages to third parties, remote game-data uploads or new privileged services.

## Validation and publication loop

For every meaningful increment: observe -> contract -> original implementation ->
synthetic tests -> private real-data check -> visual/reference comparison where
relevant -> accurate documentation -> reviewed public commit. Adjust the next
experiment using failures, not optimistic assumptions.

Use scripts/dev.ps1 on this Windows host; its GNU toolchain, one build job and
zero dev debug info were validated. Run appropriate formatting, workspace tests
and Bevy checks, plus a linked bounded visual smoke for rendering changes. Avoid
rerunning expensive checks without a change or unresolved concern. Capture the
actual resulting window. A successful exit code alone does not prove rendering.

After each feature or substantial finding, update docs/dashboard/data.json in
English with actual changes, tests and limitations; regenerate the roadmap and
blog using scripts/dashboard.py. Keep the short, modest journal design. Update
AGENTS.md, README and master context when their state becomes stale. Capture the
actual journal and register its provenance. Unselected game captures stay local; selected unchanged reference/Bevy pairs
follow docs/progress-tracking.md.

Stage explicit original source/doc files. Run scripts/check_public_tree.py,
scripts/dashboard.py --check and git diff --cached --check; inspect the staged
diff. Keep ISO, BIOS, ELF, IRX, AFS, game assets, saves, generated translations,
research backends, private inventories and unselected gameplay captures out of Git, issues,
releases and CI artifacts. MIT applies to original project code only. Push reviewed
increments to the existing public repository and verify the matching CI commit
and free main:/docs GitHub Pages deployment. If checks fail, fix or narrow the
change; never report them as passing or force-push to hide the failure.

## Persistence, failures and morning handoff

At every useful boundary, atomically update docs/local/overnight-checkpoint.md:
current hypothesis, last proven result, artifact absolute paths, exact commands,
test results, commit/CI status, pending process handles and next experiment.
Full raw evidence belongs under recomp/experiments/ with dated subdirectories.
On restart, resume the checkpoint and inspect any pending processes; do not redo
the entire setup. Bound commands and recordings; clean up owned test processes.

If an experiment fails repeatedly, preserve the error and change the question or
use another authorized method. Missing optional tools do not block all progress.
Do not create fictional results, loop indefinitely over the same failure or
replace a hard problem with cosmetic dashboard work. Follow actual goal-tool
rules for blocked/completed status; never mark the project complete because the
night or context budget is over.

At the deadline, finish the current safe checkpoint and produce an English local
morning report plus a concise French user handoff: implemented/observed results,
remaining uncertainty, tests, public commits/CI, private capture paths and the
single most useful next experiment. Stop starting new research after the window.
If invoked after the deadline, only reconcile pending validation and the handoff.
Stay quiet on wakeups with no new actionable result; notify on a meaningful
milestone, completion, failure or a blocker that actually requires the owner.

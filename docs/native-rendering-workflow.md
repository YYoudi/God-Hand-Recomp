# Autonomous native-rendering workflow

Current status, 5 November 2026: **active, explicitly resumed by the owner**.
Work in background; notify only for a substantial milestone or required intervention.

Date: 5 November 2026. Owner requested autonomous work and GitHub/journal updates.
This is an independent Rust + Bevy engine, not execution/translation of the game.
No artificial overnight deadline or completion percentage is assigned here.
No delegation unless the owner explicitly requests it.

## Goal

Starting baseline: 5d35247. Resolve the full-resource sky-overpainting failure
through controlled retail observations, then original evidence-supported rendering
behavior and synthetic CPU/GPU validation, compared at both established viewpoints.
Preserve all originals and unknowns. Keep this goal active through the remaining
GPU work; the first CPU model is not goal completion or a full scene renderer.

## Resume

Read AGENTS.md, docs/workflow.md, docs/clean-room.md, docs/architecture.md,
docs/progress-tracking.md, the active goal, Git state, docs/dashboard/data.json
and the ignored docs/local/progress-checkpoint.md. Revalidate live process handles,
not old files/PIDs. Keep unrelated changes and historical captures untouched.

## Current evidence

- Two retail views support the existing static placement hypothesis; general
  rotations/scales, hierarchy and camera motion are not established.
- All materials in the public inspector currently use host alpha blending.
- A private all-opaque control at both full-resource views restores visible
  geometry, but breaks shadows/cutouts and sky appearance. Reject as a final fix.
- Candidate buffer/state joins show blending, depth writes and alpha testing can
  coexist. Source opacity alone does not identify every observed failure action.
- specs/fragment-write-permissions-v1.md and the original fragment_alpha Rust model
  cover byte-alpha comparisons and separate RGB/framebuffer-alpha/depth permissions.
  They now drive a reusable offscreen GPU component; production retail integration
  remains pending. See ordered-alpha-gpu-2026-11-05.md. Raw traces and per-buffer
  mappings stay private.

## Execute and adjust

1. Form one falsifiable rendering hypothesis; record alternative explanations.
   Observe only data/behavior and generic independently licensed tooling. Never
   consult original game instructions, generated game C++, pseudocode or shaders.
2. Specify behavior/provenance before each implementation. Never infer per-asset
   runtime state blindly from filenames, resource ordinals or a source-alpha scan.
3. Design GPU write/discard paths from the alpha-stage contract. Keep passing
   depth-writing fragments separate from permitted framebuffer-only failures;
   failure=Keep must not write either target. Preserve byte thresholds and masks.
4. Test with original synthetic overlapping geometry, holes, fractional/opaque
   alpha, submission permutations and depth equality. Assert both RGB and stored
   alpha; distinguish GS framebuffer alpha from final presentation opacity.
   Separate arithmetic tests,
   actual GPU results and retail fidelity. No disc or game pixels in tests/CI.
5. Raw TIM2/TIM3 preservation is implemented and checked on the retail banks;
   read raw-texture-alpha-2026-11-05.md. Do not infer draw state from these bins.
   Explicitly investigate GS byte-domain blending versus host linear blending,
   texture alpha normalization/overbright loss, transparent ordering and sky
   activation. Do not silently turn a diagnostic approximation into the default.
6. Validate real full-resource views at both existing poses without moving objects
   or refitting the camera. Retain failures. A sky problem can have multiple causes.
   Never claim all resources are active or rendering fidelity from a recognizable
   image, CPU test count or a fitted geometry residual.
7. Use scripts/dev.ps1 Check/Build and bounded smokes. Record exact local commands,
   receipt hashes and actual outcomes. Do not restart a live job after observation
   timeout; follow its existing handle. Keep prototype research separate.
8. For each substantial finding/feature update docs/dashboard/data.json in English;
   generate ROADMAP/journal. Publish only selected reviewed unchanged Reference /
   Bevy originals through scripts/comparisons.py, with revision/runtime/limits.
   Before/After requires honest named iterations and unchanged camera/resources.
9. Capture the actual journal, register provenance, explicitly stage public files,
   run index privacy/drift/whitespace gates. Push main; verify matching CI and
   Pages plus live original bytes. Free Pages main:/docs and standard public
   runners only, no custom persistent caches/artifacts or paid service.
10. Update the local checkpoint at useful boundaries and adjust the next experiment.
    Notify only for meaningful results, failures, required intervention or completion.

## Completion gate

The current goal is not complete merely when depth is enabled or a CPU oracle
passes. Require evidence-supported implemented paths, synthetic GPU validation,
actual comparisons at both viewpoints, documented remaining limitations and
verified publication. Broader rendering/gameplay roadmap tasks remain independent
and must not be marked done without their complete acceptance criteria.

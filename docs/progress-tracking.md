# Progress and visual evidence protocol

Owner decision: 4 November 2026. Conversations remain French; public resources
remain English. Inspired by the Giants project's explicit progress records and
reviewed comparison gallery, not its implementation or permissions.

## Resume and checkpoint

Before work, read AGENTS.md, docs/dashboard/data.json, the current goal, Git
status/log and docs/local/progress-checkpoint.md. Read the historical overnight
checkpoint when recovering that session. Verify live process handles rather than
assuming an old job is running. Preserve unrelated edits.

docs/dashboard/data.json is the public source of truth: tasks, journal,
`progress` (current goal, last verified baseline, open questions and next
experiment) and `comparisons`. Do not create a second competing public backlog.
The ignored local checkpoint holds exact commands, evidence paths, hypotheses,
failures, process handles and pending validations. Update it at every useful
boundary, not just at the end of a chat. Checkpoint the current revision and
whether each check was run now or is a historical result.

After a feature, substantial finding, changed hypothesis or failed fidelity
comparison, record the result and adjust the next experiment. Separate:
**observed / inferred / hypothesis / unknown / implemented but unverified**.
Parsing, CPU tests, linked build, actual Bevy rendering and reference fidelity
are different gates. Mark tasks done only against their full acceptance criteria.
No global completion percentages, invented FPS, or promotional progress claims.

## Reference and Bevy comparisons

The owner's request explicitly allows selected rendered reference and Bevy
screenshots to be uploaded to this public GitHub repository and its free Pages.
This narrow exception supersedes the former journal-only screenshot policy.
It is not rights-holder permission or legal certification. MIT covers original
project code, not the game's imagery. Keep a rights/affiliation notice visible.

For each visual feature, capture the **same identifiable subject** in the retail
reference and in Bevy. Prefer comparable camera, framing, resolution, scene state
and lighting. Label all differences and missing conditions. If alignment is not
established, say so; do not present different objects as a fidelity comparison.
Do not manufacture a reference when it is missing. Record that gap and the next
capture needed. Nonvisual progress requires tests/evidence, not a fake image pair.

Publish both original, unretouched screenshot files byte-for-byte. PNG is the
normal profile. For the native backend’s P6/255 PPM output, publish that original
PPM too, alongside a clearly labeled lossless PNG viewing preview. The preparation
helper encodes the same RGB samples without resizing, cropping or color changes.
Retain the raw-original download link; never relabel the preview as an original. Keep full-resolution
images individually downloadable; a responsive side-by-side HTML view is enough.
Do not crop, repaint, AI-correct, composite away errors, or rescale the stored
originals. Do not convert an old JPEG to PNG and call it an original PNG.
If a derived annotation is ever needed, retain and clearly link both originals.
Never use an extracted texture/model sheet as a reference screenshot.

Labels are **Reference / Bevy**, not an implication that Bevy is an improvement.
If showing iteration **Before / After**, identify the two Bevy revisions and
retain the independent reference. Never silently replace the before evidence.

For each pair record in `comparisons`:

- Stable id, feature/task id, date, title and repeatable scenario.
- Alignment (`same-frame`, `same-landmark`, or `unmatched`), observations and
  explicit limitations. Matching a landmark is not pixel-perfect validation.
- Each image's public path, SHA-256, original pixel dimensions and caption.
- Capture time with timezone, renderer/runtime and exact code revision (or
  explicit `unknown` / `uncommitted`; never substitute the current commit).
- Reference profile: retail or prototype. An emulator/native backend is not a
  console capture; name the backend/version. Keep the prototype separate.
- Review confirmation: images contain only the intended game/viewer content,
  no unrelated desktop/private information, secrets, debug dumps or game code.

Private receipts retain original paths, exact launch/capture commands, source
disc identity and camera/input state. Do not export personal filesystem paths,
ISO hashes or technical dumps to the gallery. Never claim fidelity from visual
similarity alone; list geometry, placement, textures, materials, camera, lighting
and gameplay separately when relevant.

## Publication

Only manifest-registered original PNG/PPM comparisons and declared lossless previews belong in
docs/dashboard/media/. Their filenames are their SHA-256. No directory-wide
copy of private evidence is allowed. See scripts/comparisons.py for the manifest
validator and byte-preserving preparation helper. Journal captures retain their
existing separate registration under `captures`. A journal screenshot may show
thumbnails of already reviewed/registered pairs, not new unreviewed game images.

Prepare one private JSON entry with the fields listed above. For each image,
provide `source` as the absolute path to its original PNG (or reference PPM) under
recomp/experiments/, plus `caption`, `profile`, `runtime`, `revision` and
`captured_at`. The helper computes public `path`, `sha256`, `width` and `height`
from the actual file; it strips `source` from the public record. Set `reviewed`
to true only after inspecting both images. A pair requires `id`, `date`, `title`,
`feature` (a roadmap task id), `scenario`, `alignment`, nonempty `observations`
and `limits`, plus `reference` and `bevy` objects. Use an exact 40-character
revision or explicitly `unknown` / `uncommitted`.

```sh
python scripts/comparisons.py --entry docs/local/comparison-candidate.json
python scripts/dashboard.py
```

This copies just the two reviewed original files (and creates the declared
lossless preview if a reference PPM needs one), then adds their record to the
single public manifest. It never stages or pushes automatically. Existing pair
ids cannot be replaced: use a new id for a later iteration and preserve history.

Regenerate with python scripts/dashboard.py. Review the actual journal, capture
visual changes, and register that capture. Explicitly stage source, documentation,
manifest and selected screenshots. Run the public-index guard, dashboard drift
check, relevant synthetic tests and git diff --cached --check. Inspect the staged
diff; after pushing, verify the matching CI and Pages deployment.

Discs, BIOS, executables, translated/disassembled game code, extracted assets,
archives, audio, saves, states, dumps, research backends and unselected captures
remain private. Never provide screenshot game pixels as test fixtures or CI input
data; CI may validate only the approved publication files' hashes/headers.
GitHub Pages stays on main:/docs and github.io, with standard public runners and
no paid hosting or persistent CI artifacts/caches. No permissions from Giants
are imported into this project. Do not modify that repository for this workflow.

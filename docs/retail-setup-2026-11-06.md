# Personal-disc setup reproduced from a fresh workspace

Date: 6 November 2026. Project: the separate original GPL
[god-hand-pc tooling](https://github.com/YYoudi/god-hand-pc), not new Rust work.

## Why another build

The earlier tested native executable used a manually prepared research checkout.
It does not prove another owner can reproduce the full setup. The new command
fetches the pinned upstream and eight exact dependency revisions into a distinct
ignored workspace, verifies dependency license receipts and the complete personal
USA ISO, then runs the boot and 81-overlay generation/build pipeline.

The existing evaluated checkout, its working executable, original screenshots,
prototype experiment and paused Rust edits are not modified. All game-derived
outputs stay private. Only original orchestration, tests, dependency metadata and
instructions are public.

## What is actually reproduced

- Fresh network source acquisition at upstream
  `14e956a1274332440273845fbab2c1f1e30c4a5a` and eight exact dependency commits.
  Their license-file hashes agree with the earlier evaluated source receipts.
- An actual `--prepare-only` run succeeds. A distinct private ISO copy has the
  expected hash and read-only attribute; it is not a hardlink to the original.
  This copy supplies upstream's implicit sibling-disc lookup during overlay linking.
- The initial full run builds its own recompiler, generates the boot code,
  extracts and verifies all 81 overlay inputs, links them and generates their packs.
- That first run then **fails** with `unknown target 'godhand'`. Its initial Ninja
  graph had been configured before game sources existed. Merely finding an old
  `build.ninja` did not make the game target available.
- An original invented CMake/Ninja project reproduces that stale-target failure.
  Explicitly reconfiguring after generation makes its new target buildable. The
  corrected workflow now includes this second configure.
- A second entirely fresh workspace reproduces all generation gates, configures
  the game target and completes native compilation. All six steps return zero;
  the upstream source remains clean and the original ISO hash is unchanged.

The public corrected recipe is revision
`3bc0f7ae7f462dc49e1813ca0b02883999c2def2`. Its 62 original synthetic tests and
public CI pass. Tests use invented bytes/text/Git/CMake projects, never game data.
The first failed workspace and every run's original logs are retained separately.

## Completed build and actual fresh startup

The corrected run finishes at 19:00:37 UTC. Its 168,848,384-byte executable has
SHA-256 `8220c547266025a94c33f8362f2cad0549094858c786e0e2a9ea8edf637706ad`.
The local tools are Python 3.11.9, MSVC 19.44, CMake 4.4.4 and Ninja 1.13.2;
four below-normal-priority compilation jobs were used. This is a local recipe
reproduction, not a universal installer or bit-identical build claim.

Two independent, hidden, muted startup probes use this exact fresh executable:

- 100 seconds: 30 original PNGs, opening scene observed, intended deadline exit124.
- 220 seconds: 40 originals, first-fight scene reached, intended deadline exit124.

Both start with separate empty private cards, not old runtime snapshots. No
generated game C++, runtime libraries or VU1 AOT captures are imported from the
earlier research build. No cold shader-cache or real-time-performance result is
claimed. Only title/start controls are scripted, not autonomous combat completion.
An extra pause-input attempt does not produce a verified pause menu in the last
capture; this is not recorded as a successful pause/control test.

The comparison gallery retains both unchanged full-resolution originals:
earlier evaluated native field4740 at640x480 and fresh native field4741 at800x600.
All 307,200 RGB pixels in the fresh rectangle (0,120,640,480) equal the earlier
original. This is a read-only array measurement accounting for the previously
diagnosed screenshot padding, not a stored crop or a general fidelity score.
The fresh source still has that upstream readback limitation; neither original
is resized, warped or repaired. Both images are native C++ output, not Bevy or
console hardware.

The fresh baseline executable is privately backed up **before** any new runtime
adapter integration. The previously evaluated executable and all failed trials
remain preserved. This completes the fresh-build/bounded-startup roadmap gate,
not the whole continuation or a remaster.

## A zero generation exit still needs inspection

Rechecking the original tested retail log reveals one reported unhandled
instruction/error despite its successful process exit. The fresh generation also
reports one error across the boot plus 81 overlay logs, alongside 2,104 warnings.
Those logs and hashes are retained, not suppressed or rewritten.

This does not erase the earlier observed first fight, controls, texture replacement
or restoration. It also does not establish that every translated path is correct.
The orchestrator warns and records generation diagnostics independently of process
exit and linking. No guessed instruction replacement or successful-return stub is
introduced. A compiled binary remains separate from actual boot and fidelity.

## Scope and remaining checks

See [the current setup contract](https://github.com/YYoudi/god-hand-pc/blob/main/SETUP.md)
for required tools, private storage, locks and path restrictions. CMake's implicit
downloads are disabled; dependencies are acquired explicitly at pinned commits.
This is a Windows/MSVC recipe, not a cross-platform installer or bit-identical
compiler-output claim.

Next: integrate and test the public named-pack runtime bridge. This
fresh upstream build does not silently include the private texture adapter.
Real game Save/Load and later-stage stability remain open. Prototype startup's
separate kernel blocker is unchanged, with no working debug-menu claim.

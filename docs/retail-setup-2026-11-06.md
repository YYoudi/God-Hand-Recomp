# Testing the personal-disc setup from a fresh workspace

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
- A second entirely fresh workspace reproduces all generation gates and the game
  configure succeeds. Actual native compilation is running; a completed executable
  and a boot from this fresh setup are **not yet verified**.

The public corrected recipe is revision
`3bc0f7ae7f462dc49e1813ca0b02883999c2def2`. Its 62 original synthetic tests and
public CI pass. Tests use invented bytes/text/Git/CMake projects, never game data.
The first failed workspace and every run's original logs are retained separately.

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

Next: finish the same live compilation, inspect its terminal result, then run a
bounded fresh-executable boot check. Do not restart because an observer times out.
The public named-pack runtime bridge is still a separate open deliverable; this
fresh upstream build does not silently include the private texture adapter.
Real game Save/Load and later-stage stability remain open. Prototype startup's
separate kernel blocker is unchanged, with no working debug-menu claim.

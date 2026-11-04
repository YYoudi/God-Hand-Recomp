# Contributing

Read docs/clean-room.md and NOTICE.md. MIT contributions must contain original code
or explicitly identified compatible sources. State the contract's provenance, tests
and limitations in the PR. Never attach translated game code, assets, dumps, saves
or unselected gameplay captures. Reviewed original reference/Bevy screenshots
follow the narrow exception in docs/progress-tracking.md.

Use small synthetic fixtures only. Preserve read-only access, bounds and allocation
limits. Keep Bevy integration separate from GPU-free simulation.
Maintain public documentation, issues and journal entries in English.

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo check -p godhand-lab --features viewer --locked
python scripts/dashboard.py --check
```

After staging, run `python scripts/check_public_tree.py`.
Update the journal, persistent checkpoint and task status with actual results.
For visual progress, publish original same-subject Reference / Bevy pairs with
provenance and limitations; report missing reference evidence rather than inventing it.
A third-party backend result is not a result of the independent engine.

The journal uses free Pages. Do not add paid services, browser-side secrets,
large artifacts or game data to public CI.

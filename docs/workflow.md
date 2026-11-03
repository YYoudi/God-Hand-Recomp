# Independent-engine workflow

Read clean-room.md and architecture.md. Historical ghidra_analyze_retail.sh and
prompts 01–05 concern local research; their translations must not enter the engine.
Keep public resources in English and conversations with the owner in French.

The executable first-milestone plan is [G1 initialization](initialization-plan.md).
Record evidence-driven adjustments in [the decision log](initialization-decisions.md).
Keep [reference profiles](reference-profiles.json) separate throughout observations.

## Specify, then implement

Pick a task in docs/dashboard/data.json. Write its contract in specs/ with provenance
and acceptance criteria. Do not transmit game code to the implementer.

```sh
cargo test --workspace --locked
cargo check -p godhand-lab --features viewer --locked
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso"
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-textures
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --inspect-rooms
cargo run -p godhand-lab -- --iso "/path/to/God Hand (USA).iso" --room "st1/r100.dat"
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --texture "etc/subscr/face_hm00.tm2" --smoke-seconds 30
cargo run -p godhand-lab --features viewer -- --iso "/path/to/God Hand (USA).iso" --sandbox
```

Tests are synthetic only. Startup reads the ISO without modifying it, recognizes
the USA serial and does not execute the ELF. The sandbox uses original primitives only.
The separate texture view loads only a bounded supported TIM2 from the user's ISO.
Read the [data map](game-structure-2026-11-03.md) for measured progress and limits.

On this machine, use cargo +stable-x86_64-pc-windows-gnu with MinGW on PATH.
The historical MSVC path is absent. Local toolchains.lock.json pins research backends;
the independent engine uses Cargo.lock.

## Update the journal

Edit tasks and criteria in docs/dashboard/data.json. Record actual changes, checks
and limitations.

```sh
python scripts/dashboard.py
python scripts/dashboard.py --check
```

After a visual change, capture the displayed journal and record date, provenance
and caption. Gameplay images remain local. Historical French journal captures can
remain as explicitly labeled archives; new captures use the English page.
Keep static images, third-party results and independent-engine gameplay distinct.

## Publish

Explicitly stage public source, then run:

```sh
python scripts/check_public_tree.py
git diff --cached --stat
git push origin main
```

Free GitHub Pages publishes main:/docs with .nojekyll. No Sites service, paid domain
or custom deployment workflow is used. Checks run on standard public Ubuntu runners
without caches or artifact uploads. Verify CI and the published URL after pushing.

Discs, extractions, tools, Ghidra projects, translated output and gameplay captures
stay local. Before prototype/diff work, read unpublished docs/prototype-tcrf-notes.md.

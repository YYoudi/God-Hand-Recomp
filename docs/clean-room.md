# Independent implementation method

The decision is to reimplement the engine, not convert the PS2 program.
A personal ISO remains necessary as the source of game data.
This requires specifying and rebuilding logic, formats and their interpretation.

Translating MIPS to C++, then cleaning, renaming or rewriting it line by line in Rust,
does not establish an independent implementation. Do not import it into the engine.

## Observation → specification → implementation

1. **Observe** inputs, outputs and behavior using a copy owned by the observer.
   Keep raw evidence local; selected rendered comparison screenshots may be
   published under progress-tracking.md. Dumps and sensitive notes stay private.
2. **Specify** contracts in specs/: provenance, method, units, tolerances and uncertainty.
   No original code, decompiled pseudocode or function addresses.
3. **Implement** from contracts and public standards, using original code and
   synthetic tests and samples.
4. **Validate** observable results; publish conclusions or aggregate measurements
   without proprietary data.

Where a contract depends on internal game analysis, prefer separate people for
observation/specification and implementation. A session exposed to a component's
disassembly must not claim strict separation for its implementation.
Record the exposure and arrange independent review.

The project adopts a provenance protocol. It does not claim an already completed
formal clean room or legal certification. Current foundations use ISO 9660 and
original generic mechanics.

## Publication

Public: original code, fixtures created from scratch, functional contracts, original
documentation and journal captures. Original sandbox images require provenance review.

Local only: ISO, BIOS, ELF, IRX, AFS, translated game code, Ghidra projects, dumps,
saves and unselected gameplay captures. Never attach them to issues, releases or CI artifacts.

Owner decision, 2026-11-04: selected unchanged rendered reference/Bevy screenshots
may be published through the reviewed comparison manifest. This is a narrow
exception, not source-asset redistribution, rights-holder permission or a legal
certification. Game imagery is not MIT. See [the protocol](progress-tracking.md).

CI reduces disclosure risk; it does not determine code ownership.
Every contribution declares provenance and preserves notices. See NOTICE.md.

## Priority change and exposure boundary — 2026-11-05

The independent Rust engine is paused, not achieved. The owner now authorizes
separate source/build/runtime evaluation of recompilation and matching-decompilation
projects for a faithful PC port/remaster. See port-evaluation-workflow.md. Viewing
or adapting decompiled game code must be logged; affected future work cannot be
represented as independently reconstructed. Existing original Rust sources remain
separate; no third-party source is imported by this evaluation. Repository MIT/GPL
licenses do not grant rights-holder permission to the original game.

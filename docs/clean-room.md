# Independent implementation method

The decision is to reimplement the engine, not convert the PS2 program.
A personal ISO remains necessary as the source of game data.
This requires specifying and rebuilding logic, formats and their interpretation.

Translating MIPS to C++, then cleaning, renaming or rewriting it line by line in Rust,
does not establish an independent implementation. Do not import it into the engine.

## Observation → specification → implementation

1. **Observe** inputs, outputs and behavior using a copy owned by the observer.
   Keep gameplay captures, dumps and sensitive research notes local.
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
saves and gameplay captures. Never attach them to issues, releases or CI artifacts.

CI reduces disclosure risk; it does not determine code ownership.
Every contribution declares provenance and preserves notices. See NOTICE.md.

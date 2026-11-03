# Contract: <task ID and short name>

Status: draft / reviewed / validated. Version: <version>. Date: <date>.
Author/reviewer: <identity or session reference>.

## Purpose and boundary

State one observable behavior or data-format boundary. List explicit non-goals.
Identify which original engine component will consume the contract.

## Provenance and version

- Source: public standard / external observation / independently reviewed format research.
- Reference profile: retail-usa / prototype-june-2006 / synthetic-only.
- Runtime/version and scenario: <record privately where appropriate>.
- Original-code exposure declaration: <none / describe exposure and review required>.
- Transfer to retail: confirmed / prototype-only / unknown / not applicable.

## Inputs, outputs and units

Specify ranges, coordinate conventions, timing basis, state and observable outputs.
For data access, define endianness, lengths, bounds, count/allocation limits and
rejection behavior. Do not include game byte sequences or function addresses.

## Evidence, assumptions and uncertainty

Separate measured facts, reported leads, assumptions and unknowns. State tolerance,
sample count, debug settings and how reference-runtime defects are ruled out.

## Acceptance and synthetic cases

List normal, edge, truncated and malformed cases built from original synthetic data.
Define a reproducible comparison and the exact evidence needed to change status.

## Open questions and next experiment

Choose one bounded experiment. Record the result and revise this contract when
evidence contradicts it; do not silently modify behavior to match a prototype.

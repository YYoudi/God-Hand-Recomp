# First data map and fidelity milestone

Date: 2026-11-03. This is an early data-routing map, not a recovered engine
architecture or a playable scene. Public source is original MIT Rust. Full private
inventories, disc bytes and rendered game textures are excluded from Git.

## What is established

The owned USA disc has an ISO root containing a boot configuration/executable,
IOP support and the FST/AFS data pair. The independent reader now follows:

```text
User-owned ISO (read-only)
  GODHAND.FST -> bounded namespace -> exact AFS ordinal
  _DATA.AFS   -> bounded catalog   -> bounded member window
                                       TIM2 -> original decoder -> RGBA -> Bevy inspection
```

Retail: 1,445 file records, 49 directories, every AFS ordinal referenced exactly
once. Prototype: 1,078 file records, 62 directories, likewise complete coverage.
The Rust catalog example validates both separately. Prototype zero-valued child
pointers identify empty directories, not a recursive root reference. Paths can
match across profiles without identical bytes, use or gameplay behavior.

| Observed retail grouping | Files | What it suggests; what remains unknown |
| --- | ---: | --- |
| Stage-prefixed directories | 94 | Room/container candidates; contents and active room sequence unparsed |
| Player-prefixed directory | 15 | Player resource candidates; geometry/animation boundaries unknown |
| Module directory tree | 81 | Relocatable modules by signatures; no code imported or translated into Rust |
| Sound-effect grouping | 537 | Several resource formats; bank mapping and playback behavior unverified |
| Background-music grouping | 3 | An additional AFS container is present; cue behavior unknown |
| Movie grouping | 6 | MPEG-like stream prefix; decoder/timing/subtitle work remains |
| Other groups | 709 | Objects, events, UI and maps suggested by names; semantics need experiments |

These groups are disjoint and sum to the catalog count. Naming is only a lead.
There is no demonstrated room-loading graph, scene hierarchy, collision mesh,
skeleton, event interpreter or combat state machine yet. The former sandbox's
60 Hz and movement speed remain provisional and are not God Hand measurements.

## First decoded element

All 35 top-level files named as TIM2 textures have the TIM2 signature. The narrow
reader decodes 34; one multi-palette font variant is deliberately refused. This
does not cover textures embedded in other containers.

A 256x256 indexed texture was read directly from the retail ISO and displayed by
Bevy on Windows/Vulkan. Its appearance suggests an interface mockup retained in
the disc, so it is evidence for the asset pipeline only. It does not establish a
retail menu's layout, behavior or use of that texture. A second 64x64 4-bit portrait
texture was displayed separately, with transparency visible. Its use in a live
retail interface has not been verified. The viewer is an isolated orthographic, nearest-filtered,
unlit view. No placeholder scenery is counted as reconstruction progress.

The decoder follows the public [OpenKh format research](https://openkh.dev/common/tm2.html)
with original Rust and synthetic fixtures. Pixel orientation, palette-index
exchange and alpha conversion have synthetic checks; reference-runtime GS color,
alpha/blending and placement parity remain open. Unsupported layouts fail visibly.

## Evidence ledger

| ID | Method | Observation / limits |
| --- | --- | --- |
| S01 | Local read-only Python probe | FST traversal, ordinal bijection, prefix inventory; full reports private |
| S02 | Rust catalog reader, both reference images | Same file/directory counts and full ordinal coverage |
| S03 | Rust texture inspection, retail | 34 decodes, one explicit unsupported palette mode |
| S04 | Native Bevy window + local screenshot | One real ISO image visible, bounded smoke exits successfully; no in-game parity claim |
| S05 | Synthetic Rust tests | Malformed namespaces, bounded member reads, nibble/CLUT/alpha checks |
| S06 | REA 6.7.0 CLI | Doctor found Ghidra available, Codex registration missing; inspect-artifact rejected FST as unsupported |

S01–S05 are ordinary local-tool evidence, not canonical REA Evidence. No original
executable instructions, disassembly, decompilation or translated game source were
read to implement these readers. This is a provenance declaration, not legal
certification or a claim of independently separated observers and implementers.
The prototype debug menu remains a reported lead, not a verified runtime tool.

## Next experiments and work boundaries

Proceed sequentially until a useful scene contract exists:

1. Pick one retail room container; inventory section boundaries and dependencies
   with bounded reads. Keep unknown tags opaque. Produce an original synthetic
   container fixture and reject malformed offsets before decoding scene content.
2. Identify one static mesh and its texture association. Check vertex counts,
   index bounds, axes, winding, scale and position against an owned reference
   runtime. Render the measured room element before adding generic gameplay.
3. Boot the prototype separately in the reference emulator and verify the reported
   room-jump/collision tools. Record profile, hashes, inputs and captures. Recheck
   any candidate behavior on retail before transferring it into an engine contract.
4. Establish a repeatable retail starting-room scenario: input sequence, camera,
   frame basis and visible landmarks. Use native-recomp as an additional observation
   instrument; settle discrepancies with a reference emulator or console.
5. Derive collision, player rig/animation, locomotion, camera and combat contracts
   from measured scenarios. Reconstruct their order and dependencies as evidence grows.

Only then split parallel work: container/mesh ingestion, reference observation,
and simulation modules with explicit contracts and synthetic tests. Keep code-exposed
research separate from implementation and request independent review where needed.
Do not assign guessed module responsibilities from filenames alone. No subagents
were started for this milestone.

## Reproduce locally

```sh
cargo run -p godhand-lab -- --iso "/path/to/owned-retail.iso" --list
cargo run -p godhand-lab -- --iso "/path/to/owned-retail.iso" --inspect-textures
cargo run -p godhand-lab --features viewer -- --iso "/path/to/owned-retail.iso" --texture "etc/subscr/sub_mihon.tm2" --smoke-seconds 30
cargo run -p godhand-disc --example inspect_catalog -- "/path/to/owned-reference-image"
```

The last command inspects metadata only and does not authenticate an edition or
launch the engine. The game application still requires the retail USA boot serial.
Screenshots of ISO-derived images stay private; the public journal captures its
own original page only.

# Resolving a named texture without publishing game data

Date: 6 November 2026. Scope: the separate original GPL continuation tooling,
not a resumed Rust feature or a finished remaster.

## What was implemented and actually checked

[god-hand-pc](https://github.com/YYoudi/god-hand-pc) now has a bounded
`resource-plan` command. It requires the exact supported personal retail ISO,
verifies its full SHA-256 read-only, then resolves a pack's named path and explicit
container ordinals through ISO9660, FST, AFS, DAT/SCP, TM3 and TIM3 metadata.
The pack does not supply a hardcoded absolute disc offset.

The original implementation uses the lab's published format contracts as its
specification; no parser implementation or game function was copied. Forty-seven
original synthetic tests pass, including invented disc/container bytes, malformed
input, path containment, payload hashes and exclusive private plan creation.
A counted-source test checks that metadata resolution does not read texture
pixels/palettes. Full ISO identity verification necessarily reads the disc bytes
for hashing; it does not export them or feed them to CI.

On the owner's unchanged retail ISO, `st1/r100.dat`, outer SCP section **3**,
inner bank section **0**, image **0** resolves to the same 8,256-byte span as the
previous successful ground-texture experiment: 128x128, indexed 4-bit, native
stored-transfer layout. Section **0** was first tried and correctly rejected:
it is TSC, not SCP. Ordinals are container section indices, not tag occurrences.

The new named plan was mechanically converted to the already evaluated private
read-adapter's settings. A fresh, hidden, muted 100-second retail boot used an
original uniform-magenta payload, without changing the evaluated runtime source
or using a snapshot containing preloaded textures. The adapter reports applying
8,256 bytes; an actual first-fight capture shows the magenta ground. The bounded
run exits at its intended deadline with 49 original PNG captures. Only title/start
inputs were scripted, not an attempt to beat a stage. A separate original-assets
control also exits at its 100-second deadline with 49 originals. The selected
field4740 pair is now in the journal comparison gallery: all 191,360 pixels in
the top299 rows are byte-identical; 74,845 of307,200 image pixels change in the
ground region. Both unretouched 640x480 PNGs remain independently downloadable.
This verifies the named-plan bridge and restoration, not overall game fidelity.

## Boundaries

- The public command produces an ignored local **plan**, not a launched game.
  The bridge to the evaluated runtime remains private; public installation/build
  automation and runtime pack integration are separate deliverables.
- One replacement, same native pixel/CLUT size and dimensions. No HD resize,
  decoded-RGBA import, hot reload, general material API or prototype support.
- Invalid packs fail without a plan. Running the original executable without an
  override is a distinct restoration control, not an implemented automatic
  fallback/reload feature.
- No ISO, original texture, palette, generated game code, executable, dump or save
  was added to either public repository. The GPL tooling remains separate from
  the paused MIT engine. Licensing our tooling does not license the game's data.

## Prototype and interrupted validations

The host reboot interrupted the previous prototype native build and the Linux
matching build. Their old logs and zero-byte unfinished ELF/map are retained;
neither is a successful executable or a matching rebuild. Reopening the existing
Linux workspace found no surviving compiler/linker. Setup/extraction was not
silently repeated.

A previously failed prototype translation unit compiles when retried. The native
harness build was resumed with a private precompiled-header optimization, without
changing guest bytes or adding retail function bindings. Its remaining generator
diagnostic and automatic function-boundary uncertainty still matter. Object counts
are not semantic coverage, a linked PC executable, a boot or a working debug menu.
The saved HTML's debug tools remain reported leads until actually observed.

The resumed prototype build subsequently linked successfully; its actual bounded
runtime attempt stops on unsupported kernel behavior, without a validated title
or debug menu. See [the prototype feasibility result](prototype-native-feasibility-2026-11-06.md).

Next: package the reproducible retail build and connect named packs with explicit
original-assets controls; investigate the separately recorded prototype blocker.

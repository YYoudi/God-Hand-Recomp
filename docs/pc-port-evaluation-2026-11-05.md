# First PC-port evaluation and external texture proof


Later observations: [supported Linux matching environment](matching-evaluation-2026-11-05.md),
[actual input-path controls](input-evaluation-2026-11-05.md), and
[screenshot readback diagnosis](capture-readback-2026-11-06.md).
The tables below describe the initial evaluation, not a continuously revised test matrix.

Date: 2026-11-05. **Provisional direction, not a complete-game compatibility claim.**
The independent Rust/Bevy lab is paused and preserved. No new Rust feature was
started. This report concerns separate third-party C++ research builds.

Follow-up: [the matching toolchain is now being tested in Linux](matching-evaluation-2026-11-05.md).
The original Windows findings below remain historical evidence.

## Direction supported so far

Use **LasVola as the primary playable-base candidate**, with an upstream
contribution preferred for general asset-loading improvements. If maintaining
our own continuation becomes necessary, keep it in a **separate GPL project**.
Use **LucasPicoli as a targeted understanding/matching complement**, not as an
already working PC port. Keep the Rust readers, synthetic tests and comparison
workflow as potentially reusable tools; there is no reason to translate working
code into Rust merely to retain that language.

This recommendation is conditional on the persistence, presentation and wider
playthrough gates below. It is not a claim that either repository grants rights
to the original game or that the entire game has been validated.

## What was actually built

| Project | Exact upstream revision | Local observation |
| --- | --- | --- |
| [LasVola/godhand-recomp](https://github.com/LasVola/godhand-recomp) | `14e956a1274332440273845fbab2c1f1e30c4a5a` | Recompiler, boot translation, all 81 overlay packs and Windows executable built locally from the personal USA ISO. Startup, first street cinematic and first fight reproduced. |
| [LucasPicoli/god-hand-decomp](https://github.com/LucasPicoli/god-hand-decomp) | `a2c98cb5c3e5b5b3f4a8cc08a19fbfbb6499699a` | Source/build/progress audit and a bounded native-Windows tooling test performed. No local matching ELF or PC executable produced. |

LasVola's author reports playing through parts of stage 3 and comparing with a
PS2. **We did not reproduce that playthrough or hardware comparison.** Our tests
used the existing native OpenGL backend on an RTX 5080, not the optional PCSX2 GS
bridge. Audio was muted; physical keyboard/controller use was not tested.

The machine lacked the expected Visual Studio installation. A portable toolchain
was installed separately: **MSVC 19.44.35229**, toolset directory 14.44.35207,
Windows SDK 10.0.26100.0, CMake 4.4.4, Ninja 1.13.2 and Python 3.11. Builds used four
below-normal-priority jobs. The first full build took roughly forty minutes on
this shared machine; that is not a controlled performance benchmark.

Actual generated output: **10,600 boot C++ files / 183,248,356 bytes**, plus
**2,460 overlay C++ files / 37,116,382 bytes**. The native executable was
168,848,384 bytes. These are private ISO-derived outputs, not public source.
Recompilation emitted control-flow and unhandled-instruction diagnostics despite
reporting success; their absence from the observed path is not proof that all
later paths work.

The build is a **native PC executable executing statically translated game logic
with a reconstructed hardware runtime**. Generated samples retain R5900 context,
guest-memory operations and control-flow labels. That is mechanically editable
C++, not a fully recovered, high-level portable engine. The host runtime, input,
loading and rendering layers are the practical places for many modifications.

### Reproduction findings, not README assumptions

- The documented extract command placed `--iso` after `extract`; the actual parser
  requires the global option **before** the subcommand.
- Module generation assumes a sibling disc path for emission. We explicitly
  emitted with the personal ISO and the upstream `slot_for` result before module
  generation. An initial experimental heap-base mistake was rejected and its
  outputs retained separately; corrected generation passed all 81 module gates.
- The expected `vcvars64.bat` was absent. A private tool wrapper supplies the
  portable Microsoft environment; no system-wide PATH or security setting changed.
- The first runtime prepared shaders for about a minute before displaying normal
  output. We followed its live process instead of treating the black startup as
  a crash or restarting it.
- Four privately captured VU1 programs were compiled with the upstream offline
  tool into 17 additional C++ files. The AOT-enabled build completed and its
  first-fight field pacing improved markedly. A fresh introduction still logged
  another interpreted program: **this is not whole-game AOT coverage or an FPS claim**.

The executed sequence was `build.py generate`, corrected `snr2.py --iso ... extract`,
explicit ISO-aware module emission, `gen_modules.py`, `build.py modules`, configure
and build. AOT additionally uses `GODHAND_BUILD_TOOLS=ON`, `vu1_recomp` and the
retained private capture corpus. Exact commands, tool hashes, output hashes and
launch environments are retained locally. The public MIT repository does not
contain the GPL checkout, translated game code, ELF, executable or capture corpus.

## Actual runtime gates

| Gate | Result and limit |
| --- | --- |
| Disc and boot | Original USA ISO accepted; full ISO SHA-256 unchanged after the experiments. No original disc write or extraction modification. |
| Scene | Title, difficulty selection, retail opening street, Gene/Olivia and first fight rendered. Not a synthetic demo. |
| Gameplay | Enemy AI, hits, knockdown, health loss, death and Continue observed. Scripted Square pulses produced visible player attacks/impact effects and enemy reactions. No move-by-move combat equivalence claim. |
| Commands | Start/Cross title flow, cutscene skip, fresh-game pause, menu navigation and confirm/cancel reproduced through the runtime's scripted pad/FIFO interface. This is not a physical-device test. |
| Runtime snapshots | Title and first-fight snapshots saved and loaded in new processes; play progressed afterward. Some interpreted/snapshot-driven pause-input trials were inconclusive; later fresh AOT pause/navigation worked. Do not silently generalize either result. |
| Game memory card | An actual new card image was created and the in-game Load screen reported no save data, as expected. **A real gameplay save followed by load has not been reproduced.** The inter-area Map save point has not been reached; a runtime snapshot is not a substitute. |
| Presentation | Original frontbuffer PNGs are 800x600 but contain top/right black margins. Logical/physical viewport or DPI mismatch is a hypothesis, not a diagnosed cause. No cropping or correction was applied; a normal visible-window test remains open. |
| Stability | Bounded runs returned their configured time-limit code 124. This validates only the recorded paths, not later stages, audio, every movie or a full playthrough. |

## A minimal modification was reproduced

[Original-assets / external-file comparison](dashboard/#comparison-2026-11-05-lasvola-external-texture)

A small **local GPL-only extension** reads an entirely original external
**8,256-byte** payload: 128x128 indexed-four-bit uniform magenta pixels plus an
original 16-entry palette. It contains no copied game pixels or palette. Magenta
is a diagnostic marker, not a proposed remaster art direction.

The resource path is traced from the named retail room through FST/AFS, its
DAT/SCP/TM3 image window, disc loading and normal game rendering. The extension
copies only the intersection with that bounded pixel/palette window into the
successful read result. It never writes the original ISO, changes game
instructions, modifies geometry or recolors the final framebuffer.

The first implementation hooked the ISO reader and raw IOP host stream but did
**nothing** in the game. Those failed originals remain private. Actual asset reads
also pass through the **EE HLE disc-reader stream**, which bypassed both hooks.
After covering that path, the runtime logged the exact 8,256-byte intersection
and rendered the magenta ground. This is why a compiled helper or a README claim
alone was not accepted as a modification proof.

The final local research revision is
`d3a018a6952608ebfa8b22bcf9a90bcb137c20a7`, based on the pinned upstream GPL source.
It is **local-only**, not a commit published in this MIT repository. Both final
control modes use the same executable and four-program AOT configuration. The
exact installed helpers pass 561 original synthetic overlap cases and separate
malformed/configuration/file/length/budget checks; no game pixels are fixtures.

The restoration control was captured **after** the enabled trial. Global fields
3089 and 2520 were selected by three unchanged, depth-separated facade/door
regions: **29,050 pixels / 87,150 RGB samples match exactly**. No camera fitting,
object movement, image warp or stored-image crop was used. All **38,301 changed
RGB pixels** are in the visible ground band; the rest of the original frontbuffer
is unchanged. Both complete original PNGs are downloadable in the comparison.
Equal global frame numbers alone did not establish the same scene phase.

**Scope:** same-size, same-format resource replacement has been demonstrated.
Higher-resolution assets, a texture-pack identity system, hot reload, changed
resource sizes, filtering/mip policies and a remaster renderer have **not**.

## LucasPicoli: useful reconstructed C, not a ready PC engine

The checked-in report at the pinned revision states about **56.37% matched code
bytes**, **89.31% matched functions** and **54.98% complete code**. These are
**upstream report values**, not matching percentages reproduced on this machine.
A byte-identical final ELF can still carry assembled original functions and raw
data: it does not establish that all game logic has become C/C++.

The tracked source inventory contains 3,753 C/C++ files, about 8.48 MB. A limited
camera sample contains genuinely understandable fields and functions, but also
fixed offsets, 32-bit pointer assumptions and EE section attributes. The build
configuration targets PS2 compiler/assembler/linker tools and assembly fallbacks,
not a native Windows executable. These are useful comprehension inputs, not a
claim that recompiling the sources with a desktop compiler yields a port.

A fresh Windows tooling run stopped at its configured five-failure bound:
**65 passed, 5 skipped, 5 failed**. A single-file build attempt stopped at missing
generated monolithic assembly. The documented Linux/WSL2 matching environment is
absent here, so no matching ELF rebuild is claimed. Further provisioning remains
an open evaluation item, not evidence that the project cannot build on its
supported platform.

Game-derived generated/decompiled camera source was viewed in this evaluation.
The exposure is recorded locally. No such code or GPL code was copied into the
MIT engine; any future affected implementation cannot claim independent provenance.

## License and dependency boundary

LasVola declares **GPL-3.0** for its project; LucasPicoli declares **MIT** for its
authored work. Neither license is rights-holder permission for God Hand. A GPL
continuation belongs in a distinct project with the applicable source/notices;
it must not be presented as exclusively MIT. No third-party repository content
is automatically republished here. This is not a legal certification.

Fetched build dependencies were pinned from the actual local checkout:

| Component | Exact fetched revision | Declared license |
| --- | --- | --- |
| elfio | `8ae6cec5d60495822ecd57d736f66149da9b1830` | MIT |
| fmt | `407c905e45ad75fc29bf0f9bb7c5c2fd3475976f` | MIT |
| imgui | `b1bcb12a624af7509894c8e77dd47416997777fa` | MIT |
| libdwarf | `2e088b983562860f497b4fd789a3e61524eb857c` | LGPL-2.1 / per-file BSD notices |
| rabbitizer | `d5804fa488474830a47fbf9f3693048ec1e45589` | MIT |
| raylib | `c1ab645ca298a2801097931d1079b10ff7eb9df8` | zlib |
| rlimgui | `118221c8a532501a3c2c236b62950877cbefc2b4` | zlib |
| toml11 | `be08ba2be2a964edcdb3d3e3ea8d100abc26f286` | MIT |

The runtime is vendored into LasVola rather than a submodule; all of its actual
bytes are pinned by the LasVola revision. Its documented PS2Recomp/Urban Reign
ancestry is not an independently reproduced ancestry audit. Vendored PCSX2 GS
(GPL) and DirectX headers (MIT) exist, but the optional PCSX2 bridge was **off** in
these builds. SDL3 and FFmpeg options were off; the included AC5-derived movie
code carries its separate Apache-2.0 notice. Preserve per-file/component notices
before any distribution; generated game code, data and binaries stay private.

## Comparison and decision limits

| Criterion | LasVola, observed here | LucasPicoli, observed here |
| --- | --- | --- |
| Fidelity | Original translated logic reaches the retail opening/fight; no full PS2 visual, timing or combat equivalence measurement. | Matching PS2 reconstruction is the objective; no matching ELF rebuilt here. Not a demonstrated PC renderer. |
| Stability | Multiple bounded runs, inputs and cross-process snapshots work on recorded paths; later stages and game saves remain open. | Source/tooling audit only; Windows test failures and missing generated assembly block our attempted build. |
| Modification | Host-side disc-read extension changes a real loaded resource without altering guest instructions. Generated game functions remain low-level. | Some readable reconstructed C; target-specific assumptions and assembly fallbacks remain. |
| Assets/rendering | Same-size external texture proof succeeds; native renderer is accessible. HD dimensions, named packs and presentation issues still need work. | Useful resource/logic understanding, not an existing tested desktop replacement pipeline. |
| Maintenance | Existing playable runtime is a practical starting point, but hardware emulation, translation warnings and generated outputs require careful regression tests. | Matching constraints and PS2 toolchain need specialist maintenance; not a drop-in portable engine. |
| License/provenance | Distinct GPL continuation; do not merge into the MIT lab as MIT-only code. Game-derived generated material remains private. | Declared MIT for authored work is not permission for game-derived material; log decompilation exposure. |

An **upstream-first, separate GPL continuation with targeted decompilation
understanding** is the current recommendation. It is conditional, not a claim
that every requested runtime gate is passed. Reopening independent engine work
would require evidence that this route cannot meet the fidelity/modification
goals; language preference alone is not such evidence.

## Adapted next steps

1. Reproduce a genuine Map-screen gameplay save and load it in a new process.
2. Resolve the physical-window/DPI capture behavior and test keyboard/controller,
   audio and more than the first fight. Keep defects and failed controls visible.
3. Turn the narrow local data experiment into an upstreamable named-resource
   interface; do not pretend a fixed byte range is a production texture-pack API.
4. Demonstrate a genuinely higher-resolution original texture through a suitable
   host-renderer path; internal 3x rendering is not itself HD asset replacement.
5. Provision and verify the matching-decomp toolchain where feasible, then use
   specific understood components only with explicit provenance.
6. Prefer upstream work; use a separate GPL fork if needed. Preserve the MIT lab
   and do not rewrite working logic in Rust for language consistency.

The single public backlog remains [ROADMAP.md](../ROADMAP.md), generated from the
journal data. These open gates are not marked complete by this first proof.

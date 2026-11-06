# Public native pack bridge — 6 November 2026

## Decision

Continue the tested LasVola PC base through the separate original GPL tooling
[god-hand-pc](https://github.com/YYoudi/god-hand-pc), upstream-first, not another
Rust rewrite and not an automatic upstream mirror. Preserve the MIT lab unchanged.
LucasPicoli remains a comprehension/matching reference, not a demonstrated PC port.
Its provisioned Linux compiler smoke tests and sample object passed; the interrupted
full ELF/map remain zero-byte outputs, so no matching rebuild is claimed.
See the [evaluation](pc-port-evaluation-2026-11-05.md) and
[prototype result](prototype-native-feasibility-2026-11-06.md).

## What was actually reproduced

The [fresh personal-disc build](retail-setup-2026-11-06.md) was preserved before
relinking. The public installer checks exact pinned host-file preimages, creates a
private derived source revision and builds the original bridge. It edits five
host/build integration paths and adds four original bridge files, not generated
game functions or guest instructions. All nine changed paths and the binary are
checked before launch; unrelated source/binary identity failures refuse execution.

| Evidence | Exact result |
| --- | --- |
| Upstream | `14e956a1274332440273845fbab2c1f1e30c4a5a` |
| Launcher used for the native runs | `8b70389c93bb7183529a1fd7cea6e8e92c11a8e1` |
| Private derived runtime source | `193076584e277c95644dcbbb4a6922c3a59ca2b8` |
| Native adapter fingerprint | `f7a48919c4ad38ce1b7d4591c68783e701f27c7937395f4caccd4173d9667824` |
| Native executable SHA-256 | `eaed174f4d0c1df5530e5a65b2d53611a14d3037c9e01a3e0aa02c8d6a0d4e53` |
| Install configure/build | Both exit 0, baseline executable backed up before relink |
| Enabled / disabled / invalid-pack | Three independent hidden 220-second runs, each expected deadline exit 124 |
| Synthetic tooling | 81 tests; no game bytes in tests or CI |

The original retail ISO SHA-256 remains
`268b8e284619bb9e07d5c522d7e7cfa14cb0498623d7fba04a97d18b63bc0a7c`.
It is verified read-only before and after runs. No old runtime state, private
capture fix, VU1 AOT capture or prototype binding is imported into this build.

### Resource to rendering

The pack names `st1/r100.dat`, outer SCP ordinal **3**, bank **0**, image **0**.
The original metadata planner resolves ISO9660 → FST/AFS → DAT/SCP → TM3/TIM3,
checks the retail profile and accepts only the supported same-size stored bytes.
No absolute disc offset belongs in a user pack. The launcher freezes the original
validated payload per run; the native bridge checks its exact size/digest and
applies overlaps to read buffers. The normal guest resource loader and renderer
then consume those bytes. CD-image, positively identified raw-ISO IOP host-stream
and EE sector-fallback paths are covered; the actual enabled run logs application
through the EE sector reader. No rendered-image repainting is involved.

The original test file contains uniform magenta indexed pixels and an authored
palette: **8,256 bytes**, SHA-256
`fe467079e9695b5832af4b79a383db7ae76bc704ab439c2e481ceb65a543c357`.
The public `make_example_pack.py` generates those exact bytes without reading a
game asset. This is a diagnostic, not remaster art or an extracted texture.

### Controls and original evidence

All runs use the same bounded title/start inputs and fresh private cards, no
combat-completion script. Selected field **4741** has the same camera/actors/HUD.
Enabled versus disabled changes **74,845 pixels** in the ground region; the top
**419 rows** are equal. These are read-only measurements, not a fidelity score.
The full original **800x600** PNGs retain the known upstream black readback padding;
no crop, resize, warp, overlay or retouching is published.

- Pack enabled: original diagnostic ground is visibly magenta.
- Pack omitted: selected original PNG is byte-identical to the preserved
  pre-adapter fresh baseline (`fd53182e…`).
- Deliberately invalid payload digest: launcher warns and uses original assets;
  the selected original PNG is byte-identical to that disabled control.
- The same invalid pack with `--strict-pack`: actual exit **1**, before native launch.
- Wrong prototype disc: actual exit **1**, before native launch; not a fallback
  around the ISO requirement.

Both reviewed full-resolution originals are in the
[journal comparison](https://yyoudi.github.io/god-hand-rs/dashboard/#comparisons).
They are native C++ output, not Bevy, PS2 hardware or PCSX2.

## Reproduction boundary

Use the separate GPL repository's [setup](https://github.com/YYoudi/god-hand-pc/blob/main/SETUP.md)
and [runtime contract](https://github.com/YYoudi/god-hand-pc/blob/main/RUNTIME_PACKS.md):

```powershell
python pc.py setup --iso "G:\path\God Hand (USA).iso" --vsroot "G:\your-msvc-root"
python pc.py install-packs --workspace-name retail-setup
python make_example_pack.py --output .local/example-pack
python pc.py run --workspace-name retail-setup --iso "G:\path\God Hand (USA).iso" --pack .local/example-pack/pack.json
```

Run without `--pack` for original assets. Installer/source/binary/ISO checks are
not silently ignored when a pack is invalid. Native read failures leave original
read-buffer bytes unchanged; strict mode concerns prelaunch pack validation.
An installed derived workspace is not a pristine setup workspace: use a new name
for another fresh build. Do not overwrite old failed attempts or original discs.

## Remaining roadmap, in order

1. **Persistence and usability:** owner-led manual controller/audio checks and a
   genuine in-game save then load in a new process. Use a legitimate owner save
   or a focused inter-area state; do not make development depend on beating levels.
2. **Broader resource validation:** another named resource/scene with restoration
   controls before expanding the accepted storage-format subset or pack count.
3. **HD asset path:** specify a decoded original-texture import and size-aware
   upload boundary; prove larger dimensions with an invented asset and same-view
   controls. Same-size byte replacement does not prove HD support.
4. **Rendering/stability:** collect targeted original/reference pairs and bounded
   regression states, retain defects; resolve diagnostics rather than guessing NOPs
   or successful returns. No full-playthrough, performance or hardware-fidelity claim.
5. **Prototype:** separately investigate the reproduced unsupported kernel syscall
   **0x7a** against licensed primary SDK/runtime ABI with synthetic tests. The linked
   native prototype has no observed title/frame/debug menu. Do not reuse retail
   function maps or claim the reported debug shortcut already works.

Generation still reports one unresolved error. Physical controls, audible audio,
real game Save/Load, HD resizing, hot reload, later stages and complete fidelity
remain open. Stored workspace cards and runtime snapshots are not a verified
in-game save round trip. The paused independent-engine rendering objective is
**not achieved** by this separate native continuation.

Only original GPL tooling/docs are in god-hand-pc; no ISO, ELF, generated game code,
extracted asset, dump, save or executable is published. MIT covers the independent
lab's original source, not this continuation or game imagery. A repository license
is not rights-holder permission, and this is not a legal certification.

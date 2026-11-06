# Prototype native feasibility: linked, but startup is blocked

Date: 6 November 2026. This is a separate private experiment, not a retail port
or a working debug-menu demonstration. Original prototype files remain untouched.

## Input and generation

Read the owner's saved June-build explanation HTML under `GH-Prototype`. Its
reported L2 + R3 debug menu, room jump and player warp are leads, not locally
verified functionality.

The actual June21-named MODE1/2048 image boots `SLPM_665.50`, entry `0x100008`.
The disc SHA-256 is
`5871f717ea7f3239cbc75e49790d5202c0b5041c392d1317e616ee1d344ceb7e`;
the original boot ELF SHA-256 is
`f9974f97a7c38a6f1d0fcdd66e0f9424bbe109ecca745493628b468d8ce5242e`.
The original CUE names a different June22 file; neither CUE nor BIN was edited.

Initial generic generation exited zero but reported 2,998 diagnostics. A separate
analysis copy changes only non-text executable section flags, outside every load
segment: all mapped guest bytes remain identical. This produces 9,791 discovered
functions with one unhandled-instruction diagnostic and boundary warnings still
open. A zero generation status is not a clean translation.

## Actual build and runtime result

Two sampled generated objects compiled. The first full native build was
interrupted around the host reboot; its exact parent exit is unknown. Its failed
translation-unit command succeeds when retried. Original logs are retained.

The resumed MSVC19.44/CMake/Ninja build, with an original generic harness and
private precompiled headers, **links successfully**, exit0. The 121,732,608-byte
PC executable has SHA-256
`2c41f647b71077459863fcc19710eaed80c16202cba1af167cb5393d7aa6a406`.
This is compilation evidence, not semantic validation of discovered functions.

The harness links the pinned generic runtime/IOP/graphics libraries from private
revision `1646d8854e1c3e16e5ec9bb7aeddebe0fe0fa14a`. It does **not** link retail
generated game libraries, use retail function maps, install retail game HLE
addresses or reuse retail room-module slots. Its runtime input is the original
prototype ELF and BIN, with a separate private card directory.

An actual hidden, muted, 60-second native run mounts the prototype image,
initializes OpenGL and loads the original ELF. It repeatedly reports an
unimplemented kernel syscall, identifier **0x7a**. The stop watchdog requests
shutdown and the harness returns0; this is **not successful game startup**.
No scheduled framebuffer PNG is produced, no title/scene is validated and the
debug-menu combination cannot meaningfully be tested at this stage.

## What this establishes, and next work

- Generation and a linked PC executable were reproduced locally, not inferred
  from an author's README. The first observed runtime blocker is recorded.
- Correct prototype kernel/runtime support, automatic function-boundary accuracy,
  the remaining generator diagnostic, module loading and subsequent startup are
  still open. Fixing the first syscall may expose more blockers.
- Do not return a guessed constant, install retail addresses or suppress warnings
  merely to advance execution. Investigate the generic kernel ABI against
  appropriately licensed primary SDK/runtime sources, with original tests.
- Once an actual title/scene runs, attempt the reported debug menu with bounded
  controls. Prototype shortcuts cannot validate retail progression or Save/Load.
- No prototype source translation, executable, ELF, asset, dump or original disc
  is published. No new independent Rust feature was implemented.

The practical PC continuation remains the tested retail base in the separate
[god-hand-pc tooling project](https://github.com/YYoudi/god-hand-pc). Prototype
research is potentially useful for inspection, not a replacement for that base.

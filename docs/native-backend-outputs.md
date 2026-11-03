# What ps2-native-recomp produces

This describes files observed in our local trial at commit
cba42f57913d181b3ff2fd3064ca48ab0cbeea11. Paths below are relative to the ignored
backend checkout under recomp/backends/ps2-native-recomp/.

## Build and runtime outputs

| Output | Local location | Contents |
| --- | --- | --- |
| Extracted inputs | games/godhand/disc/ and modules/ | Boot ELF and relocatable modules from the personal ISO |
| Translated EE code | generated/godhand/ | C++ units, supporting declarations and registration table; 265 units emitted in our build |
| Compiled game archive | build/native-godhand/libps2game.a | Native object code compiled from the translated game |
| Runtime library | build/runtime-native-*/libps2runtime.a | Console services and hardware implementation |
| Windows runner | build/native-godhand/ps2run.exe | Translated game linked with the runtime; dependency DLLs alongside it |
| Recorded VU programs and compiled cache | games/godhand/vu1/ and vu1-live/ | Captured vector programs and native libraries; 31 additional programs compiled in our first windowed run |
| EE live cache, when needed | games/godhand/ee-live/ | Native libraries for newly discovered code; no EE-window translation reported in this trial |
| Test artifacts | Local experiment directory | Frame images, raw logs, memory snapshots, input schedules and timing metadata |

Generated C++ preserves machine-level behavior. It is not the original game source
and does not supply documented high-level player, combat or level systems ready
to transplant into Bevy.

## How it can help the independent engine

- **Executable observations:** replay a known sequence, inspect visible behavior
  and document timings before the whole independent engine exists.
- **Repeatable experiments:** save a runtime state and apply controlled inputs.
  State files remain private and backend/version-specific.
- **Comparison:** compare our engine's outputs with observations. Cross-check against
  PCSX2 or a console; the experimental native backend can have its own bugs.
- **Generic tools:** ISO access, capture and replay infrastructure may help after
  source and license review. MIT tooling requires attribution and does not license
  translated game code.

These benefits do not automatically provide independent specifications of AFS/FST,
models, animation, scripts, AI or combat. Reaching gameplay in the backend does
not implement those systems in Rust/Bevy.

## Provenance boundary

The backend's original MIT source and code/data generated from the user's game
are different categories. Keep translated C++, linked game binaries, raw modules,
VU program copies, memory dumps and gameplay captures out of the public repository.

Do not convert generated C++ to Rust line by line and call it clean room.
An implementer under strict separation should receive reviewed behavioral contracts
rather than that code. Preserve the boundary in [the method](clean-room.md).

For rapid playable recompilation, the runner is already useful locally.
For our selected independent public engine, its main role is observation and
validation. Adopting a different engine architecture would be a separate scope change.

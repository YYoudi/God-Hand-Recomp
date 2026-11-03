# First local ps2-native-recomp trial

Date: 3 November 2026. This evaluates the third-party
[ps2-native-recomp](https://github.com/exoteric33/ps2-native-recomp) backend.
**It is not a result of the independent Rust/Bevy engine.**

## Observed result

The local build boots God Hand USA, displays the title, starts a new game, plays
the opening cinematic and reaches the first fight. Enemies are active and the HUD
is present. A runtime state saved during that fight can be loaded in a new process;
the fight continues afterward.

This confirms that the backend merits testing with our copy of the game.
It validates neither the casino reached by another tester nor a full playthrough.

## Conditions

- Unmodified backend at cba42f57913d181b3ff2fd3064ca48ab0cbeea11.
- Locally provided USA disc, boot SLUS_215.03, patch identifier 6FB69282.
- No PNACH patches, aspect-ratio changes or game-specific fixes applied.
- Windows 11, Core i7-14700KF, RTX 5080, 32 GB RAM.
- Direct3D 12 with DXC, default internal resolution.
- Portable llvm-mingw, CMake and Ninja; dependencies verified with pinned upstream checksums.

The first build inherited an external ccache path interpreted incorrectly under
MinGit. It was rerun with an isolated PATH and ccache disabled, after clearing that
option in the local CMake cache. No backend source changes were needed.

## Executed cases

| Case | Observation | Limitation |
| --- | --- | --- |
| Build | Translators, runtime and runner compiled; 265 translation units emitted | Translated code stays private |
| Automatic exploration | 1,800 frames, normal exit; title visible in a capture | A final black frame alone did not establish the boot outcome |
| Windowed run with captures | 7,200 frames in 122.44 seconds total; cinematic and first fight observed | Captures select one-thread mode in this version |
| Normal window, VU cache populated | 4,800 frames in 81.51 seconds; state saved at frame 4,500; normal exit | Includes boot, menus and cinematic; not a combat benchmark |
| Resume and scripted input | Loaded frame 4,500, continued to 6,300; new state saved at 6,200 | Scripted analog/buttons only; physical keyboard/controller not validated |

Resume captures show changed orientation/framing after analog input, combat
exchanges and changing health bars. This does not measure input or combat accuracy
against the console.

The first windowed session compiled 31 additional VU programs. The next boot loaded
them from cache. The tested path uses compilation during execution; it does not
establish complete offline AOT coverage.

## Performance and limitations

Observed title-bar readings during the opening were around 60 FPS, with one
28.6 FPS reading during a transition. These are instantaneous observations, not a
frametime trace or controlled benchmark. Total times include startup and do not
establish stable performance throughout the game. No PCSX2 comparison was performed.

The normal-window audio log reports one queue underrun, about 0.60 seconds of silence
in total, and no dropped samples. This does not replace comparative listening.
Repeated SIF warnings occur despite progression; their effect later is unknown.

Pause is not validated. The tested save is a **runtime snapshot, not the game's native
memory-card save**: no card-page writes were recorded.

Two Start-input trials on the restored state did not clearly identify the pause menu.
Later captures show a transition and continuation text after damage.
The test is inconclusive; it does not establish a menu hang or rendering defect.

## Implications

The backend is locally verified through the first fight. It supplies an executable
reference and an experimental way to play. It does not implement an original engine
or change the public Rust/Bevy project's MIT scope. Do not copy its translations.
See [outputs and reuse boundaries](native-backend-outputs.md).

Useful next tests: pause and menus, physical input, native saves/loads, a longer
session, then a matched PCSX2 comparison. Passing one does not validate the others.

## Data kept local

The ISO, extracted modules, generated C++, linked runner, memory states, raw logs
and gameplay images remain excluded from Git. Only this written report is public.
Public captures show the development journal.

The ISO's SHA-256 matched before and after the trials: the source disc was unchanged.

# A screenshot-size bug, not a demonstrated rendering defect

Date: 2026-11-06. This is a separate native-runtime evaluation, not Rust/Bevy work.

## What changed in our understanding

Earlier native PNGs were 800 × 600 with black padding above and to the right
of a 640 × 480 content region. A brief visible-window observation showed the
menu filling its client area. The owner interrupted desktop control before
our keyboard-navigation action; that keyboard test is **not passed**. No
further desktop interaction was attempted after the interruption.

An original, game-free hidden-window probe reproduced the padding using the
same built raylib library. This gives a narrower, experimentally supported
explanation: the screenshot readback dimensions were wrong on this setup.
It does not establish that every presentation, DPI or resizing issue is fixed.

## Synthetic experiment actually run

The original probe draws a solid field with distinct corner markers, then
reads the framebuffer directly. It contains no game pixels, assets or logic.
On this Windows/NVIDIA setup the reported desktop scale is 1.25:

| Window configuration | Logical size | Actual render size | Default screenshot | Render-size readback |
| --- | --- | --- | --- | --- |
| Normal, high-DPI rendering disabled | 640 × 480 | 640 × 480 | 800 × 600; content bounds (0,120)–(640,600) | 640 × 480, full content |
| High-DPI rendering enabled | 640 × 480 | 800 × 600 | 800 × 600, full content | 800 × 600, byte-identical |

The installed helper was compiled into the probe and both cases passed.
A longer normal-window control reproduced the same result. Viewport dimensions
were **not queried**; these findings use reported render dimensions and original
pixel readbacks, not an invented GL-viewport measurement.

The inspected [raylib 5.5 readback implementation](https://github.com/raysan5/raylib/blob/5.5/src/rtextures.c)
multiplies logical dimensions by desktop DPI scale. That scale is not necessarily
the allocated framebuffer scale when high-DPI rendering is disabled. Our local
GPL-only runtime change uses actual render dimensions for direct framebuffer
readback. It does not resize, crop or repair an existing PNG, alter the game,
or change the renderer's output dimensions.

## Native control actually reproduced

The private runtime patch is revision
`1646d8854e1c3e16e5ec9bb7aeddebe0fe0fa14a`, based on the earlier input-observation
branch. The incremental native build completed successfully. A new bounded
15-second hidden trial restored the same verified paused retail snapshot and
exited normally at its deadline. New original captures are 640 × 480.

Seven of eight common named captures have exactly the same pixels as the content
region of the earlier originals, including six consecutive settled menu captures.
One early capture differs at 7,110 pixels and is retained; the result is **not**
a full noninterference, gameplay or console-fidelity proof. Region comparison
was read-only array analysis; no derived game image was written or published.
All earlier original screenshots and the published texture comparison remain
unchanged, including their historical padding and provenance.

No third-party source, executable, game input or extracted asset is added to
the MIT repository. The patch, installed-helper tests, original synthetic probe,
commands, hashes and retail control originals remain in the separate private
research workspace. Baseline/AOT and earlier texture-test binaries remain
preserved. The immediately preceding diagnostic binary was rebuilt in place;
its exact old executable was not separately backed up, although its source
revision and run hashes remain recorded. Future relinks must save the executable
before launching the build, not after linking has begun.

## Other evaluation gates stay open

Normal retail inputs have progressed into the saloon and defeated a surviving
regular enemy, after which a purple demon appeared. Later combat attempts failed;
this is not an area-clear or playthrough result. Runtime snapshots are useful
research checkpoints, not genuine game saves. The inter-area Map save/load
round trip is still unverified. The current runtime uses directory-backed card
roots; importing a different backend's raw-card filename would be incorrect.

The original Linux matching setup is still running. Its two-hour host observation
timeout occurred, but the existing remote build process was checked alive after
that timeout and was **not restarted**. Object-file counts are not matching C
coverage or evidence of an identical ELF. Physical gamepad, keyboard-navigation
round trip, audible audio, broader stability and HD asset dimensions remain open.

The provisional direction remains upstream-first LasVola or a separate GPL
continuation, with LucasPicoli as a separately evaluated comprehension tool.
The independent MIT lab and its unachieved rendering work remain paused.

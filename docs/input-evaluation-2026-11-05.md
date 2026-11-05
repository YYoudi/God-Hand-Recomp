# Following the input path before changing it

Date: 2026-11-05. Separate LasVola GPL research build; the MIT Rust lab remains paused.

Some short scripted inputs and restored-state trials did not produce the expected
view. We did not treat that alone as proof that controller emulation was broken.
A small original, opt-in diagnostic now records the input clock, frontend state
and actual pad replies. It changes no guest instructions, button behavior,
health, collision, camera parameters or scene progression.

## Observed path and controls

The older EE HLE pad-backend callback was not used in this retail trial. The
actual path went through the IOP SIO2 / emulated DualShock reply. The failed
first observation point is retained; inspecting a plausible source function was
not accepted as evidence that the game called it.

With one restored retail state and a repeatable script:

- The frontend recorded exactly 1, 12 and 120 L1-active fields for the three
  requested durations. The input clock continued after the runtime-state load.
- The actual IOP reply carried the expected active-low L1 button bytes and
  DualShock pressure-mode identifier. Event deduplication means these logs do
  **not** independently count delivery of every repeated identical press.
- Dense original captures show a view change after a longer press and another
  later view change. An end frame facing a similar wall was insufficient to
  conclude that the button had been ignored. Walking, backing away, horizontal
  movement and pause were also observed through the normal input interface.
- Tracing on/off used the **same executable and script**. Of 56 common named
  captures, **54 were byte-identical**, including the late paused controls.
  The two transient differences remain retained; this is not an all-frame
  noninterference or deterministic-timing claim. Disabled mode created no trace file.

The final private diagnostic revision is
`0145bd40192d0e5af7b1c03eb13d398fef15a565`, based on the previously evaluated GPL
branch. Its executable SHA-256 is
`1f70babb48371dc9efcce6eb72889aa6ec8fa5b0dc15bed31beabc13e83dbb21`.
The earlier executable and source revisions are preserved separately. The
installed logger passes four original synthetic cases: independent-source
deduplication, an 8,192-row limit, disabled/no-output and refusal to overwrite an
existing file. No game pixels, guest code or disc data are test fixtures.

## What this does not prove

This narrows the earlier failure hypotheses; it is **not a controller fix**.
Neither the exact original turn angle/timing nor physical keyboard/controller
behavior has been compared with a console. The single-field trial alone does
not establish how the original game samples short presses. No source patch was
made to guessed clock, button, pressure or camera semantics.

Natural retail play reached a 200G state and continued through attacks,
movement and menus. Navigation and collisions remain under test; no inter-area
save screen has been reached. The pause-menu world map is not that save screen.
A genuine gameplay memory-card save followed by a new-process load remains an
**open gate**. Runtime snapshots used to resume these tests are not substitutes.

All original gameplay captures, raw traces, snapshots, private GPL source and
binaries remain local. No artificial demonstration scene or prototype behavior
was substituted. The public English journal records the findings without
publishing those materials or importing GPL/game-derived source into the MIT lab.

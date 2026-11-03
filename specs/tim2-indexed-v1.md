# Single-picture indexed TIM2, version 1

Date: 2026-11-03. Status: narrow decoder; renderer fidelity remains to be compared
with a reference runtime. Source: [OpenKh's format research](https://openkh.dev/common/tm2.html),
plus header observations from owned retail data. Original implementation and
synthetic fixtures; no game executable or original texture-decoder code consulted.

## Supported boundary

Little-endian version 4, alignment selector zero, one picture. File header 16
bytes; picture header 48 bytes. Picture format zero, mipmap count one. Index type
4 (4-bit, low nibble first) or 5 (8-bit); palette type 3 (RGBA32, CSM1). Exactly
16 or 256 palette entries respectively. CSM1 8-bit palette exchanges index bits
3 and 4 within each block of 32 colors; 4-bit palette remains sequential. Only
linear index bytes with exact width*height storage are supported. No implicit
unswizzling of arbitrary GS memory. Width/height nonzero, maximum 4096 each,
decoded pixel count at most 4,194,304; input at most 16 MiB.

Header/image/palette sizes must fit the picture total and file, without overlap.
PS2 alpha is converted to u8 by min(alpha*255/128, 255), rounding half upward.
Return row-major straight RGBA, top row first. Preserve dimensions and pixels;
the viewer uses nearest sampling and an unlit material, with no color grading.
This is an inspection view, not reconstruction of a menu's layout or blending.

Reject direct-color, multiple-picture, mipmapped, nonstandard-header, alternate
palette-bank/storage formats, size mismatches, zero/oversized dimensions and
truncation with an explicit unsupported/invalid error. Do not guess missing data.

## Acceptance and limits

Synthetic tests must distinguish low/high nibbles, the 8/16 palette-index exchange,
half/full alpha and malformed layouts. Retail contains 35 top-level TIM2 signatures;
the multi-palette font variant is deliberately outside this boundary. Decode and
inspect a small named UI texture locally through ISO -> FST -> AFS -> TIM2 -> Bevy.
Visual recognizability does not establish pixel-perfect parity with in-game GS
blending. Reference-runtime comparison and other formats remain separate tasks.

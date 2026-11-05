# TM3 bank and indexed TIM3, version 1

Date: 2026-11-03. Provenance: owned retail bank/header observations and independent
[Chihuahua TM3 pattern](https://github.com/anasrar/chihuahua/blob/d27b348818163342f8c1f10841415b4e5240dbc1/pkg/tm3/tm3.hexpat),
TIM3 research and GS storage mapping. No original game code consulted. The direct
bit-coordinate decoder is original; independent MIT table-based tooling is a
differential reference, not a copied game decoder.

## Bank boundary

Uppercase TM3 signature with zero terminator, u32 count, eight opaque bytes;
count u32 offsets at byte 16. For odd count, skip four padding bytes, then count
eight-byte raw names. Offsets are relative to bank origin, strictly increasing,
16-byte aligned and beyond all metadata. Each image window ends at the next
offset or bank end. Limit count to 4,096, require unchanged backing length and
reject truncated/outside/overlapping metadata. Names are not output paths.

## Image profile

TIM3 signature, version 4, one picture. The common single-picture indexed layout
and palette checks are shared with specs/tim2-indexed-v1.md. Layout selector 0
means linear bytes; selector 6 selects the measured GS transfer profile. Require
power-of-two dimensions at least 128 for selector 6. Other selectors are refused.

For selector 6, image bytes represent a PSMCT32 transfer with width W/2 and height
H/4 (4-bit) or H/2 (8-bit), then indexed PSMT4/PSMT8 reads at W,H. Derive the
source index by composing these address maps, not by treating the image as linear.
PSMCT32 pages are 64x32 pixels, PSMT4 pages 128x128, PSMT8 pages 128x64;
each is 8 KiB. Indexed pages per row are W/128. Check every computed source index.

Within a page, the low-to-high nibble-address bits for PSMT4 are:
Y1,X3,X4,X0,Y0,X1,(X2 xor Y1 xor Y2),Y2,Y3,Y4,X5,Y5,X6,Y6.
The PSMT8 byte-address bits are:
Y1,X3,X0,Y0,X1,(X2 xor Y1 xor Y2),Y2,Y3,X4,Y4,X5,Y5,X6.
Invert the PSMCT32 byte map to its raster coordinate and byte lane. Its page bits
2..12 are X0,Y0,X1,X2,Y1,Y2,X3,Y3,X4,Y4,X5; bits 0..1 select the byte lane.
All axes refer to pixel coordinates, not a model transform. Palette/index/alpha
conversion follows the narrow TIM2 contract. Unsupported palettes/mipmaps fail.

## Acceptance and fidelity limits

Test bank bounds/ordinals, linear 4/8-bit images, stored-layout rejection and
swizzled coordinate samples plus full synthetic-image checks against an independent
table-based reference. Measure all images in the three selected room banks.
This proves texture decoding and resource association, not the game's lighting,
alpha-test/blending, mipmap selection or camera. Actual pixels remain private.

## Explicit raw-alpha API, 5 November 2026

TIM3 now shares the distinct raw expansion API with TIM2. Transfer/palette
addressing is unchanged; decode keeps normalized presentation alpha, decode_raw
preserves all four palette bytes. See raw-texture-alpha-v1.md.

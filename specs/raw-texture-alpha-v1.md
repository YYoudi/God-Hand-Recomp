# Raw indexed texture alpha, version 1

Date: 5 November 2026. Status: implemented; storage and synthetic component checks, not runtime mapping.
Prerequisites: tim2-indexed-v1.md, tm3-tim3-v1.md, ordered-alpha-gpu-v1.md.
Provenance: existing independently implemented indexed readers and synthetic
palette data. No new format interpretation, game program or shader input.

## Preserve, do not invert

Add a distinct RawRgbaImage type and explicit TIM2/TIM3 decode_raw APIs. Expand
indices and CSM1/transfer addressing exactly as before, but copy all four palette
bytes unchanged. A raw alpha of128 remains128;129..255 must not be clamped away.
Return row-major RGBA and original dimensions under all existing input limits.
The existing RgbaImage and decode APIs remain presentation-normalized: conversion
is min((raw_alpha*255+64)/128,255), RGB unchanged. Implement that conversion once,
consuming the raw image; never attempt to recover raw alpha from normalized data.
Do not silently change existing viewer assets or change a field's meaning.

The GPU adapter may consume RawRgbaImage explicitly, without alpha conversion.
Its synthetic exhaustive alpha ramp must also exercise the decoder -> typed
adapter path. A presentation image is not interchangeable with a raw image.

## Acceptance

Original synthetic tests must cover all256 palette-alpha values, CSM1 ordering,
4-bit nibbles, linear TIM2/TIM3, selector6 transfer addressing and malformed-input
rejection for both APIs. Existing normalized decoder fixtures must remain exact.
Check the byte-for-byte RGB/presentation relation, not only alpha min/max. CI
uses only original synthetic inputs; no game pixels or runtime data.

Add an opt-in read-only alpha inspector: own ISO -> validated FST/AFS -> exact
caller-selected DAT/SCP/TM3 bank. Require unique section tags, refuse ambiguity.
Process at most16 requested room paths,512 bank images and64Mi decoded pixels;
existing per-file bounds still apply. Compare raw expansion against the existing
normalized API and report aggregate alpha bins/threshold differences only. Never
write assets or infer active materials, blend/failure states or scene visibility.
Fail on any mismatch; no partial-success claim. Do not publish raw histograms or
per-texture data as fixtures. Verify the original ISO remains unchanged.

Local retail checks should cover the three established candidates. These prove
storage-byte preservation and legacy-output compatibility, not correctness of
runtime texture binding, GS blend behavior or a fixed full-resource game view.
No new reference pair is justified without an actual rendered integration.

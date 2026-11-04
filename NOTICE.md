# Scope and dependencies

MIT covers only this repository's original code and documentation.
God Hand and its programs, images, music, text and trademarks belong to their
respective rights holders. No license to those materials is granted here.
This independent project is unaffiliated with Capcom, Clover Studio or Sony.

This repository distributes an early independent engine, not the game or a playable
port. Each user provides a USA disc image (SLUS_215.03). Startup checks for it;
tests use synthetic fixtures without a disc. No game or BIOS download, decryption
or copy-protection bypass is provided.

## Dependencies obtained separately

| Project | Observed license | Use |
| --- | --- | --- |
| [Rust](https://www.rust-lang.org/) and [Bevy](https://bevy.org/) | MIT / Apache-2.0 | Toolchain and ECS; dependencies fetched by Cargo |
| [ps2-native-recomp](https://github.com/exoteric33/ps2-native-recomp) | MIT | Ignored local research reference, not an engine dependency |
| [ps2-web-recomp](https://github.com/snuri00/ps2-web-recomp) | MIT | Origin of the native backend family |
| [PS2Recomp](https://github.com/ran-j/PS2Recomp) | GPL-3.0 | Historical local research |
| [wgpu-native](https://github.com/gfx-rs/wgpu-native) | MIT / Apache-2.0 | Native backend rendering |
| [SDL2](https://github.com/libsdl-org/SDL) | Zlib | Native backend window, audio and controllers |
| [Chihuahua](https://github.com/anasrar/chihuahua) by Anas Rin | MIT | Independent public DAT/SCR/MDB/TM3/TIM3 format research; pinned local reference d27b3488, no parser source vendored |

Complete notices remain with dependencies. The native IPU decoder includes a BSD
notice for tables originating in Play!. Portable tools have their own licenses.
MIT does not relicense imported GPL code; future incorporation must preserve notices
and meet applicable obligations. No research-backend code is currently copied here.

## Publication

Never publish discs, BIOS, PS2 executables, IRX, data archives, translated game C++,
linked game binaries, memory dumps, Ghidra projects, saves or source assets.

Owner decision, 2026-11-04: selected rendered reference/Bevy screenshots may now
be published for comparison under docs/progress-tracking.md. This supersedes the
earlier journal-only image rule, not the game-data exclusions. Original files
are registered with provenance and hashes; unselected captures remain private.
These images are not MIT, and the owner request is neither rights-holder
permission nor legal certification. No source textures, models or game code
may be published under this screenshot exception.

Git exclusions and CI reduce accidental disclosure risk.
They do not perform an exhaustive assessment of copyright ownership.

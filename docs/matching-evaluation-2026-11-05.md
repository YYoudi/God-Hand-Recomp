# Testing the matching-decompilation toolchain in Linux

Date: 2026-11-05. Follow-up to the [first PC-port evaluation](pc-port-evaluation-2026-11-05.md).
This is a matching **PS2** build experiment, not another PC port or Rust feature.

## Environment actually reproduced

The host reports WSL is not installed and no active hypervisor. Rather than
stopping at the earlier unsupported Windows test, we booted a private Debian
12.15 Linux environment in portable QEMU 11.1.0, using two emulated CPUs and
4 GiB RAM. No WSL installation, driver, elevation, system PATH or security change
was required. The vendor installer was hash-checked and extracted, not run as a
system installer. [QEMU's download page](https://www.qemu.org/download/#windows)
links the [Windows binary vendor](https://qemu.weilnetz.de/w64/).

The Debian image is the official `20261026-2643` genericcloud amd64 build,
verified against the archive SHA-512 list. Its unchanged base has a separate
writable overlay; there is no host directory share. SSH is forwarded only on
localhost. The first legacy-CD seed was not detected; its failed overlay was
retained, then a separate overlay used the documented virtio raw seed method.
The latter boot completed cloud-init and accepted key-only SSH. See the
[cloud-init NoCloud documentation](https://docs.cloud-init.io/en/22.4.2/topics/datasources/nocloud.html).

Actual guest tools: Linux `6.1.0-54-cloud-amd64`, Python **3.11.2**, GNU MIPS
binutils **2.40**, 32-bit glibc and the upstream project toolchain. The checkout
is exactly **`a2c98cb5c3e5b5b3f4a8cc08a19fbfbb6499699a`**, on its Linux filesystem.
The original personal ISO was read only. Just the verified retail boot ELF,
small tables and the requested 1,720-byte `r207.rel` were transferred to this
same-machine private VM. No game input was sent to a remote build service.

## What passed, and what did not

The upstream setup completed. Its original synthetic probes actually exercised:

- Cygnus EE compilation/assembly producing the expected R5900 object flags;
- the separate SN compiler running through wibo, producing the six expected
  quadword callee-save stores in the probe.

These are compiler-tooling tests, not compiled game-function or game-fidelity tests.
The complete initial Linux tooling suite returned **529 passed, 21 skipped,
1 failed**. The failing CLI-filter test requires `expected/build`, which had not
yet been created by the matching setup. This result is retained, not called a
clean test pass. It differs from the earlier bounded Windows run, which stopped
after five failures; the counts are not a controlled platform benchmark.

An actual retail matching attempt has started: verify the registered ELF hash,
create the ROM with `mipsel-linux-gnu-objcopy`, run `scripts/resplat.sh`, split the
small overlay, extract ELF metadata, run `compile.py --setup`, compile the sampled
camera unit, then attempt the full two-worker build and compare the entire ELF.
**At this update, the main split has scanned the input and is still splitting.
No game C object, matching ELF or matching percentage is claimed yet.** Symbol
range/overlap warnings are retained in the private log; a live slow process is
not restarted merely because an observation times out.

After setup, rerun the prerequisite-dependent test. Any failure in generation,
compilation or linking must be recorded against its actual stage, rather than
presenting an upstream report as a local result. Even a byte-identical final ELF
would not establish complete C/C++ recovery: assembly and raw data are still
part of this workflow.

## Actual dependency and license observations

The upstream setup uses moving Python packages and several tool repositories.
These are the versions actually fetched here, not an assertion that the setup
script seals every dependency:

| Component | Observed version/revision | Declared license |
| --- | --- | --- |
| splat64 | 0.50.1 | MIT |
| spimdisasm | 1.42.4 | MIT |
| rabbitizer | 1.16.2 | MIT (wheel notice inspected) |
| pytest | 9.2.0 | MIT (test tooling) |
| m2c | `c632946b8cfbe8fce442b522dba92dfa72f1f317` | GPL-3.0 |
| asm-differ | `0dd09af8f8008f1f880327cf0aca3b26d2562ea2` | Unlicense |
| decomp-permuter | `8556c81d80d1c1af98a858c8f4dc951357f29139` | MIT |
| wibo | 0.6.13; upstream setup checks its binary hash | [MIT](https://github.com/decompals/wibo/blob/0.6.13/LICENSE) |
| objdiff-cli | 3.7.1; upstream setup checks its binary hash | [MIT](https://github.com/encounter/objdiff/blob/v3.7.1/LICENSE-MIT) / [Apache-2.0](https://github.com/encounter/objdiff/blob/v3.7.1/LICENSE-APACHE) notices |

The legacy Cygnus/SN compiler and assembler blobs are fetched privately by the
upstream setup and hash-checked. Their availability online is **not** a conclusion
that all can be redistributed. Their notice/distribution obligations remain a
separate review; no compiler package is published here. Compiler tools, GPL
analysis tools, repository-authored code and game-derived outputs are different
provenance categories. A tool or repository license is not permission for the game.

The existing MIT Rust engine remains paused and unchanged. No GPL/tool/decompiled
source, game binary, generated assembly or ISO-derived asset was copied into it.
Private command scripts, complete logs, output hashes, package freeze and exact
tool revisions are retained locally. Public progress stays in the single
[journal/roadmap manifest](dashboard/data.json).

<div align="center">
  <img alt="nna logo" src="/logo/logo_bg.webp">
</div>

# nna (No Name Architecture)

My custom designed 8 bit general purpose architecture and processors.

## Arches

- [nna8v3](spec/nna8v3.md) (WIP)
- [nna8v2](spec/nna8v2.md)
- [nna8v1](spec/nna8v1.md)

## Assembler (nnaasm)

Example programs are available in `hw/<nna_arch>/programs/bin` where `<nna_arch>` is one of: `nna8v1`,`nna8v2`,`nna8v3`,...

You can write your own programs using the [nnaasm](https://github.com/Xgames123/nna/blob/main/spec/nnaasm.md). assembler

The assembler can be installed by running `./tools/install.sh`
[Assembler docs](spec/nnaasm.md)

## Hardware

- [tt_nna8v3](https://github.com/Xgames123/tt_nna8v3) nna8v3 chip on [tinytapeout](https://tinytapeout.com)

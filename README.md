 ## Sanechips/ZXIC ZX297520V3 USB download protocol
Reverse engineered implementation of the USB download protocol for the ZX297520V3 platform (might be working for the ZX297520V2 too).

The main purpose is booting mainline U-Boot without touching the flash.

## Run
  - Obtain tloader (stage1.bin) / openloader and tboot (stage2.bin) / mainline U-Boot for your device
  - Disassemble the device and find USB download pad (often labeled as 'BOOT' or grid of 14 pads on ZTE MF920U)
  - `sudo -E cargo r --release -- -1 stage1.bin -2 stage2.bin` (untested on the Windows!)
  - Short pad and GND or 2 top pads for MF920U (from the non-USB side) and connect the device

`dump.ts` Frida script can be used to view protocol communication on the Windows with vendor-specific device unbrick app.

## Loaders
Sanechips/ZXIC/ZTE don't call them as loaders, but `evb_tloader.bin` and `evb_tboot.bin`.

First stage is proprietary bootloader running on Cortex M0 (not sure, but it's some M series core) called zloader (flash) or tloader (USB). It initializes DRAM and kicks up main Cortex A53.

Second stage is U-Boot built for download mode.

### Header
First stage has header described in src/header.rs. Size is 0x1b8 bytes. You can modify it by using `mkheader` and `rmheader` tools in this repository. Some examples:
- `cargo r --release --bin rmheader -- --input stage1.bin -o raw -p saved-hdr` - save header data to the `saved-hdr`, and write raw blob to the `raw`. `-p` can be omitted if the header is not needed.
- `cargo r --release --bin mkheader -- --input raw --output with-hdr --preset saved-hdr` - create `with-hdr` image, with `raw` payload and `saved-hdr` header. If `-p` is not present, the image will have dummy header enough for USB boot (but probably not enough for booting from the flash). The header data size will be updated from the saved header file.

The saved header data can be modified to do some fun things (stored in RON format). Note that entrypoint must NOT be masked with Thumb bit. The tool will do it automatically.

## Secure boot bypass
Run with `-e` to enable exploit to bypass secure boot on fused devices.

The [exploit](https://github.com/rva3/CVE-2026-40003) is provided only for research and software freedom purposes. You may use it to boot free and open source U-Boot and the OpenWRT, to replace outdated stock firmware. You may use exploit to repair stock firmware on the device. You may NOT use exploit on devices which you don't own, or without explicit user consent. You may NOT use exploit for any malicious actions.


## Brick tool warning
Do not run anything from unknown sources. BootROM exploit gives full access to your device, including permanent damage.

## License
[AGPLv3](./LICENSE)

 ## Sanechips/ZXIC ZX297520V3 USB download protocol
Reverse engineered implementation of the USB download protocol for the ZX297520V3 platform (might be working for the ZX297520V2 too).

The main purpose is booting mainline U-Boot without touching the flash.

## Run
  - Obtain tloader (stage1.bin) and tboot (stage2.bin) / mainline U-Boot for your device
  - `sudo -E cargo r --release -- stage1.bin stage2.bin` (untested on the Windows!)
  - Disassemble the device and find 14 pads
  - Short 2 top pads (from the non-USB side) and connect the device

`dump.ts` Frida script can be used to view protocol communication on the Windows with vendor-specific device unbrick app.

## Loaders
Sanechips/ZXIC/ZTE don't call them as loaders, but `evb_tloader.bin` and `evb_tboot.bin`.

First stage has some header (0x40 or 0x90 or ? bytes) with unknown entrypoint.

Second stage is U-Boot built for download mode.


## License

[AGPLv3](./LICENSE)

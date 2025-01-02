# Guide for connecting STM32G407VGTx to rust script

This is mostly based on the Embedded Rust Book, however there were some changes as cargo-embed is now part of probe-rs' group of tools. Probe-rs is useful for programming to the MCU as well as debugging. I also added the MCU datasheet and documentation which is needed for memory.x.

## Different Chips

For different boards, It is important memory.x is has corrected info. FLASH and RAM can be found on the manufacturer's website as well as their datasheets. You can also find the origin of RAM by typing ```cargo size -- -Ax```. Heres an example output:
<br>
<br>
```
section                size         addr
.vector_table         0x400    0x8000000 <- FLASH Origin
.text                 0x4dc    0x8000400
.rodata               0x2e8    0x80008dc
.data                     0   0x20000000 <- RAM Origin
.bss                      0   0x20000000
...
```

Within .cargo/config.toml, the chip that is being used also needs to be specified within target, as well as the GNU. Cortex-M4 uses thumbv7em-none-eabihf, however you can find the different targets on this [Platform Support]{https://doc.rust-lang.org/rustc/platform-support.html} page.

## Resources
https://docs.rust-embedded.org/book
https://crates.io/crates/cortex-m-rt
https://probe.rs/
https://probe.rs/targets/?manufacturer=STMicroelectronics&family=SHOW_ALL_FAMILIES

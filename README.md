# Guide for connecting STM32G407VGTx to rust script

This is mostly based on the Embedded Rust Book and the youtube video linked at the bottom of the page, however there were some changes as cargo-embed is now part of probe-rs' group of tools. Probe-rs is useful for programming to the MCU as well as debugging. I also added the MCU datasheet and documentation which is needed for memory.x.

## Debuging with rtt-target:
Crate page: https://crates.io/crates/rtt-target
<br>
<br>
rtt-target is a real time transfer I/O protocal that creates output via a debug probe. It may be preferable to use rtt-target as there would be no delays and minimal blocking for real-time applications. A debug probe is a specialized hardware device that allows developers to control and monitor the execution of a program on a target device. It allows for logging on the microcontroller.
<br>
<br>
Add the following:
<br>
```cargo add cortex-m --features critical-section-single-core```
<br>
```cargo add rtt-target```
<br>
<br>


In Embed.toml, paste the following:
```
[default.rtt]
enabled = true 
```
If you are using gdb, set it to false.
<br>
Your main.rs should look something like the following:
```
use rtt_target::{rtt_init_print, rprintln};

fn main() {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
    }
}
```
run cargo embed and ou should get "Hello, world!" 

## Debuging with GDB:

For the use of GDB, download an [ARM GNU toolchain]{https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads} from ARMS' website. It should be under AArch32 bare-metal target (arm-none-eabi). 
<br>
<br>
Afterwards, go into Embed.toml and paste the following:
```
[default.gdb]
enabled = true

[default.reset]
halt_afterwards = true
```
If you have rtt-target enabled, set it to false.
<br>
<br>
When you run cargo embed, a gdb stub should constantly loop. In another terminal, to run gdb, type the following:
```
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/[file_name]
```
You can connect to the gdb stub by typing ```target remote [Port GDB stub listening at]```.
<br>
Afterwards, you should be set.

## Different Microcontrollers

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
https://www.youtube.com/watch?v=TOAynddiu5M

https://docs.rust-embedded.org/book

https://crates.io/crates/cortex-m-rt

https://probe.rs/

https://probe.rs/targets/?manufacturer=STMicroelectronics&family=SHOW_ALL_FAMILIES

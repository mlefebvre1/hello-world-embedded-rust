# Hello-World Embedded Rust

For the STM32F4DISCOVERY board, provides LEDs blink patterns. The USER push-button allows switching pattern.

## How to install
See : https://docs.rust-embedded.org/book/intro/install/linux.html<br><br>
Add arch
```shell
$ rustup target add thumbv7em-none-eabi
```
## How to use
Build
```shell
$ cargo build --target thumbv7m-none-eabi
```
Start OpenOCD and Debug
In one terminal session:
```shell
$ openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg
```
In a second terminal session:
```shell
$ gdb-multiarch -q target/thumbv7m-none-eabi/debug/hello-world-embedded
```
Upload new code in GDB
```
load
```
Or simply
```shell
$ cargo run
```

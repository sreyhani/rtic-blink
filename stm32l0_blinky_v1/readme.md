# STM32l0 RTIC Blink example

Working example of simple LED blinking application for STM32L051C8 board. Example uses schedule API and peripherials access. This example is based on the example in [rtic repository](https://github.com/rtic-rs/rtic/tree/master/examples/stm32f3_blinky).

## How-to

### Build

Run `cargo build` to compile the code. If you run it for the first time, it will take some time to download and compile dependencies.

```bash
$ cargo build
```

### Setup environment, flash and run program

In the [Discovery Book](https://rust-embedded.github.io/discovery) you find all needed informations to setup the environment, flash the controler and run the program.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};

#[app(device = stm32l0xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;
    use systick_monotonic::{fugit::Duration, Systick};
    use stm32l0xx_hal::gpio::gpiob::PB0;
    use stm32l0xx_hal::gpio::{Output, PushPull};
    use stm32l0xx_hal::rcc::RccExt;
    use stm32l0xx_hal::rcc::Config;
    use stm32l0xx_hal::gpio::GpioExt;
    use stm32l0xx_hal::prelude::OutputPin;
    
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PB0<Output<PushPull>>,
        state: bool,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Setup clocks
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());

        let mono = Systick::new(cx.core.SYST, 36_000_000);

        rtt_init_print!();
        rprintln!("init");


        // Setup LED
        let gpiob = cx.device.GPIOB.split(&mut rcc);
        let mut led = gpiob
            .pb0
            .into_push_pull_output();
        led.set_high().unwrap();

        // Schedule the blinking task
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();

        (
            Shared {},
            Local { led, state: false },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, state])]
    fn blink(cx: blink::Context) {
        rprintln!("blink");
        if *cx.local.state {
            cx.local.led.set_high().unwrap();
            *cx.local.state = false;
        } else {
            cx.local.led.set_low().unwrap();
            *cx.local.state = true;
        }
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(200)).unwrap();
    }
}

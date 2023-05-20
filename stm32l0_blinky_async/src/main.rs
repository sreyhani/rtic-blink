#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;
use rtic::app;
use rtic_monotonics::systick::*;
use rtt_target::{rprintln, rtt_init_print};

#[app(device = stm32l0xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;
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

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Setup clocks
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());

        // Initialize the systick interrupt & obtain the token to prove that we did
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_mono_token); // default STM32F303 clock-rate is 36MHz

        rtt_init_print!();
        rprintln!("init");


        // Setup LED
        let gpiob = cx.device.GPIOB.split(&mut rcc);
        let mut led = gpiob
            .pb0
            .into_push_pull_output();
        led.set_high().unwrap();

        // Schedule the blinking task
        blink::spawn().ok();

        (Shared {}, Local { led, state: false })
    }

    #[task(local = [led, state])]
    async fn blink(cx: blink::Context) {
        loop {
            rprintln!("blink");
                cx.local.led.set_high().unwrap();
                *cx.local.state = false;
                Systick::delay(300.millis()).await;
                cx.local.led.set_low().unwrap();
                *cx.local.state = true;
                Systick::delay(300.millis()).await;

        }
    }
}

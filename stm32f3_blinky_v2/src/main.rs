#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};

#[app(device = stm32l0xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;
    use stm32l0xx_hal::gpio::gpiob::PB0;
    use stm32l0xx_hal::gpio::GpioExt;
    use stm32l0xx_hal::prelude::*;
    use stm32l0xx_hal::rcc::Config;
    use stm32l0xx_hal::gpio::*;

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
        // let mut flash = cx.device.FLASH.constrain();
        // let mut rcc = cx.device.RCC.constrain();
        let mut rcc = cx.device.RCC.freeze(Config::hsi16());
        // let clock_config = rcc::Config::pll(
        //     rcc::PLLSource::HSE(12.mhz()),
        //     rcc::PLLMul::Mul8,
        //     rcc::PLLDiv::Div4,
        // );
        // let mut rcc = cx.device.RCC.freeze(clock_config);


        rtt_init_print!();
        rprintln!("init");

        // let _clocks = rcc
        //     .cfgr
        //     .use_hse(8.MHz())
        //     .sysclk(36.MHz())
        //     .pclk1(36.MHz());
            // .freeze(&mut flash.acr);

        // Setup LED
        let gpiob = cx.device.GPIOB.split(&mut rcc);
        let mut led = gpiob.pb0.into_push_pull_output();

        // let mut led = gpiob
        //     .pb0
        //     .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        led.set_high().unwrap();

        // Schedule the blinking task

        (Shared {}, Local { led, state: false })
    }

    #[idle(local = [led, state])]
    fn idle(_cx: idle::Context) -> !{
        loop {
            
        }
    }
}

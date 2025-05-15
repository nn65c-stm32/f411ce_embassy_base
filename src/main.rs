#![no_std]
#![no_main]

use defmt::{error, info, trace};
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::{
    Peri, Peripherals,
    exti::{AnyChannel, ExtiInput},
    gpio::{AnyPin, Level, Output, Pull, Speed},
    rcc::{
        AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, Sysclk,
    },
    time::Hertz,
};
use embassy_time::Timer;

async fn handle_blink(mut led: Output<'static>) {
    loop {
        trace!("Blinking LED");
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
pub async fn blink_anypin(pin: Peri<'static, AnyPin>, level: Level, speed: Speed) {
    let led = Output::new(pin, level, speed);
    handle_blink(led).await;
}

#[embassy_executor::task]
pub async fn blink_outpin(led: Output<'static>) {
    handle_blink(led).await;
}

// Debounce button with vertical counting.
// - Polls every 10 ms.
// - Requires 3 stable reads (30 ms) before confirming state change.
// - Resets if bouncing is detected.
async fn handle_button(button: ExtiInput<'static>) {
    let mut last_state = button.is_high();
    let mut stable_count = 0;
    let required_stable_reads = 3;

    loop {
        Timer::after_millis(10).await;

        let current = button.is_high();
        if current != last_state {
            stable_count += 1;
            if stable_count >= required_stable_reads {
                last_state = current;
                stable_count = 0;

                if current {
                    info!("Button released!");
                } else {
                    info!("Button pressed!");
                }
            }
        } else {
            stable_count = 0;
        }
    }
}

#[embassy_executor::task]
pub async fn get_button_anypin(
    pin: Peri<'static, AnyPin>,
    channel: Peri<'static, AnyChannel>,
    pull: Pull,
) {
    let button = ExtiInput::new(pin, channel, pull);
    handle_button(button).await;
}

#[embassy_executor::task]
pub async fn get_button_pin(button: ExtiInput<'static>) {
    handle_button(button).await;
}

#[allow(dead_code)]
fn clock_hse_25mhz() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: Hertz(25_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV12,
        mul: PllMul::MUL96,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV5),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.sys = Sysclk::PLL1_P;

    config
}

#[allow(dead_code)]
fn clock_hse_8mhz() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL100,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV5),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.sys = Sysclk::PLL1_P;

    config
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = clock_hse_25mhz();
    // let config = clock_hse_8mhz();

    let p: Peripherals = embassy_stm32::init(config);

    info!("Start");

    let led_pin_level = Level::Low;
    let led_pin_speed = Speed::High;

    // let led_anypin = p.PC13.into();
    // if let Err(e) = spawner.spawn(blink_anypin(led_anypin, led_pin_level, led_pin_speed)) {
    //     error!("Failed to spawn blink_anypin task: {:?}", e);
    // }

    let led_outpin = p.PC13;
    if let Err(e) = spawner.spawn(blink_outpin(Output::new(
        led_outpin,
        led_pin_level,
        led_pin_speed,
    ))) {
        error!("Failed to spawn blink_outpin task: {:?}", e);
    }

    let button_pin_channel = p.EXTI0.into();
    let button_pin_pull = Pull::Up;

    // let button_anypin = p.PA0.into();
    // if let Err(e) = spawner.spawn(get_button_anypin(button_anypin, button_pin_channel, button_pin_pull)) {
    //     error!("Failed to spawn button_anypin task: {:?}", e);
    // }

    let button_pin = p.PA0;
    if let Err(e) = spawner.spawn(get_button_pin(ExtiInput::new(
        button_pin,
        button_pin_channel,
        button_pin_pull,
    ))) {
        error!("Failed to spawn button_input task: {:?}", e);
    }
}

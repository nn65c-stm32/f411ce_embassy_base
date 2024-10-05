#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{AnyPin, Level, Output, Pin, Pull, Speed}};
use embassy_time::Timer;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, Speed::High);

    loop {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    defmt::info!("Start");
    spawner.spawn(blink(p.PC13.degrade())).unwrap();

    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);

    loop {
        button.wait_for_falling_edge().await;
        info!("Button pressed!");
        button.wait_for_rising_edge().await;
        info!("Button released!");
    }
}
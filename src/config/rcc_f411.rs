#![allow(dead_code)]

use embassy_stm32::rcc as rcc_f411;
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;

/// Initializes the RCC configuration for a 25 MHz HSE and 100 MHz system clock.
pub fn init_25mhz_hse_100mhz(config: &mut Config) {
    config.rcc.pll_src = rcc_f411::PllSource::HSE;
    config.rcc.hse = Some(rcc_f411::Hse {
        freq: Hertz(25_000_000),
        mode: rcc_f411::HseMode::Oscillator,
    });
    config.rcc.pll = Some(rcc_f411::Pll {
        prediv: rcc_f411::PllPreDiv::DIV25,
        mul: rcc_f411::PllMul::MUL200,
        divp: Some(rcc_f411::PllPDiv::DIV2),
        divq: Some(rcc_f411::PllQDiv::DIV4),
        divr: None,
    });
    config.rcc.ahb_pre = rcc_f411::AHBPrescaler::DIV1;
    config.rcc.apb1_pre = rcc_f411::APBPrescaler::DIV2;
    config.rcc.apb2_pre = rcc_f411::APBPrescaler::DIV1;
    config.rcc.sys = rcc_f411::Sysclk::PLL1_P;
}

/// Initializes the RCC configuration for an 8 MHz HSE and 100 MHz system clock.
pub fn init_8mhz_hse_100mhz(config: &mut Config) {
    config.rcc.pll_src = rcc_f411::PllSource::HSE;
    config.rcc.hse = Some(rcc_f411::Hse {
        freq: Hertz(8_000_000),
        mode: rcc_f411::HseMode::Oscillator,
    });
    config.rcc.pll = Some(rcc_f411::Pll {
        prediv: rcc_f411::PllPreDiv::DIV4,
        mul: rcc_f411::PllMul::MUL100,
        divp: Some(rcc_f411::PllPDiv::DIV2),
        divq: Some(rcc_f411::PllQDiv::DIV4),
        divr: None,
    });
    config.rcc.ahb_pre = rcc_f411::AHBPrescaler::DIV1;
    config.rcc.apb1_pre = rcc_f411::APBPrescaler::DIV2;
    config.rcc.apb2_pre = rcc_f411::APBPrescaler::DIV1;
    config.rcc.sys = rcc_f411::Sysclk::PLL1_P;
}
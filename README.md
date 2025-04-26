# Embedded Rust with STM32 and Embassy

Base Rust project for BlackPill.

Board used is [STM32F411CEU6](https://github.com/WeActStudio/WeActStudio.MiniSTM32F4x1).

## Base setup for STM32 and ST-link programmer
[Embedded Rust with STM32 and Embassy](https://github.com/nn65c-stm32/.github/blob/main/profile/README.md)

## Expected results
Blinking LED.

Button press:
```
INFO  Button pressed!
└─ f411ce_embassy_base::____embassy_main_task::{async_fn#0} @ src\main.rs:41
INFO  Button released!
└─ f411ce_embassy_base::____embassy_main_task::{async_fn#0} @ src\main.rs:43
```
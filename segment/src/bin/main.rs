#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]

use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::xtensa_lx::timer::delay;
use esp_println::println;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    loop {
        println!("Panic!!!!\n {}", panic_info);
    }
}

esp_bootloader_esp_idf::esp_app_desc!();

const fn us_to_ticks(us: f32) -> u16 {
    (us * 80.0) as u16 // 1 tick = 12.5 ns at 80 MHz
}

// WS2812 bit timings (ticks @ 80 MHz)
const T0H: u16 = us_to_ticks(0.35); // ≈ 28
const T0L: u16 = us_to_ticks(0.80); // ≈ 64
const T1H: u16 = us_to_ticks(0.70); // ≈ 56
const T1L: u16 = us_to_ticks(0.60); // ≈ 44

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let mut pin_a = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let mut pin_b = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let mut pin_c = Output::new(peripherals.GPIO6, Level::Low, OutputConfig::default());
    let mut pin_d = Output::new(peripherals.GPIO39, Level::Low, OutputConfig::default());
    let mut pin_e = Output::new(peripherals.GPIO40, Level::Low, OutputConfig::default());
    let mut pin_f = Output::new(peripherals.GPIO41, Level::Low, OutputConfig::default());
    let mut pin_g = Output::new(peripherals.GPIO42, Level::Low, OutputConfig::default());

    let mut button = Input::new(peripherals.GPIO35, InputConfig::default().with_pull(Pull::Up));
    let mut pin_decimal = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    // 1111111 - nothing
    // 1111110 - 0
    // 0110000 - 1
    // 1101101 - 2
    // 1111001 - 3
    // 0110010 - 4
    // 1011011 - 5
    // 1011111 - 6
    // 1110000 - 7
    // 1111111 - 8
    // 1111011 - 9

    let mut digit: u8 = 0;

    println!("Button is {:?}", button.level());

    loop {
        println!("Button is {:?}", button.level());
        let mut i: u8 = 1;
        // while i < 6 {
        //     pin_decimal.toggle();
        //     delay.delay_millis(200);b      
        //     i += 1;
        // }
        let digital_signal: [bool; 7] = set_digit(digit);

        pin_a.set_level(digital_signal[0].into());
        pin_b.set_level(digital_signal[1].into());
        pin_c.set_level(digital_signal[2].into());
        pin_d.set_level(digital_signal[3].into());
        pin_e.set_level(digital_signal[4].into());
        pin_f.set_level(digital_signal[5].into());
        pin_g.set_level(digital_signal[6].into());

        if (button.is_high()) {
            digit = if (digit + 1) > 9 { 0 } else { digit + 1 };
        }
        delay.delay_millis(100);
    }
}

fn set_digit(digit: u8) -> [bool; 7] {
    match digit {
        0 => [false,false,false,false,false,false,true],
        1 => [true,false,false,true,true,true,true],
        2 => [false,false,true,false,false,true,false],
        3 => [false,false,false,false,true,true,false],
        4 => [true,false,false,true,true,false,false],
        5 => [false,true,false,false,true,false,false],
        6 => [false,true,false,false,false,false,false],
        7 => [false,false,false,true,true,true,true],
        8 => [false,false,false,false,false,false,false],
        9 => [false,false,false,false,true,false,false],
        _ => [true,true,true,true,true,true,true]
    }
}

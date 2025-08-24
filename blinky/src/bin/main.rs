#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::gpio::Level;
use esp_hal::rmt::{PulseCode, Rmt, TxChannel, TxChannelConfig, TxChannelCreator};
use esp_hal::rng::Rng;
use esp_hal::time::Rate;
use esp_println::println;
use blinky::random_color;

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
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();

    let mut tx_config = TxChannelConfig::default();

    tx_config = tx_config.with_carrier_modulation(false);
    tx_config = tx_config.with_clk_divider(1);
    tx_config = tx_config.with_idle_output(true);
    tx_config = tx_config.with_idle_output_level(Level::Low);

    let mut tx = rmt.channel0.configure_tx(peripherals.GPIO38, tx_config).unwrap();

    let delay = Delay::new();

    let mut rng: Rng = Rng::new(peripherals.RNG);

    let mut prev: [u8; 3] = [0, 0, 0];

    let steps: i16 = 50;

    loop {
        let next = random_color(&mut rng);

        // calculates intermediate colors and displays them in neo pixel (in ESP32-S3-DevKitC-1-N8R8) led creating a fading effect
        for step in 0..=steps {
            let g = prev[0] as i16 + (step * (next[0] as i16 - prev[0] as i16) / steps);
            let r = prev[1] as i16 + (step * (next[1] as i16 - prev[1] as i16) / steps);
            let b = prev[2] as i16 + (step * (next[2] as i16 - prev[2] as i16) / steps);

            let grb = [g as u8, r as u8, b as u8];
            let pulses = build_pulses(&grb);

            let in_prog = tx.transmit(&pulses).unwrap();
            tx = in_prog.wait().unwrap();

            delay.delay_millis(20);
        }
        prev = next;
    }
}

fn build_pulses(grb: &[u8;3]) -> [u32;25] {
    let mut pulses = [0; 25];

    let mut i = 0;

    for &byte in grb.iter() {
        for bit in (0..8).rev() {
            let is_one = ((byte >> bit) & 1) != 0;
            pulses[i] = if is_one {
                PulseCode::new(Level::High, T1H, Level::Low, T1L)
            } else {
                PulseCode::new(Level::High, T0H, Level::Low, T0L)
            };
            i += 1;
        }
    }

    // RESET pulse
    PulseCode::reset(&mut pulses[i]);

    pulses
}

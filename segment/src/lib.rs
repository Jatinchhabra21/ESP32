#![no_std]

use esp_hal::rng::Rng;
use rand::RngCore;

pub fn random_color(rng: &mut Rng) -> [u8; 3] {
    let r = rng.next_u32() as u8;
    let g = rng.next_u32() as u8;
    let b = rng.next_u32() as u8;
    [r, g, b]
}

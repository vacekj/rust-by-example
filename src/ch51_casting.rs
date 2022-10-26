// Suppress all warnings from casts which overflwo
#![allow(overflowing_literals)]

pub fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    let integer: u8 = decimal;
}

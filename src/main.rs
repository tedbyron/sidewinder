#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![doc = include_str!("../README.md")]
#![windows_subsystem = "console"]

mod ppm;

fn main() {
    ppm::write(256, 256).unwrap();
}

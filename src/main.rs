pub mod io;
use crate::io::*;
use quicksilver::prelude::*;

pub fn main() {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<Game>("Learn Rust", Vector::new(2080, 1240), settings);
}

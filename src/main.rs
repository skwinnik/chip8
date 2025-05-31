mod cpu;
mod display;
mod memory;
mod u12;

use crate::display::{get_display_adapter, DisplayAdapter};

fn main() {
    let mut display_adapter = get_display_adapter();

    let display_size = display_adapter.get_size();
    let mut buffer: Vec<u32> = vec![0; display_size.0 * display_size.1];

    let mut color = 0x000000;
    for (i, pixel) in buffer.iter_mut().enumerate() {
        *pixel = color;
        color += u32::try_from(i).unwrap() * 10
    }

    while display_adapter.is_open() {
        display_adapter.update(&mut buffer);
    }
}

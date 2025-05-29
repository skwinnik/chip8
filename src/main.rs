mod display;

use crate::display::{get_display_adapter, DisplayAdapter};

fn main() {
    let mut display_adapter = get_display_adapter();

    let display_size = display_adapter.get_size();
    let mut buffer: Vec<u32> = vec![0; display_size.0 * display_size.1];

    for i in buffer.iter_mut() {
        *i = 0xff0000;
    }

    while display_adapter.is_open() {
        display_adapter.update(&mut buffer);
    }
}

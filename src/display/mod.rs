mod minifb;

pub trait DisplayAdapter {
    fn new() -> Self;
    fn update(&mut self, buffer: &mut Vec<u32>);
    fn get_size(&self) -> (usize, usize);
    fn is_open(&self) -> bool;
}

pub fn get_display_adapter() -> impl DisplayAdapter {
    minifb::MinifbDisplay::new()
}

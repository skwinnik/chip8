pub mod minifb;
pub mod test_display;

pub trait Display {
    fn new() -> Self;
    fn update(&mut self, buffer: &mut Vec<bool>);
    fn get_size(&self) -> (usize, usize);
    fn is_open(&self) -> bool;
}

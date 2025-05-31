pub mod minifb;

pub trait Display {
    fn new() -> Self;
    fn update(&mut self, buffer: &mut Vec<u32>);
    fn get_size(&self) -> (usize, usize);
    fn is_open(&self) -> bool;
}

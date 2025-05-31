use crate::display::Display;

pub struct TestDisplay {}

impl Display for TestDisplay {
    fn new() -> Self {
        TestDisplay {}
    }

    fn update(&mut self, _buffer: &mut Vec<bool>) {
        print!("update")
    }

    fn get_size(&self) -> (usize, usize) {
        (64, 32)
    }

    fn is_open(&self) -> bool {
        true
    }
}

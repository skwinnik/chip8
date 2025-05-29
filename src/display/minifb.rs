use minifb::{Window, WindowOptions};

static TITLE: &str = "Chip-8";
static WIDTH: usize = 640;
static HEIGHT: usize = 320;

pub struct MinifbDisplay {
    window: Window,

    width: usize,
    height: usize,
}

impl crate::display::DisplayAdapter for MinifbDisplay {
    fn new() -> Self {
        let window_result = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default());
        match window_result {
            Ok(mut window) => {
                window.set_target_fps(60);
                window.topmost(true);

                MinifbDisplay {
                    window: window,
                    width: WIDTH,
                    height: HEIGHT,
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    fn update(&mut self, buffer: &mut Vec<u32>) {
        let result = self
            .window
            .update_with_buffer(buffer, self.width, self.height);

        match result {
            Ok(_) => return,
            Err(err) => panic!("{}", err),
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

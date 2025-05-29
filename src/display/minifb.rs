use minifb::{Window, WindowOptions};

static TITLE: &str = "Chip-8";
static WIDTH: usize = 64;
static HEIGHT: usize = 32;
static SCALING_FACTOR: usize = 32;

pub struct MinifbDisplay {
    window: Window,

    width: usize,
    height: usize,
}

impl MinifbDisplay {
    fn get_scaled_buffer(&self, buffer: Vec<u32>) -> Vec<u32> {
        let mut scaled_buffer: Vec<u32> =
            vec![0; self.width * SCALING_FACTOR * self.height * SCALING_FACTOR];

        for y in 0..self.height {
            for x in 0..self.width {
                let original_pixel = buffer[y * self.width + x];

                // Scale each pixel to a SCALING_FACTOR x SCALING_FACTOR block
                for dy in 0..SCALING_FACTOR {
                    for dx in 0..SCALING_FACTOR {
                        let scaled_x = x * SCALING_FACTOR + dx;
                        let scaled_y = y * SCALING_FACTOR + dy;
                        let scaled_index = scaled_y * (self.width * SCALING_FACTOR) + scaled_x;
                        scaled_buffer[scaled_index] = original_pixel;
                    }
                }
            }
        }

        scaled_buffer
    }

    fn set_grid(&self, scaled_buffer: &mut Vec<u32>) {
        let scaled_width = self.width * SCALING_FACTOR;
        let scaled_height = self.height * SCALING_FACTOR;
        let grid_color = 0x404040; // Dark gray color for grid lines

        // Draw vertical grid lines
        for x in (0..scaled_width).step_by(SCALING_FACTOR) {
            for y in 0..scaled_height {
                let index = y * scaled_width + x;
                if index < scaled_buffer.len() {
                    scaled_buffer[index] = grid_color;
                }
            }
        }

        // Draw horizontal grid lines
        for y in (0..scaled_height).step_by(SCALING_FACTOR) {
            for x in 0..scaled_width {
                let index = y * scaled_width + x;
                if index < scaled_buffer.len() {
                    scaled_buffer[index] = grid_color;
                }
            }
        }
    }
}

impl crate::display::DisplayAdapter for MinifbDisplay {
    fn new() -> Self {
        let window_result = Window::new(
            TITLE,
            WIDTH * SCALING_FACTOR,
            HEIGHT * SCALING_FACTOR,
            WindowOptions {
                ..Default::default()
            },
        );
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
        let mut scaled_buffer = self.get_scaled_buffer(buffer.to_vec());
        self.set_grid(&mut scaled_buffer);
        let result = self.window.update_with_buffer(
            &scaled_buffer,
            self.width * SCALING_FACTOR,
            self.height * SCALING_FACTOR,
        );

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

use crate::graphics::Graphics;
use crate::input::ButtonState;
use crate::input::Input;
use std::sync::Arc;

static mut TEST: u32 = 0;

mod drawables {
    mod grid;
    mod square;

    pub use grid::Grid;
    pub use square::Square;
}

pub struct App {
    input: Arc<Input>,
    grid: drawables::Grid,
    square: drawables::Square,
}

impl App {
    pub fn new(gfx: &mut Graphics, input: Arc<Input>) -> Self {
        Self {
            input: input,
            grid: drawables::Grid::new(gfx, cgmath::Vector2 { x: 5, y: 4 }, 50.0),
            square: drawables::Square::new(gfx, cgmath::Vector2::new(0.0, 0.0), 10.0),
        }
    }

    pub fn resize_callback(&self, gfx: &mut Graphics) {
        gfx.recreate_swapchain();
    }

    pub fn run(&self, _gfx: &Graphics) {
        self.square.transform.access_data(|data| {
            data.transform = match self.input.keyboard.get_key_state(28) {
                Some(ButtonState::Held(_)) => cgmath::Matrix4::from_scale(10.0).into(),
                _ => cgmath::Matrix4::from_scale(5.0).into(),
            }
        });

        //// Input test
        //unsafe {
        //    if self.input.keyboard.is_key_pressed(28) {
        //        TEST += 1;
        //    }
        //    print!("\r{}\r", (0..100).into_iter().map(|p| ' ').collect::<String>());
        //    print!("Enter pressed {TEST} times. ");
        //    let duration = match self.input.keyboard.is_key_held(28) {
        //        Some(t) => t.as_secs_f32(),
        //        None => 0.0
        //    };
        //    print!("Enter key held down for {} seconds", duration);
        //    _ = std::io::stdout().flush();
        //}

        //// Cursor position test + CartesianToNormalized test
        //let window_extent = gfx.get_window().inner_size();
        //let matrix =
        //    cgmath::ortho(
        //        -((window_extent.width / 2) as f32), (window_extent.width / 2) as f32,
        //        (window_extent.height / 2) as f32, -((window_extent.height / 2) as f32),
        //        0.0, 10.0
        //    );
        //
        //let cc = self.input.mouse.cursor_position.get();
        //let cart_coords = Vector4{x: cc.x as f32, y: cc.y as f32, z: 0.0, w: 1.0};
        //let norm_coords = matrix * cart_coords;
        //
        //print!("\r{}\r", (0..100).into_iter().map(|p| ' ').collect::<String>());
        //if self.input.keyboard.is_key_held(28).is_some() {
        //    print!("Normalized : {:?}", norm_coords.truncate().truncate());
        //}
        //else {
        //    print!("Cartesian : {:?}", cart_coords.truncate().truncate());
        //}
        //_ = std::io::stdout().flush();
    }
}

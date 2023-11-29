use cgmath::Vector2;

use crate::graphics::Graphics;
use crate::input::Input;
use std::sync::Arc;

mod drawables {
    mod grid;
    mod square;

    pub use grid::Grid;
    pub use square::Square;
}

pub struct App {
    input: Arc<Input>,
    window_size: Vector2<f32>,

    pixels: Vec<drawables::Square>,
    grid: drawables::Grid,
}

impl App {
    pub fn new(gfx: &mut Graphics, input: Arc<Input>) -> Self {
        let window_extent = gfx.get_window().inner_size();
        let mut app = Self {
            input: input,
            window_size: Vector2 {
                x: window_extent.width as f32,
                y: window_extent.height as f32,
            },
            pixels: Vec::new(),
            grid: drawables::Grid::new(gfx, cgmath::Vector2 { x: 5, y: 4 }, 50.0),
        };

        create_pixels(gfx, &app.grid, &mut app.pixels);

        return app;
    }

    pub fn resize_callback(&mut self, gfx: &mut Graphics) {
        gfx.recreate_swapchain();

        let window_extent = gfx.get_window().inner_size();
        self.window_size = Vector2 {
            x: window_extent.width as f32,
            y: window_extent.height as f32,
        };

        self.grid.pc.access_data(|data| {
            data.transform = cgmath::Matrix4::from_scale(
                gfx.get_window().inner_size().height as f32 / (self.grid.dimensions.y + 1) as f32,
            )
            .into()
        });
    }

    pub fn run(&self, gfx: &Graphics) {
        if self.input.mouse.is_button_pressed(1) {
            if let Some(i) = self.grid_collision() {
                println!("clicked box: {i}");
            }
        }
    }

    fn grid_collision(&self) -> Option<u32> {
        let scaling_factor = self.window_size.y / (self.grid.dimensions.y + 1) as f32;
        let cursor_pos = self.input.mouse.cursor_position.get();

        let x_offset = -(self.grid.dimensions.x as f32 / 2.0) * scaling_factor;
        let y_offset = (self.grid.dimensions.y as f32 / 2.0) * scaling_factor;

        for y in 0..self.grid.dimensions.y {
            for x in 0..self.grid.dimensions.x {
                let x_min = x_offset + (x as f32) * scaling_factor;
                let x_max = x_offset + ((x + 1) as f32) * scaling_factor;
                let y_min = y_offset - ((y + 1) as f32) * scaling_factor;
                let y_max = y_offset - (y as f32) * scaling_factor;

                if x_min < cursor_pos.x as f32
                    && (cursor_pos.x as f32) < x_max
                    && y_min < cursor_pos.y as f32
                    && (cursor_pos.y as f32) < y_max
                {
                    return Some(x + self.grid.dimensions.x * y);
                }
            }
        }
        return None;
    }
}

fn create_pixels(gfx: &mut Graphics, grid: &drawables::Grid, out: &mut Vec<drawables::Square>) {
    let window_size = gfx.get_window().inner_size();
    let scaling_factor = window_size.height as f32 / (grid.dimensions.y + 1) as f32;

    let x_offset = -(grid.dimensions.x as f32 / 2.0) * scaling_factor;
    let y_offset = (grid.dimensions.y as f32 / 2.0) * scaling_factor;

    for y in 0..grid.dimensions.y {
        for x in 0..grid.dimensions.x {
            let center = Vector2 {
                x: x_offset + ((x as f32) + 0.5) * scaling_factor,
                y: y_offset - ((y as f32) + 0.5) * scaling_factor,
            };

            out.push(drawables::Square::new(gfx, center, scaling_factor * 0.5));
        }
    }
}

fn square_transform_helper(position: Vector2<f32>) -> cgmath::Matrix4<f32> {
    todo!()
}

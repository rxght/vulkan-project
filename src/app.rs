use std::io::Write;
use std::sync::Arc;
use cgmath::Deg;
use cgmath::ElementWise;
use cgmath::Point3;
use cgmath::Rad;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::{event_loop::EventLoop, window::Window};
use crate::graphics::Graphics;
use crate::graphics::bindable;
use crate::graphics::drawable::{DrawableEntry, self};
use crate::input::Input;

use self::drawables::*;
use self::drawables::textest::Pc;

static mut TEST: u32 = 0;

mod drawables {
    pub mod triangle;
    pub mod cube;
    pub mod textest;
    pub mod grid;
}

pub struct App
{
    input: Arc<Input>,
    grid: drawables::grid::Grid,
}

impl App
{
    pub fn new(gfx: &mut Graphics, input: Arc<Input>) -> Self
    {
        Self {
            input: input,
            grid: drawables::grid::Grid::new(gfx, cgmath::Vector2{x: 5, y: 4}),
        }
    }
    
    pub fn resize_callback(&self)
    {
        
    }

    pub fn run(&self, gfx: &Graphics)
    {
        const MARGIN_PX: f32 = 50.0;

        let window_size = gfx.get_window().inner_size();
        let y_scaling = 2.0 / window_size.height as f32;
        let x_scaling = 2.0 / window_size.width as f32;

        let y = (window_size.height as f32 - 2.0 * MARGIN_PX) / self.grid.dimensions.y as f32;
        let x = (window_size.width as f32 - 2.0 * MARGIN_PX) / self.grid.dimensions.x as f32;
        let shared = f32::min(x, y);

        self.grid.pc.access_data(|data| {
            data.scaling = [shared * x_scaling, shared * y_scaling];
        });
    }
}
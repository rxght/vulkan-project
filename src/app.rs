use std::sync::Arc;
use cgmath::Deg;
use cgmath::Point3;
use cgmath::Rad;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::{event_loop::EventLoop, window::Window};
use crate::graphics::Graphics;
use crate::graphics::bindable;
use crate::graphics::drawable::{DrawableEntry, self};

use self::drawables::*;
use self::drawables::textest::Pc;

mod drawables {
    pub mod triangle;
    pub mod cube;
    pub mod textest;
}

pub struct App
{
    start_time: std::time::Instant,
    textured_square: drawables::textest::TexturedSquare,
}

impl App
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        let window_extent = gfx.get_window().inner_size();
        let aspect = window_extent.width as f32 / window_extent.height as f32;
        Self {
            start_time: std::time::Instant::now(),
            textured_square: drawables::textest::TexturedSquare::new(gfx, true),
        }
    }
    
    pub fn resize_callback(&mut self)
    {
        
    }

    pub fn run(&mut self, gfx: &Graphics)
    {
        let time = (std::time::Instant::now() - self.start_time).as_secs_f32();

        self.textured_square.pc.access_data(|data| {
            data.model = cgmath::Matrix4::from_angle_y(Rad((time).sin())).into()
        });
    }
}
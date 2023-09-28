use std::sync::Arc;
use cgmath::Rad;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::{event_loop::EventLoop, window::Window};
use crate::graphics::Graphics;
use crate::graphics::bindable;
use crate::graphics::drawable::{DrawableEntry, self};

use self::drawables::*;
use self::drawables::textest::Ubo;

mod drawables {
    pub mod triangle;
    pub mod cube;
    pub mod textest;
}

pub struct App
{
    start_time: std::time::Instant,
    textest: drawables::textest::TexturedSquare,
}

impl App
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        Self {
            start_time: std::time::Instant::now(),
            textest: drawables::textest::TexturedSquare::new(gfx, true)
        }
    }
    
    pub fn resize_callback(&mut self)
    {
        
    }

    pub fn run(&mut self)
    {
        let time = (std::time::Instant::now() - self.start_time).as_secs_f32();

        unsafe {
            let ubo = self.textest.uniform.subbuffer.mapped_ptr().unwrap().cast::<Ubo>().as_mut();
            ubo.model = cgmath::Matrix4::from_angle_y(Rad(time.sin())).into();
        }
    }
}
use std::sync::Arc;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::event_loop::EventLoop;
use graphics::Graphics;
use crate::app::graphics::bindable;
use self::{graphics::drawable::{DrawableEntry, self}, drawables::Ubo};

mod graphics;
mod drawables {
    pub mod triangle;
    pub mod cube;
    mod ubotest; pub use ubotest::*;
}

pub struct App
{
    start_time: std::time::Instant,
    triangle: drawables::UboTestDrawable,
    gfx: Graphics
}

impl App
{
    pub fn new() -> (App, EventLoop<()>)
    {
        let (mut gfx, event_loop) = Graphics::new();
        (Self {
            start_time: std::time::Instant::now(),
            triangle: drawables::UboTestDrawable::new(&mut gfx, true),
            gfx: gfx

        }, event_loop)
    }
    
    pub fn resize_callback(&mut self)
    {
        self.gfx.recreate_swapchain();
    }

    pub fn run(&mut self)
    {
        let time = (std::time::Instant::now() - self.start_time).as_secs_f32();
        let brightness = (time.sin() + 1.0) / 2.0;

        self.triangle.uniform.update_data(Ubo{ brightness: brightness }).unwrap();

        self.gfx.draw_frame();
    }
}
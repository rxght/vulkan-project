use std::sync::Arc;
use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::event_loop::EventLoop;
use graphics::Graphics;
use crate::app::graphics::bindable;
use self::graphics::drawable::{DrawableEntry, self};

mod graphics;
mod drawables {
    pub mod triangle;
}

pub struct App
{
    triangle: DrawableEntry,

    gfx: Graphics
}

impl App
{
    pub fn new() -> (App, EventLoop<()>)
    {
        let (mut gfx, event_loop) = Graphics::new();
        (Self {

            triangle: drawables::triangle::new(&mut gfx, true),
            gfx: gfx

        }, event_loop)
    }
    
    pub fn resize_callback(&mut self)
    {
        self.gfx.recreate_swapchain();
    }

    pub fn run(&mut self)
    {
        self.gfx.draw_frame();
    }
}
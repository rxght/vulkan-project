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
    pub mod grid;
}

pub struct App
{
    grid: drawables::grid::Grid,
}

impl App
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        Self {
            grid: drawables::grid::Grid::new(gfx),
        }
    }
    
    pub fn resize_callback(&self)
    {
        
    }

    pub fn run(&self, gfx: &Graphics)
    {

    }
}
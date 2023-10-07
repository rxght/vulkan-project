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
    textest2: drawables::textest::TexturedSquare,
    vp: cgmath::Matrix4<f32>,
}

impl App
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        let window_extent = gfx.get_window().inner_size();
        let aspect = window_extent.width as f32 / window_extent.height as f32;
        Self {
            start_time: std::time::Instant::now(),
            textest: drawables::textest::TexturedSquare::new(gfx, true),
            textest2: drawables::textest::TexturedSquare::new(gfx, true),
            vp: cgmath::perspective(Deg(70.0), aspect, 0.2, 10.0) *
                cgmath::Matrix4::look_at_rh(
                    Point3{x: 0.0, y: 0.8, z: 1.5},
                    Point3{x: 0.0, y: 0.0, z: 0.0},
                    Vector3{x: 0.0, y: -1.0, z: 0.0}
                ),
        }
    }
    
    pub fn resize_callback(&mut self)
    {
        
    }

    pub fn run(&mut self)
    {
        let time = (std::time::Instant::now() - self.start_time).as_secs_f32();

        self.textest.pc.data.lock().unwrap().mvp = (self.vp * cgmath::Matrix4::from_angle_y(Rad(time.sin()))).into();

        self.textest2.pc.data.lock().unwrap().mvp = (self.vp * cgmath::Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: (time*2.0).sin()/4.0 })).into();
        
    }
}
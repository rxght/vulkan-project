use std::io::Write;
use std::sync::Arc;
use cgmath::Deg;
use cgmath::ElementWise;
use cgmath::Point3;
use cgmath::Rad;
use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Vector4;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::{event_loop::EventLoop, window::Window};
use crate::graphics::GlobalBindableId;
use crate::graphics::Graphics;
use crate::graphics::bindable;
use crate::graphics::bindable::UniformBuffer;
use crate::graphics::drawable::{DrawableEntry, self};
use crate::input::ButtonState;
use crate::input::Input;

use self::drawables::*;

static mut TEST: u32 = 0;

mod drawables {
    mod grid;
    mod point;

    pub use grid::Grid;
    pub use point::Point;
}

pub struct App
{
    input: Arc<Input>,
    //grid: drawables::Grid,
    point: drawables::Point,
}

impl App
{
    pub fn new(gfx: &mut Graphics, input: Arc<Input>) -> Self
    {

        Self {
            input: input,
            //grid: drawables::Grid::new(gfx, cgmath::Vector2{x: 5, y: 4}),
            point: drawables::Point::new(gfx),
        }
    }
    
    pub fn resize_callback(&self)
    {
        
    }

    pub fn run(&self, gfx: &Graphics)
    {
        self.point.data.access_data(|data| {
            data.point_position = [0.0, 0.0];
            data.radius = 5.0
        });

        //const MARGIN_PX: f32 = 50.0;
        //let window_size = gfx.get_window().inner_size();
        //let y_scaling = 2.0 / window_size.height as f32;
        //let x_scaling = 2.0 / window_size.width as f32;
        //let y = (window_size.height as f32 - 2.0 * MARGIN_PX) / self.grid.dimensions.y as f32;
        //let x = (window_size.width as f32 - 2.0 * MARGIN_PX) / self.grid.dimensions.x as f32;
        //let shared = f32::min(x, y);
        //self.grid.pc.access_data(|data| {
        //    data.scaling = [shared * x_scaling, shared * y_scaling];
        //});


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
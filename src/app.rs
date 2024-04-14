#![allow(unused_variables)]
mod drawables;

use cgmath::Vector2;
use crate::graphics::Graphics;
use crate::input::Input;
use std::sync::Arc;

use self::drawables::square::Square;

pub struct App {
    input: Arc<Input>,
    window_size: Vector2<f32>,
    square: Square,
}

impl App {
    pub fn new(gfx: &mut Graphics, input: Arc<Input>) -> Self {
        let window_extent = gfx.get_window().inner_size();
        let app = Self {
            input: input,
            window_size: Vector2 {
                x: window_extent.width as f32,
                y: window_extent.height as f32,
            },
            square: Square::new(gfx, [0.0, 0.0].into(), 50.0),
        };

        return app;
    }

    pub fn resize_callback(&mut self, gfx: &mut Graphics) {
        gfx.recreate_swapchain();

        let window_extent = gfx.get_window().inner_size();
        self.window_size = Vector2 {
            x: window_extent.width as f32,
            y: window_extent.height as f32,
        };
    }

    pub fn run(&mut self, gfx: &mut Graphics) {

    }
}

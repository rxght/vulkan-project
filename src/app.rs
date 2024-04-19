#![allow(unused_variables)]
mod drawables;

use crate::graphics::Graphics;
use crate::input::Input;
use cgmath::Vector2;
use std::sync::Arc;

use self::drawables::text_test::TextTest;

pub struct App {
    input: Arc<Input>,
    window_size: Vector2<f32>,
    glyph_test: TextTest,
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
            glyph_test: TextTest::new(gfx),
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

    pub fn run(&mut self, gfx: &mut Graphics) {}
}

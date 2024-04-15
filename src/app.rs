#![allow(unused_variables)]
mod drawables;

use cgmath::Vector2;
use crate::graphics::Graphics;
use crate::input::Input;
use std::sync::Arc;

use self::drawables::bezier::Bezier;
use self::drawables::circle::Circle;

pub struct App {
    input: Arc<Input>,
    window_size: Vector2<f32>,
    p_0: Circle,
    p_1: Circle,
    p_2: Circle,
    bezier: Bezier,
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
            p_0: Circle::new(gfx, [0.0, 0.0].into(), 4.0),
            p_1: Circle::new(gfx, [20.0, 20.0].into(), 4.0),
            p_2: Circle::new(gfx, [40.0, 0.0].into(), 4.0),
            bezier: Bezier::default(gfx),
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
        self.p_0.update(self.input.as_ref());
        self.p_1.update(self.input.as_ref());
        self.p_2.update(self.input.as_ref());
        self.bezier.p_0.set(self.p_0.position.get());
        self.bezier.p_1.set(self.p_1.position.get());
        self.bezier.p_2.set(self.p_2.position.get());
        self.bezier.update_vertices(gfx);
    }
}

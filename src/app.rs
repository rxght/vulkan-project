use cgmath::Vector3;
use vulkano::{sync::event::Event, buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, format};
use winit::event_loop::EventLoop;

use crate::app::graphics::{bindable, drawable::DrawableEntry};

use self::graphics::drawable;

mod graphics;


pub struct App
{
    gfx: graphics::Graphics,
    test_drawable: Option<DrawableEntry>
}

impl App
{
    pub fn new() -> (App, EventLoop<()>)
    {
        let (gfx, event_loop) =
            graphics::Graphics::new();
        (
            Self {
                gfx: gfx,
                test_drawable: None,
            },
            event_loop
        )
    }
    
    pub fn resize_callback(&mut self)
    {
        self.gfx.recreate_swapchain();
    }

    pub fn run(&mut self)
    {
        if self.test_drawable.is_none() {
            self.test_drawable = Some(drawable::GenericDrawable::new(&self.gfx, 0, || {
                vec![] // no per instance bindables necessary
            }, || {
                #[derive(BufferContents, Vertex)]
                #[repr(C)]
                struct Vertex {
                    #[format(R32G32_SFLOAT)]
                    pos: [f32; 2],
                }
                let vertices: Vec<Vertex> = vec![
                    Vertex{pos: [-0.5, -0.5]},
                    Vertex{pos: [ 0.0,  0.5]},
                    Vertex{pos: [ 0.5, -0.5]}
                ];
                let indices: Vec<u32> = vec![
                    0, 1, 2
                ];

                vec![
                    bindable::VertexShader::from_module(self.gfx.create_shader_module("shaders/first.vert")),
                    bindable::FragmentShader::from_module(self.gfx.create_shader_module("shaders/first.frag")),
                    bindable::IndexBuffer::new(&self.gfx, indices),
                    bindable::VertexBuffer::new(&self.gfx, vertices),
                ]
            }));
            self.gfx.register_drawable(self.test_drawable.as_mut().unwrap());
        }

        self.gfx.draw_frame();
    }
}
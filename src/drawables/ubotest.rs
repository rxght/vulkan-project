use std::sync::Arc;

use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable::{self, UniformBuffer}};

#[derive(BufferContents)]
#[repr(C)]
pub struct Ubo
{
    pub brightness: f32
}
pub struct UboTestDrawable
{
    entry: DrawableEntry,
    pub uniform: Arc<UniformBuffer<Ubo>>
}

impl UboTestDrawable
{
    pub fn new(gfx: &mut Graphics, create_registered: bool) -> Self
    {
        let uniform =
            bindable::UniformBuffer::new(gfx, 0, Ubo{ brightness: 1.0 });

        let mut entry = GenericDrawable::new(&gfx, 0, || {

            vec![ uniform.clone() ]
        }, || {
            #[derive(BufferContents, Vertex)]
            #[repr(C)]
            struct Vertex {
                #[format(R32G32_SFLOAT)]
                pub pos: [f32; 2],
                #[format(R32G32B32_SFLOAT)]
                pub col: [f32; 3],
            }
            let vertices: Vec<Vertex> = vec![
                Vertex{pos: [-0.5,  0.5], col: [1.0, 1.0, 0.0]},
                Vertex{pos: [ 0.0, -0.5], col: [0.0, 1.0, 1.0]},
                Vertex{pos: [ 0.5,  0.5], col: [1.0, 0.0, 1.0]}
            ];
            let indices: Vec<u32> = vec![
                0, 1, 2
            ];

            vec![
                bindable::VertexShader::from_module(gfx.create_shader_module("shaders/first.vert")),
                bindable::FragmentShader::from_module(gfx.create_shader_module("shaders/uniform_test.frag")),
                bindable::IndexBuffer::new(&gfx, indices),
                bindable::VertexBuffer::new(&gfx, vertices),
            ]
        });

        if create_registered {
            gfx.register_drawable(&mut entry);
        }
        
        Self {
            entry: entry,
            uniform: uniform
        }
    }
}
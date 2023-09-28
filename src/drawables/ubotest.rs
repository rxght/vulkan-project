use std::sync::Arc;

use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable::{self, UniformBuffer}, shaders::{frag_uniform_test, vert_first}};

pub use frag_uniform_test::ubo as Ubo;

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
            bindable::UniformBuffer::new(gfx, 0, Ubo{ brightness: 1.0 }, ShaderStages::FRAGMENT);

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
                bindable::VertexShader::from_module(vert_first::load(gfx.get_device()).unwrap()),
                bindable::FragmentShader::from_module(frag_uniform_test::load(gfx.get_device()).unwrap()),
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
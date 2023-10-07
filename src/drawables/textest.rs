use std::sync::Arc;

use cgmath::{SquareMatrix, Point3, Vector3, Deg};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable::{self, UniformBuffer, PushConstant}, shaders::{vert_first, frag_first, vert_textured, frag_textured}};

pub use vert_textured::Ubo;

pub struct TexturedSquare
{
    entry: DrawableEntry,
    pub pc: Arc<PushConstant<Ubo>>,
}

impl TexturedSquare {
    pub fn new(gfx: &mut Graphics, create_registered: bool) -> Self
    {

        let pc = PushConstant::new(gfx, 0, Ubo {
            mvp: cgmath::Matrix4::identity().into(),
        }, ShaderStages::VERTEX);

        let mut entry = GenericDrawable::new(&gfx, 0, || {
            vec![
                pc.clone()
            ]
        }, || {
            #[derive(BufferContents, Vertex)]
            #[repr(C)]
            struct Vertex {
                #[format(R32G32_SFLOAT)]
                pub pos: [f32; 2],
                #[format(R32G32_SFLOAT)]
                pub uv: [f32; 2],
            }
            let vertices: Vec<Vertex> = vec![
                Vertex{pos: [-0.5,  0.5], uv: [0.0, 0.0]},
                Vertex{pos: [-0.5, -0.5], uv: [0.0, 1.0]},
                Vertex{pos: [ 0.5, -0.5], uv: [1.0, 1.0]},
                Vertex{pos: [ 0.5,  0.5], uv: [1.0, 0.0]}
            ];
            let indices: Vec<u32> = vec![
                0, 1, 2,
                0, 2, 3
            ];

            vec![
                bindable::VertexShader::from_module(vert_textured::load(gfx.get_device()).unwrap()),
                bindable::FragmentShader::from_module(frag_textured::load(gfx.get_device()).unwrap()),
                bindable::IndexBuffer::new(gfx, indices),
                bindable::VertexBuffer::new(gfx, vertices),
                bindable::Texture::new(gfx, "textures/batako.png", 0, 0),
            ]
        });

        if create_registered {
            gfx.register_drawable(&mut entry);
        }

        Self {
            entry: entry,
            pc: pc,
        }
    }
}
use std::sync::Arc;

use cgmath::Vector2;
use vulkano::{
    buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages,
};

use crate::graphics::{
    bindable::{self, PushConstant},
    drawable::{DrawableEntry, GenericDrawable},
    shaders::{frag_solid_white, vert_cartesian_2d},
    Graphics,
};

pub struct Square {
    entry: DrawableEntry,
    pub transform: Arc<PushConstant<vert_cartesian_2d::Data>>,
}

impl Square {
    pub fn new(gfx: &mut Graphics, pos: Vector2<f32>, radius: f32) -> Self {
        let data = PushConstant::new(
            gfx,
            0,
            vert_cartesian_2d::Data {
                transform: (cgmath::Matrix4::from_translation(cgmath::Vector3 {
                    x: pos.x,
                    y: pos.y,
                    z: 0.0,
                }) * cgmath::Matrix4::from_scale(radius))
                .into(),
            },
            ShaderStages::VERTEX,
        );

        let mut entry = GenericDrawable::new(
            gfx,
            5,
            || vec![data.clone()],
            || {
                #[derive(BufferContents, Vertex)]
                #[repr(C)]
                struct Vertex {
                    #[format(R32G32_SFLOAT)]
                    pos: [f32; 2],
                }

                let vertices = vec![
                    Vertex { pos: [-1.0, -1.0] },
                    Vertex { pos: [1.0, -1.0] },
                    Vertex { pos: [-1.0, 1.0] },
                    Vertex { pos: [1.0, 1.0] },
                ];

                let indices: Vec<u32> = vec![0, 3, 1, 0, 2, 3];

                vec![
                    bindable::VertexBuffer::new(gfx, vertices),
                    bindable::IndexBuffer::new(gfx, indices),
                    bindable::VertexShader::from_module(
                        vert_cartesian_2d::load(gfx.get_device()).unwrap(),
                    ),
                    bindable::FragmentShader::from_module(
                        frag_solid_white::load(gfx.get_device()).unwrap(),
                    ),
                    gfx.get_utils().cartesian_to_normalized.clone(),
                ]
            },
        );

        gfx.register_drawable(&mut entry);

        Self {
            entry: entry,
            transform: data,
        }
    }
}

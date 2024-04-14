use std::sync::Arc;

use cgmath::{SquareMatrix, Vector2};
use vulkano::{
    buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages,
};

use crate::graphics::{
    bindable::{self, PushConstant},
    drawable::{DrawableEntry, GenericDrawable},
    shaders::{vert_cartesian_2d, frag_solid_white},
    Graphics,
};

pub struct Circle {
    pub postition: Vector2<f32>,
    pub radius: f32,

    entry: DrawableEntry,
    transform: Arc<PushConstant<vert_cartesian_2d::Data>>,
}

impl Circle {
    pub fn new(gfx: &mut Graphics, pos: Vector2<f32>, radius: f32) -> Self {
        let data = PushConstant::new(
            gfx,
            0,
            vert_cartesian_2d::Data {
                transform: cgmath::Matrix4::identity().into(),
            },
            ShaderStages::VERTEX,
        );

        let mut entry = GenericDrawable::new(
            gfx,
            || vec![
                data.clone()
            ],
            shared_bindables
        );

        gfx.register_drawable(&mut entry);

        Self {
            postition: pos,
            radius: radius,
            entry: entry,
            transform: data,
        }
    }
}

fn shared_bindables(gfx: &Graphics) -> Vec<Arc<dyn bindable::Bindable>>
{
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
}
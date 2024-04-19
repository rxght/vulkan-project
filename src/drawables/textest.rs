use std::sync::Arc;

use cgmath::SquareMatrix;
use vulkano::{
    buffer::BufferContents, pipeline::graphics::vertex_input::Vertex, shader::ShaderStages,
};

use crate::graphics::{
    bindable::{self, PushConstant},
    drawable::{DrawableEntry, GenericDrawable},
    shaders::{frag_textured, vert_textured},
    Graphics,
};

pub use vert_textured::GlobalUbo;
pub use vert_textured::Pc;

pub struct TexturedSquare {
    entry: DrawableEntry,
    pub pc: Arc<PushConstant<Pc>>,
}

impl TexturedSquare {
    pub fn new(gfx: &mut Graphics, create_registered: bool) -> Self {
        let pc = PushConstant::new(
            gfx,
            0,
            Pc {
                model: cgmath::Matrix4::identity().into(),
            },
            ShaderStages::VERTEX,
        );

        let mut entry = GenericDrawable::new(&gfx, || vec![pc.clone()], shared_bindables);

        if create_registered {
            gfx.register_drawable(&mut entry);
        }

        Self {
            entry: entry,
            pc: pc,
        }
    }
}

fn shared_bindables(gfx: &Graphics) -> Vec<Arc<dyn bindable::Bindable>> {
    #[derive(BufferContents, Vertex)]
    #[repr(C)]
    struct Vertex {
        #[format(R32G32_SFLOAT)]
        pub pos: [f32; 2],
        #[format(R32G32_SFLOAT)]
        pub uv: [f32; 2],
    }
    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: [-0.5, 0.5],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [-0.5, -0.5],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [0.5, -0.5],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [0.5, 0.5],
            uv: [1.0, 0.0],
        },
    ];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

    vec![
        bindable::VertexShader::from_module(vert_textured::load(gfx.get_device()).unwrap()),
        bindable::FragmentShader::from_module(frag_textured::load(gfx.get_device()).unwrap()),
        bindable::IndexBuffer::new(gfx, indices),
        bindable::VertexBuffer::new(gfx, vertices),
        bindable::Texture::new(gfx, "textures/batako.png", 1, 0),
    ]
}

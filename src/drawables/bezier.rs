use std::{cell::Cell, sync::Arc};

use cgmath::{SquareMatrix, Vector2};
use vulkano::{
    buffer::BufferContents,
    pipeline::graphics::{
        input_assembly::{InputAssemblyState, PrimitiveTopology},
        vertex_input,
    },
    shader::ShaderStages,
};

use crate::graphics::{
    bindable::{self, IndexBuffer, MutableVertexBuffer, PushConstant},
    drawable::{DrawableEntry, GenericDrawable},
    pipeline::PipelineBuilder,
    shaders::{frag_solid_white, vert_cartesian_2d},
    Graphics,
};

#[derive(BufferContents, vertex_input::Vertex)]
#[repr(C)]
struct Vertex {
    #[format(R32G32_SFLOAT)]
    pos: [f32; 2],
}

pub struct Bezier {
    pub p_0: Cell<Vector2<f32>>,
    pub p_1: Cell<Vector2<f32>>,
    pub p_2: Cell<Vector2<f32>>,

    vertex_buffer: Arc<MutableVertexBuffer<Vertex>>,
    segment_count: u32,
    entry: DrawableEntry,
}

impl Bezier {
    pub fn new(
        gfx: &mut Graphics,
        p_0: Vector2<f32>,
        p_1: Vector2<f32>,
        p_2: Vector2<f32>,
    ) -> Self {
        let data = PushConstant::new(
            gfx,
            0,
            vert_cartesian_2d::Data {
                transform: cgmath::Matrix4::identity().into(),
            },
            ShaderStages::VERTEX,
        );

        let segment_count = 10;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(segment_count + 1);
        let mut indices: Vec<u32> = Vec::with_capacity(segment_count + 1);

        for i in 0..=segment_count as u32 {
            let t = i as f32 / segment_count as f32;
            let vertex = t * t * (p_0 - 2.0 * p_1 + p_2) + 2.0 * t * (-p_0 + p_1) + p_0;
            vertices.push(Vertex { pos: vertex.into() });
            indices.push(i);
        }

        let vertex_buffer = MutableVertexBuffer::new(gfx, vertices);

        let mut entry = GenericDrawable::new(
            gfx,
            || {
                vec![
                    data.clone(),
                    vertex_buffer.clone(),
                    IndexBuffer::new(gfx, indices),
                ]
            },
            shared_bindables,
        );

        gfx.register_drawable(&mut entry);

        Self {
            p_0: Cell::new(p_0),
            p_1: Cell::new(p_1),
            p_2: Cell::new(p_2),

            vertex_buffer: vertex_buffer,
            segment_count: segment_count as u32,
            entry: entry,
        }
    }
    pub fn default(gfx: &mut Graphics) -> Self {
        return Self::new(gfx, [0.0, 0.0].into(), [0.0, 0.0].into(), [0.0, 0.0].into());
    }
    pub fn update_vertices(&self, gfx: &Graphics) {
        self.vertex_buffer.write_vertices(gfx, |iter| {
            for (i, vertex) in iter.enumerate() {
                let t = i as f32 / self.segment_count as f32;
                let p_0 = self.p_0.get();
                let p_1 = self.p_1.get();
                let p_2 = self.p_2.get();
                let new_vertex = t * t * (p_0 - 2.0 * p_1 + p_2) + 2.0 * t * (-p_0 + p_1) + p_0;
                vertex.pos = new_vertex.into();
            }
        })
    }
}

fn shared_bindables(gfx: &Graphics) -> Vec<Arc<dyn bindable::Bindable>> {
    use bindable::*;
    vec![
        VertexShader::from_module(vert_cartesian_2d::load(gfx.get_device()).unwrap()),
        FragmentShader::from_module(frag_solid_white::load(gfx.get_device()).unwrap()),
        bindable::GodBindable::new(
            |cmd, pipeline_layout| {},
            |builder: &mut PipelineBuilder, _| {
                builder.input_assembly_state =
                    InputAssemblyState::default().topology(PrimitiveTopology::LineStrip);
            },
        ),
        gfx.get_utils().cartesian_to_normalized.clone(),
    ]
}

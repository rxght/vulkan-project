use std::sync::Arc;

use cgmath::{SquareMatrix, Vector2};
use vulkano::{
    buffer::BufferContents,
    pipeline::graphics::{
        input_assembly::{InputAssemblyState, PrimitiveTopology},
        vertex_input,
    },
    shader::ShaderStages,
};

use crate::{
    font::FontLoader,
    graphics::{
        bindable::{self, IndexBuffer, PushConstant, VertexBuffer},
        drawable::{DrawableEntry, GenericDrawable},
        pipeline::PipelineBuilder,
        shaders::{frag_solid_white, vert_cartesian_2d},
        Graphics,
    },
};

#[derive(BufferContents, vertex_input::Vertex)]
#[repr(C)]
struct Vertex {
    #[format(R32G32_SFLOAT)]
    pos: [f32; 2],
}

pub struct TextTest {
    entry: DrawableEntry,
}

impl TextTest {
    pub fn new(gfx: &mut Graphics) -> Self {
        let data = PushConstant::new(
            gfx,
            0,
            vert_cartesian_2d::Data {
                transform: cgmath::Matrix4::identity().into(),
            },
            ShaderStages::VERTEX,
        );

        let text = "text";
        let font = "fonts/mono_test.ttf";

        let font_loader = FontLoader::new();
        let font = font_loader.load_ttf(font).unwrap();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut x_offset = -400.0;
        for c in text.chars() {
            let glyph = font.get_glyph(c);

            for outline in &glyph.contours {
                let outline_offset = vertices.len() as u32;
                let outline_vertices = crate::font::outline::calc_contour_vertices(&outline);

                let offset_vector = Vector2 {
                    x: x_offset,
                    y: 0.0,
                };
                vertices.extend(outline_vertices.iter().map(|&p| Vertex {
                    pos: (p + offset_vector).into(),
                }));

                // create indices
                for i in 1..outline_vertices.len() as u32 {
                    let i = i + outline_offset;
                    indices.push(i - 1);
                    indices.push(i);
                }
                indices.push(vertices.len() as u32 - 1);
                indices.push(outline_offset);
            }
            x_offset += 120.0;
        }

        let mut entry = GenericDrawable::new(
            gfx,
            || {
                vec![
                    data.clone(),
                    VertexBuffer::new(gfx, vertices),
                    IndexBuffer::new(gfx, indices),
                ]
            },
            shared_bindables,
        );

        gfx.register_drawable(&mut entry);

        Self { entry: entry }
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
                    InputAssemblyState::default().topology(PrimitiveTopology::LineList);
            },
        ),
        gfx.get_utils().cartesian_to_normalized.clone(),
    ]
}

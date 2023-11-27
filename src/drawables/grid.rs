use std::sync::Arc;

use cgmath::Vector2;
use vulkano::{buffer::BufferContents, pipeline::graphics::{vertex_input::Vertex, input_assembly::{InputAssemblyState, PrimitiveTopology}}, format, shader::ShaderStages};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable::{self, PushConstant}, shaders::{vert_first, frag_first, frag_line_segment, vert_2d}};

pub struct Grid
{
    entry: DrawableEntry,
    pub pc: Arc<PushConstant<vert_2d::Pc>>,
    pub dimensions: Vector2<u32>
}

const SQUARE_WIDTH: f32 = 1.0;

const MARGIN_FROM_EDGE: f32 = 0.1;

impl Grid
{
    pub fn new(gfx: &mut Graphics, dimensions: Vector2<u32>) -> Self
    {
        let pc =
            bindable::PushConstant::new(gfx, 0, vert_2d::Pc{scaling: [0.5, 0.5]}, ShaderStages::VERTEX);

        let mut entry = GenericDrawable::new(&gfx, 0, || {
            vec![
                pc.clone()
            ] // no per instance bindables necessary
        }, || {
            let window_size = gfx.get_window().inner_size();

            let mut vertices: Vec<Vertex> = Vec::with_capacity((dimensions.x * 2 + dimensions.y * 2) as usize);
            let mut indices: Vec<u32> = Vec::with_capacity(((dimensions.x + 1) * 2 + (dimensions.y + 1) * 2) as usize);

            let half_width = SQUARE_WIDTH * dimensions.x as f32 / 2.0;
            let half_height = SQUARE_WIDTH * dimensions.y as f32 / 2.0;

            #[derive(BufferContents, Vertex)]
            #[repr(C)]
            struct Vertex {
                #[format(R32G32_SFLOAT)]
                position: [f32; 2],
            }

            // The outer square
            vertices.push(Vertex{ position: [ half_width,  half_height]});
            vertices.push(Vertex{ position: [-half_width,  half_height]});
            vertices.push(Vertex{ position: [ half_width, -half_height]});
            vertices.push(Vertex{ position: [-half_width, -half_height]});
            indices.extend([0,1,0,2,2,3,1,3].iter());

            // grid lines
            for x in 1..dimensions.x
            {
                vertices.push(Vertex { position: [half_width - x as f32,  half_height] });
                vertices.push(Vertex { position: [half_width - x as f32, -half_height] });
            }
            for y in 1..dimensions.y
            {
                vertices.push(Vertex { position: [ half_width, half_height - y as f32] });
                vertices.push(Vertex { position: [-half_width, half_height - y as f32] });
            }

            // fill indices
            indices.extend((4..vertices.len() as u32).into_iter());

            vec![
                bindable::VertexShader::from_module(vert_2d::load(gfx.get_device()).unwrap()),
                bindable::FragmentShader::from_module(frag_line_segment::load(gfx.get_device()).unwrap()),
                bindable::IndexBuffer::new(&gfx, indices),
                bindable::VertexBuffer::new(&gfx, vertices),
                bindable::GodBindable::new( |_, _| {},
                    |pipeline_builder, _|
                    {
                        pipeline_builder.input_assembly_state = InputAssemblyState::new().topology(PrimitiveTopology::LineList);
                    }
                )
            ]
        });

        gfx.register_drawable(&mut entry);

        Self{
            entry: entry,
            pc: pc,
            dimensions: dimensions,
        }
    }
}
use std::sync::Arc;

use bytemuck::Zeroable;
use vulkano::{buffer::BufferContents, shader::ShaderStages, pipeline::graphics::{vertex_input::Vertex, input_assembly::{InputAssemblyState, PrimitiveTopology}}};

use crate::graphics::{Graphics, bindable::{UniformBuffer, self, PushConstant}, drawable::{DrawableEntry, GenericDrawable}, shaders::{vert_2d, vert_point, frag_solid_color}};

pub struct Point
{
    entry: DrawableEntry,
    pub data: Arc<PushConstant<vert_point::PointData>>
}

impl Point
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        let data =
            PushConstant::new(gfx, 0, vert_point::PointData{point_position:[0.0, 0.0], radius: 10.0}, ShaderStages::VERTEX);
        
        let mut entry = GenericDrawable::new(gfx, 5,
            || {
                vec![ data.clone() ]
            },
            || {
                #[derive(BufferContents, Vertex)]
                #[repr(C)]
                struct Vertex {
                    #[format(R32G32_SFLOAT)]
                    position: [f32; 2],
                }

                let vertices = vec![
                    Vertex{position: [-1.0, -1.0]},
                    Vertex{position: [ 1.0, -1.0]},
                    Vertex{position: [-1.0,  1.0]},
                    Vertex{position: [ 1.0,  1.0]},
                ];

                let indices: Vec<u32> = vec![
                    0, 3, 1,
                    1, 2, 3,
                ];

                vec![
                    bindable::VertexBuffer::new(gfx, vertices),
                    bindable::IndexBuffer::new(gfx, indices),
                    bindable::VertexShader::from_module(vert_point::load(gfx.get_device()).unwrap()),
                    bindable::FragmentShader::from_module(frag_solid_color::load(gfx.get_device()).unwrap()),
                    gfx.get_global_bindable(crate::graphics::GlobalBindableId::CartesianToNormalized),
                ]
            }
        );

        gfx.register_drawable(&mut entry);

        Self {
            entry: entry,
            data: data,
        }
    }
}
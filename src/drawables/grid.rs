use cgmath::Vector2;
use vulkano::{buffer::BufferContents, pipeline::graphics::{vertex_input::Vertex, input_assembly::{InputAssemblyState, PrimitiveTopology}}, format};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable, shaders::{vert_first, frag_first, frag_line_segment, vert_2d}};

const SQUARE_WIDTH: u32 = 100;
const GRID_SIZE: Vector2<u32> = Vector2{ x: 5, y: 3 };

pub struct Grid
{
    entry: DrawableEntry,
}

impl Grid
{
    pub fn new(gfx: &mut Graphics) -> Self
    {
        let mut entry = GenericDrawable::new(&gfx, 0, || {
            vec![] // no per instance bindables necessary
        }, || {
            let window_size = gfx.get_window().inner_size();

            let x_offset = (window_size.width - SQUARE_WIDTH * GRID_SIZE.x) as f32 / window_size.width as f32;
            let y_offset = (window_size.height - SQUARE_WIDTH * GRID_SIZE.y) as f32 / window_size.height as f32;

            let square_width = 2.0 * SQUARE_WIDTH as f32 / window_size.width as f32;
            let square_height = 2.0 * SQUARE_WIDTH as f32 / window_size.height as f32;

            let mut vertices = Vec::with_capacity((GRID_SIZE.x * 2 + GRID_SIZE.y * 2) as usize);
            let mut indices = Vec::with_capacity(((GRID_SIZE.x + 1) * 2 + (GRID_SIZE.y + 1) * 2) as usize);

            #[derive(BufferContents, Vertex)]
            #[repr(C)]
            struct Vertex {
                #[format(R32G32_SFLOAT)]
                position: [f32; 2],
            }

            // The outer square
            vertices.push(Vertex{ position: [ (x_offset - 1.0),  (y_offset - 1.0)]});
            vertices.push(Vertex{ position: [-(x_offset - 1.0),  (y_offset - 1.0)]});
            vertices.push(Vertex{ position: [ (x_offset - 1.0), -(y_offset - 1.0)]});
            vertices.push(Vertex{ position: [-(x_offset - 1.0), -(y_offset - 1.0)]});
            indices.extend([0,1,0,2,2,3,1,3].iter());

            let mut index_offset = 4;

            // grid lines

            for x in 1..GRID_SIZE.x {
                vertices.push(Vertex{ position: [(x_offset - 1.0) + x as f32 * square_width,  (y_offset - 1.0)]});
                vertices.push(Vertex{ position: [(x_offset - 1.0) + x as f32 * square_width, -(y_offset - 1.0)]});
                indices.extend([0 + index_offset, 1 + index_offset].iter());
                index_offset += 2;
            }

            for y in 1..GRID_SIZE.y {
                vertices.push(Vertex{ position: [ (x_offset - 1.0), (y_offset - 1.0) + y as f32 * square_height]});
                vertices.push(Vertex{ position: [-(x_offset - 1.0), (y_offset - 1.0) + y as f32 * square_height]});
                indices.extend([0 + index_offset, 1 + index_offset].iter());
                index_offset += 2;
            }

            vec![
                bindable::VertexShader::from_module(vert_2d::load(gfx.get_device()).unwrap()),
                bindable::FragmentShader::from_module(frag_line_segment::load(gfx.get_device()).unwrap()),
                bindable::IndexBuffer::new(&gfx, indices),
                bindable::VertexBuffer::new(&gfx, vertices),
                bindable::GodBindable::new( |_, _| {},
                    |pipeline_builder, _|
                    {
                        println!("sup!");
                        pipeline_builder.input_assembly_state = InputAssemblyState::new().topology(PrimitiveTopology::LineList);
                    }
                ),
            ]
        });

        gfx.register_drawable(&mut entry);

        Self{entry: entry}
    }
}
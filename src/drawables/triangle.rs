use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable, shaders::{vert_first, frag_first}};


pub fn new(gfx: &mut Graphics, create_registered: bool) -> DrawableEntry
{
    let mut entry = GenericDrawable::new(&gfx, 0, || {
        vec![] // no per instance bindables necessary
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
            bindable::FragmentShader::from_module(frag_first::load(gfx.get_device()).unwrap()),
            bindable::IndexBuffer::new(&gfx, indices),
            bindable::VertexBuffer::new(&gfx, vertices),
        ]
    });

    if create_registered {
        gfx.register_drawable(&mut entry);
    }

    entry
}
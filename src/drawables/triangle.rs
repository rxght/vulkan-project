use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::app::graphics::{drawable::{GenericDrawable, DrawableEntry}, Graphics, bindable};


pub fn new(gfx: &mut Graphics, create_registered: bool) -> DrawableEntry
{
    let mut entry = GenericDrawable::new(&gfx, 0, || {
        vec![] // no per instance bindables necessary
    }, || {
        #[derive(BufferContents, Vertex)]
        #[repr(C)]
        struct Vertex {
            #[format(R32G32_SFLOAT)]
            pos: [f32; 2],
        }
        let vertices: Vec<Vertex> = vec![
            Vertex{pos: [-0.5,  0.5]},
            Vertex{pos: [ 0.0, -0.5]},
            Vertex{pos: [ 0.5,  0.5]}
        ];
        let indices: Vec<u32> = vec![
            0, 1, 2
        ];

        vec![
            bindable::VertexShader::from_module(gfx.create_shader_module("shaders/first.vert")),
            bindable::FragmentShader::from_module(gfx.create_shader_module("shaders/first.frag")),
            bindable::IndexBuffer::new(&gfx, indices),
            bindable::VertexBuffer::new(&gfx, vertices),
        ]
    });

    if create_registered {
        gfx.register_drawable(&mut entry);
    }

    entry
}
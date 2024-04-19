use std::{cell::Cell, sync::Arc};

use cgmath::{MetricSpace, SquareMatrix, Vector2};
use vulkano::{
    buffer::BufferContents,
    pipeline::graphics::{color_blend::ColorBlendState, vertex_input::Vertex},
    shader::ShaderStages,
};

use crate::{
    graphics::{
        bindable::{self, PushConstant},
        drawable::{DrawableEntry, GenericDrawable},
        pipeline::PipelineBuilder,
        shaders::{frag_circle, vert_circle},
        Graphics,
    },
    input::{ButtonState, Input},
};

pub struct Circle {
    pub position: Cell<Vector2<f32>>,
    pub radius: Cell<f32>,

    is_grabbed: Cell<bool>,
    entry: DrawableEntry,
    transform: Arc<PushConstant<vert_circle::Data>>,
}

impl Circle {
    pub fn new(gfx: &mut Graphics, pos: Vector2<f32>, radius: f32) -> Self {
        let data = PushConstant::new(
            gfx,
            0,
            vert_circle::Data {
                transform: cgmath::Matrix4::identity().into(),
            },
            ShaderStages::VERTEX,
        );

        let mut entry = GenericDrawable::new(gfx, || vec![data.clone()], shared_bindables);

        gfx.register_drawable(&mut entry);

        let circle = Self {
            position: Cell::new(pos),
            radius: Cell::new(radius),
            entry: entry,
            is_grabbed: Cell::new(false),
            transform: data,
        };

        circle.update_transform();
        return circle;
    }

    pub fn update_transform(&self) {
        self.transform.access_data(|data| {
            let pos = self.position.get();
            data.transform = (cgmath::Matrix4::from_translation([pos.x, pos.y, 0.0].into())
                * cgmath::Matrix4::from_scale(self.radius.get()))
            .into()
        })
    }

    pub fn update(&self, input: &Input) {
        match self.is_grabbed.get() {
            false => {
                if input.mouse.is_button_pressed(1) {
                    let distance = input
                        .mouse
                        .cursor_position
                        .get()
                        .distance(self.position.get());
                    if distance < self.radius.get() {
                        self.is_grabbed.set(true);
                    }
                }
            }
            true => {
                match input.mouse.get_button_state(1).unwrap() {
                    ButtonState::Held(_) => {
                        //update_position
                        self.position.set(input.mouse.cursor_position.get());
                        self.update_transform();
                    }
                    ButtonState::Released => {
                        self.position.set(input.mouse.cursor_position.get());
                        self.update_transform();
                        self.is_grabbed.set(false);
                    }
                    ButtonState::Pressed(_) => {
                        // wtf?
                    }
                }
            }
        }
    }
}

fn shared_bindables(gfx: &Graphics) -> Vec<Arc<dyn bindable::Bindable>> {
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
        bindable::GodBindable::new(
            |cmd, pipeline_layout| {},
            |builder: &mut PipelineBuilder, _| {
                builder.color_blend_state = ColorBlendState::default().blend_alpha();
            },
        ),
        bindable::VertexBuffer::new(gfx, vertices),
        bindable::IndexBuffer::new(gfx, indices),
        bindable::VertexShader::from_module(vert_circle::load(gfx.get_device()).unwrap()),
        bindable::FragmentShader::from_module(frag_circle::load(gfx.get_device()).unwrap()),
        gfx.get_utils().cartesian_to_normalized.clone(),
    ]
}

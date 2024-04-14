use std::sync::Arc;

use bytemuck::Zeroable;
use vulkano::shader::ShaderStages;

use crate::graphics::bindable::UniformBuffer;

use super::Graphics;

#[derive(Clone, Copy, Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct MatrixUbo {
    pub matrix: [[f32; 4]; 4],
}

pub struct Utils {
    pub perspective_projection: Arc<UniformBuffer<MatrixUbo>>,
    pub cartesian_to_normalized: Arc<UniformBuffer<MatrixUbo>>,
}

impl Utils {
    pub fn new(gfx: &Graphics) -> Self {
        let window_extent = gfx.get_window().inner_size();
        let aspect = window_extent.width as f32 / window_extent.height as f32;
        const MAX_DEPTH: f32 = 10.0;

        let perspective_projection = UniformBuffer::new(
            gfx,
            0,
            MatrixUbo {
                matrix: (cgmath::perspective(cgmath::Deg(60.0), aspect, 0.1, MAX_DEPTH)
                    * cgmath::Matrix4::look_at_rh(
                        cgmath::Point3 {
                            x: 0.0,
                            y: 0.8,
                            z: 1.5,
                        },
                        cgmath::Point3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        cgmath::Vector3 {
                            x: 0.0,
                            y: -1.0,
                            z: 0.0,
                        },
                    ))
                .into(),
            },
            ShaderStages::VERTEX,
        );

        let cartesian_to_normalized = UniformBuffer::new(
            gfx,
            0,
            MatrixUbo {
                matrix: cgmath::ortho(
                    -((window_extent.width / 2) as f32),
                    (window_extent.width / 2) as f32,
                    (window_extent.height / 2) as f32,
                    -((window_extent.height / 2) as f32),
                    -MAX_DEPTH,
                    MAX_DEPTH,
                )
                .into(),
            },
            ShaderStages::VERTEX,
        );

        Self {
            perspective_projection: perspective_projection,
            cartesian_to_normalized: cartesian_to_normalized,
        }
    }

    pub fn recreate(&self, gfx: &Graphics) {
        let window_extent = gfx.get_window().inner_size();
        let aspect = window_extent.width as f32 / window_extent.height as f32;
        const MAX_DEPTH: f32 = 10.0;

        self.perspective_projection.access_data(|data| {
            data.matrix = (cgmath::perspective(cgmath::Deg(60.0), aspect, 0.1, MAX_DEPTH)
                * cgmath::Matrix4::look_at_rh(
                    cgmath::Point3 {
                        x: 0.0,
                        y: 0.8,
                        z: 1.5,
                    },
                    cgmath::Point3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    cgmath::Vector3 {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                ))
            .into();
        });

        self.cartesian_to_normalized.access_data(|data| {
            data.matrix = cgmath::ortho(
                -((window_extent.width / 2) as f32),
                (window_extent.width / 2) as f32,
                (window_extent.height / 2) as f32,
                -((window_extent.height / 2) as f32),
                -MAX_DEPTH,
                MAX_DEPTH,
            )
            .into()
        });
    }
}

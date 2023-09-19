use vulkano::{pipeline::graphics::{GraphicsPipelineBuilder, vertex_input::VertexInputState}, shader::ShaderModule};

use super::pipeline::PipelineBuilder;

mod buffer;
mod shader;

pub use buffer::*;
pub use shader::*;

pub trait Bindable
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder);
    fn bind(&self, gfx: &crate::app::graphics::Graphics) {}
}
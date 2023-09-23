use vulkano::{shader::ShaderModule, command_buffer::{AutoCommandBufferBuilder, allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer}};

use super::{pipeline::PipelineBuilder, Graphics};

mod buffer;
mod shader;

pub use buffer::*;
pub use shader::*;

pub trait Bindable
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32);
    fn bind(&self, _gfx: &Graphics,
        _builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>
    ) {}
}
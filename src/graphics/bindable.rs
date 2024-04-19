use std::sync::Arc;

use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        PrimaryAutoCommandBuffer,
    },
    pipeline::PipelineLayout,
    shader::ShaderModule,
};

use super::{pipeline::PipelineBuilder, Graphics};

mod buffer;
mod god_bindable;
mod mutable_buffer;
mod push_constant;
mod shader;
mod texture;
mod uniform;

pub use buffer::*;
pub use god_bindable::*;
pub use mutable_buffer::*;
pub use push_constant::*;
pub use shader::*;
pub use texture::*;
pub use uniform::*;

pub trait Bindable {
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32);
    fn bind(
        &self,
        _gfx: &Graphics,
        _builder: &mut AutoCommandBufferBuilder<
            PrimaryAutoCommandBuffer,
            StandardCommandBufferAllocator,
        >,
        _pipeline_layout: Arc<PipelineLayout>,
    ) {
    }
}

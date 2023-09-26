use std::sync::Arc;

use vulkano::{
    buffer::{BufferContents, Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::AllocationCreateInfo, pipeline::graphics::vertex_input::Vertex,
    command_buffer::{
        AutoCommandBufferBuilder, allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer
    }
};

use crate::app::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;
pub struct VertexBuffer<T> 
    where
    T: Vertex + BufferContents
{
    subbuffer: Subbuffer<[T]>
}

impl<T> Bindable for VertexBuffer<T>
    where
    T: Vertex + BufferContents
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder,
        _index_count: &mut u32)
    {
        builder.vertex_buffer_description = Some(T::per_vertex());
    }
    
    fn bind(&self, _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>
    ) {
        builder.bind_vertex_buffers(0, self.subbuffer.clone());
    }
}

impl<T> VertexBuffer<T>
    where
    T: Vertex + BufferContents
{
    pub fn new(gfx: &Graphics, vertices: Vec<T>) -> Arc<Self>
    where
        T: Vertex + BufferContents
    {
        Arc::new(Self {
            subbuffer: Buffer::from_iter(gfx.get_allocator(), BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            }, AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            }, vertices.into_iter()
            )
                .expect("Failed to create index buffer.")
        })
    }
}

pub struct IndexBuffer
{
    subbuffer: Subbuffer<[u32]>
}

impl Bindable for IndexBuffer
{
    fn bind_to_pipeline(&self, _builder: &mut PipelineBuilder,
        index_count: &mut u32)
    {
        *index_count = self.subbuffer.len().try_into().unwrap();
    }
    fn bind(&self, _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>
    ) {
        builder.bind_index_buffer(self.subbuffer.clone());
    }
}

impl IndexBuffer
{
    pub fn new(gfx: &Graphics, indices: Vec<u32>) -> Arc<Self>
    {
        Arc::new(Self {
            subbuffer: Buffer::from_iter(gfx.get_allocator(), BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            }, AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            }, indices.into_iter()
            )
                .expect("Failed to create index buffer.")
        })
    }
}
use std::sync::Arc;

use vulkano::{buffer::{BufferContents, Buffer, BufferCreateInfo, BufferCreateFlags, BufferUsage, Subbuffer}, memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo}, pipeline::graphics::vertex_input::Vertex, format, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, allocator::StandardCommandBufferAllocator}};

use crate::app::graphics::Graphics;

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
    fn bind_to_pipeline(&self, builder: &mut crate::app::graphics::pipeline::PipelineBuilder) {
        builder.vertex_buffer_description = Some(T::per_vertex());
    }
    
    fn bind(&self, gfx: &crate::app::graphics::Graphics) {
        
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
                usage: vulkano::memory::allocator::MemoryUsage::DeviceOnly,
                ..Default::default()
            }, vertices.into_iter()
            )
                .expect("Failed to create index buffer.")
        })
    }
}

pub struct IndexBuffer
{
    subbuffer: Subbuffer<[u8]>
}

impl Bindable for IndexBuffer
{
    fn bind_to_pipeline(&self, builder: &mut crate::app::graphics::pipeline::PipelineBuilder) {}
}

#[derive(BufferContents)]
#[repr(C)]
pub struct BufferContentWrapper<T> { index: T }
impl<T> BufferContentWrapper<T>
{
    pub fn new(val: T) -> Self {
        Self {
            index: val
        }
    }
}

impl IndexBuffer
{
    pub fn new<T>(gfx: &Graphics, indices: Vec<T>) -> Arc<Self>
    where
        BufferContentWrapper<T>: BufferContents
    {
        Arc::new(Self {
            subbuffer: Buffer::from_iter(gfx.get_allocator(), BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            }, AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::DeviceOnly,
                ..Default::default()
            }, indices.into_iter().map(|p| BufferContentWrapper::new(p))
            )
                .expect("Failed to create index buffer.")
                .into_bytes()
        })
    }
}
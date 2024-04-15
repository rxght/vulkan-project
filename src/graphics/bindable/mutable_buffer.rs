use std::{mem::align_of, slice::IterMut, sync::Arc};

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        CopyBufferInfoTyped, PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract,
    },
    memory::allocator::{AllocationCreateInfo, DeviceLayout},
    pipeline::{graphics::vertex_input::Vertex, PipelineLayout},
    sync::GpuFuture,
};

use crate::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;
pub struct MutableVertexBuffer<T>
where
    T: Vertex + BufferContents,
{
    subbuffer: Subbuffer<[T]>,
    staging_buffer: Subbuffer<[T]>,
}

impl<T> Bindable for MutableVertexBuffer<T>
where
    T: Vertex + BufferContents,
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, _index_count: &mut u32) {
        builder.vertex_buffer_description = Some(T::per_vertex());
    }

    fn bind(
        &self,
        _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<
            PrimaryAutoCommandBuffer,
            StandardCommandBufferAllocator,
        >,
        _: Arc<PipelineLayout>,
    ) {
        builder.bind_vertex_buffers(0, self.subbuffer.clone());
    }
}

impl<T> MutableVertexBuffer<T>
where
    T: Vertex + BufferContents,
{
    pub fn new(gfx: &Graphics, vertices: Vec<T>) -> Arc<Self>
    where
        T: Vertex + BufferContents,
    {
        let staging_buffer = Buffer::from_iter(
            gfx.get_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            },
            vertices.into_iter(),
        )
        .expect("Failed to create vertex buffer.");

        let main_buffer = Buffer::new(
            gfx.get_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST | BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::DeviceOnly,
                ..Default::default()
            },
            DeviceLayout::from_size_alignment(staging_buffer.size(), align_of::<T>() as u64)
                .unwrap(),
        )
        .expect("Failed to create index buffer.");

        let main_subbuffer = Subbuffer::new(main_buffer).cast_aligned();

        let mut builder = AutoCommandBufferBuilder::primary(
            gfx.get_cmd_allocator(),
            gfx.graphics_queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer(CopyBufferInfoTyped::buffers(
                staging_buffer.clone(),
                main_subbuffer.clone(),
            ))
            .unwrap();

        let fence = builder.build().unwrap()
            .execute(gfx.graphics_queue())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        fence.wait(None).unwrap();

        Arc::new(Self {
            subbuffer: main_subbuffer,
            staging_buffer: staging_buffer,
        })
    }
    pub fn write_vertices(&self, gfx: &Graphics, writing_function: impl FnOnce(IterMut<'_, T>))
    {
        match self.staging_buffer.write() {
            Ok(mut write_guard) => {
                writing_function(write_guard.iter_mut());
            },
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        }

        let mut builder = AutoCommandBufferBuilder::primary(
            gfx.get_cmd_allocator(),
            gfx.graphics_queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer(CopyBufferInfoTyped::buffers(
                self.staging_buffer.clone(),
                self.subbuffer.clone(),
            ))
            .unwrap();

        let fence = builder
            .build()
            .unwrap()
            .execute(gfx.graphics_queue())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        fence.wait(None).unwrap();
    }
}

pub struct MutableIndexBuffer {
    subbuffer: Subbuffer<[u32]>,
}

impl Bindable for MutableIndexBuffer {
    fn bind_to_pipeline(&self, _builder: &mut PipelineBuilder, index_count: &mut u32) {
        *index_count = self.subbuffer.len().try_into().unwrap();
    }
    fn bind(
        &self,
        _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<
            PrimaryAutoCommandBuffer,
            StandardCommandBufferAllocator,
        >,
        _: Arc<PipelineLayout>,
    ) {
        builder.bind_index_buffer(self.subbuffer.clone());
    }
}

impl MutableIndexBuffer {
    pub fn new(gfx: &Graphics, indices: Vec<u32>) -> Arc<Self> {
        let staging_buffer = Buffer::from_iter(
            gfx.get_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            },
            indices.into_iter(),
        )
        .expect("Failed to create index buffer.");

        let main_buffer = Buffer::new(
            gfx.get_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST | BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::DeviceOnly,
                ..Default::default()
            },
            DeviceLayout::from_size_alignment(staging_buffer.size(), align_of::<u32>() as u64)
                .unwrap(),
        )
        .expect("Failed to create index buffer.");

        let main_subbuffer = Subbuffer::new(main_buffer).cast_aligned();

        let mut builder = AutoCommandBufferBuilder::primary(
            gfx.get_cmd_allocator(),
            gfx.graphics_queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer(CopyBufferInfoTyped::buffers(
                staging_buffer,
                main_subbuffer.clone(),
            ))
            .unwrap();

        let fence = builder
            .build()
            .unwrap()
            .execute(gfx.graphics_queue())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        fence.wait(None).unwrap();

        Arc::new(Self {
            subbuffer: main_subbuffer,
        })
    }
}

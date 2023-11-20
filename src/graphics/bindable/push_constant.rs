use std::{sync::{Arc, Mutex}, mem::size_of, collections::BTreeMap, ptr::{addr_of, addr_of_mut}};

use vulkano::{
    buffer::{BufferContents, Buffer, BufferCreateInfo, BufferUsage, Subbuffer, BufferError, subbuffer::BufferWriteGuard},
    memory::allocator::{AllocationCreateInfo, MemoryUsage}, pipeline::{PipelineLayout, layout::PushConstantRange},
    command_buffer::{
        AutoCommandBufferBuilder, allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer
    }, sync::Sharing, descriptor_set::{DescriptorSet, PersistentDescriptorSet, layout::{DescriptorSetLayoutCreateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorType}, WriteDescriptorSet}, shader::ShaderStages
};

use crate::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;

pub struct PushConstant<T> 
    where
    T: BufferContents
{
    push_constant_range: PushConstantRange,
    data: Mutex<T>,
}

impl<T> PushConstant<T>
    where
    T: BufferContents + Clone
{
    pub fn new(gfx: &Graphics, offset: u32, data: T, stages: ShaderStages) -> Arc<Self>
    {
        let range = PushConstantRange {
            stages: stages,
            offset: offset,
            size: size_of::<T>() as u32,
        };

        Arc::new(
            Self {
                push_constant_range: range,
                data: Mutex::new(data),
            }
        )
    }

    pub fn access_data(&self, accessing_function: impl FnOnce(&mut T))
    {
        match self.data.lock() {
            Ok(mut guard) => { accessing_function(&mut *guard)},
            Err(e) => println!("Push Constant access failed!")
        }
    }
}

impl<T> Bindable for PushConstant<T>
    where
    T: BufferContents + Clone
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder,
        _index_count: &mut u32)
    {
        builder.push_constant_ranges.push(self.push_constant_range.clone());
    }
    fn bind(&self, gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>,
        pipeline_layout: Arc<PipelineLayout>
    ) {
        builder.push_constants(pipeline_layout.clone(), self.push_constant_range.offset, self.data.lock().unwrap().clone());
    }
}
use std::{sync::Arc, mem::size_of, collections::BTreeMap};

use vulkano::{
    buffer::{BufferContents, Buffer, BufferCreateInfo, BufferUsage, Subbuffer, BufferError},
    memory::allocator::{AllocationCreateInfo, MemoryUsage}, pipeline::PipelineLayout,
    command_buffer::{
        AutoCommandBufferBuilder, allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer
    }, sync::Sharing, descriptor_set::{DescriptorSet, PersistentDescriptorSet, layout::{DescriptorSetLayoutCreateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorType}, WriteDescriptorSet}, shader::ShaderStages
};

use crate::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;
pub struct UniformBuffer<T> 
    where
    T: BufferContents
{
    subbuffer: Subbuffer<T>,
    layout: Arc<DescriptorSetLayout>,
    descriptor_set: Arc<PersistentDescriptorSet>,
}

impl<T> UniformBuffer<T>
    where
    T: BufferContents
{
    pub fn new(gfx: &Graphics, binding: u32, data: T) -> Arc<Self>
    {
        let subbuffer = Buffer::from_data(gfx.get_allocator(), BufferCreateInfo{
            sharing: Sharing::Exclusive,
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        }, AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        }, data)
        .unwrap();

        let layout = DescriptorSetLayout::new(
            gfx.get_device(), DescriptorSetLayoutCreateInfo {
                bindings: BTreeMap::from_iter(
                    [(binding, DescriptorSetLayoutBinding {
                        descriptor_count: 1,
                        variable_descriptor_count: false,
                        stages: ShaderStages::FRAGMENT,
                        ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::UniformBuffer)
                    })],
                ),
                ..Default::default()
            }
        ).unwrap();

        let set = PersistentDescriptorSet::new(
            gfx.get_descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::buffer_with_range(binding, subbuffer.clone(), 0..size_of::<T>() as u64)]
        ).unwrap();

        Arc::new(Self {
            subbuffer: subbuffer,
            layout: layout,
            descriptor_set: set,
        })
    }

    pub fn update_data(&self, data: T) -> Result<(), BufferError>
    {
        let mutable_reference;
        unsafe
        {
            mutable_reference = self.subbuffer
                .mapped_ptr()
                .unwrap()
                .cast::<T>()
                .as_mut();
        }
        //let guard = self.subbuffer.write()?;
        *mutable_reference = data;
        
        Ok(())
    }

    pub fn data(&self) -> Result<&T, BufferError>
    {
        let reference;
        unsafe
        {
            reference = self.subbuffer
                .mapped_ptr()
                .unwrap()
                .cast::<T>()
                .as_ref();
        }
        Ok(reference)
    }
}

impl<T> Bindable for UniformBuffer<T>
    where
    T: BufferContents
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder,
        _index_count: &mut u32)
    {
        builder.desriptor_set_layouts.push(self.layout.clone());
    }
    fn bind(&self, _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>,
        pipeline_layout: Arc<PipelineLayout>
    ) {
        builder.bind_descriptor_sets(vulkano::pipeline::PipelineBindPoint::Graphics, pipeline_layout.clone(), 0, self.descriptor_set.clone());
    }
}
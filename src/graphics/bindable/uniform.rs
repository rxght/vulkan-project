use std::{sync::{Arc, Mutex}, mem::size_of, collections::BTreeMap, ptr::{addr_of, addr_of_mut}};

use vulkano::{
    buffer::{BufferContents, Buffer, BufferCreateInfo, BufferUsage, Subbuffer, BufferError, subbuffer::BufferWriteGuard},
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
    subbuffers: Vec<Subbuffer<T>>,
    layout: Arc<DescriptorSetLayout>,
    descriptor_sets: Vec<Arc<PersistentDescriptorSet>>,
}

impl<T> UniformBuffer<T>
    where
    T: BufferContents + Clone
{
    pub fn new(gfx: &Graphics, binding: u32, data: T, stages: ShaderStages) -> Arc<Self>
    {
        let subbuffers: Vec<Subbuffer<T>> = (0..gfx.get_in_flight_count()).into_iter().map(|_| {
            Buffer::new_sized::<T>(gfx.get_allocator(), BufferCreateInfo{
                sharing: Sharing::Exclusive,
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            }, AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            }).unwrap()
        }).collect();

        subbuffers.iter().for_each(|p| {
            match p.write() {
                Ok(mut guard) => *guard = data.clone(),
                Err(e) => println!("error when writing initial value to uniform buffer: {e}"),
            }
        });

        let layout = DescriptorSetLayout::new(
            gfx.get_device(), DescriptorSetLayoutCreateInfo {
                bindings: BTreeMap::from_iter(
                    [(binding, DescriptorSetLayoutBinding {
                        descriptor_count: 1,
                        variable_descriptor_count: false,
                        stages: stages,
                        ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::UniformBuffer)
                    })],
                ),
                ..Default::default()
            }
        ).unwrap();

        let mut sets = Vec::with_capacity(gfx.get_in_flight_count());
        
        for set in subbuffers.iter().map(|subbuffer| {
            PersistentDescriptorSet::new(
                gfx.get_descriptor_set_allocator(),
                layout.clone(),
                [WriteDescriptorSet::buffer_with_range(binding, subbuffer.clone(), 0..size_of::<T>() as u64)]
            ).unwrap()
        }) {
            sets.push(set);
        }

        Arc::new(Self {
            subbuffers: subbuffers,
            layout: layout,
            descriptor_sets: sets,
        })
    }

    pub fn write(&self, gfx: &Graphics, writing_function: impl FnOnce(&mut T)) {

        let in_fligt_index = gfx.get_in_flight_index();

        match self.subbuffers[in_fligt_index].write() {
            Ok(mut guard) => {writing_function(&mut *guard)},
            Err(e) => {println!("unifom write error: {e}")},
        }
    }
}

impl<T> Bindable for UniformBuffer<T>
    where
    T: BufferContents + Clone
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder,
        _index_count: &mut u32)
    {
        builder.desriptor_set_layouts.push(self.layout.clone());
    }
    fn bind(&self, gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>,
        pipeline_layout: Arc<PipelineLayout>
    ) {
        let in_fligt_index = gfx.get_in_flight_index();

        builder.bind_descriptor_sets(vulkano::pipeline::PipelineBindPoint::Graphics, pipeline_layout.clone(), 0, self.descriptor_sets[in_fligt_index].clone());
    }
}
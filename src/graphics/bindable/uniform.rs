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

struct UniformBufferMutablePart<T>
{
    pub subbuffer_validity: Vec<bool>,
    pub staging_buffer: T,
}

pub struct UniformBuffer<T> 
    where
    T: BufferContents
{
    subbuffers: Vec<Subbuffer<T>>,
    layout: Arc<DescriptorSetLayout>,
    descriptor_sets: Vec<Arc<PersistentDescriptorSet>>,

    mutable_part: Mutex<UniformBufferMutablePart<T>>,
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

            mutable_part: Mutex::new( UniformBufferMutablePart{
                subbuffer_validity: vec![true; gfx.get_in_flight_count()],
                staging_buffer: data,
            }),
        })
    }

    pub fn access_data(&self, accessing_function: impl FnOnce(&mut T)) {

        match self.mutable_part.lock() {
            Ok(mut mutex_guard) => {
                // invalidate all subbuffers
                mutex_guard.subbuffer_validity.iter_mut().for_each(|p| *p = false );
                accessing_function(&mut mutex_guard.staging_buffer);
            }
            Err(e) => {
                println!("Uniform buffer mutex could not be locked! {e}");
            }
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
        let in_flight_index = gfx.get_in_flight_index();

        match self.mutable_part.lock() {
            Ok(mut mutex_guard) => {

                let valid = mutex_guard.subbuffer_validity[in_flight_index];
                if !valid {
                    if let Ok(mut buffer) = self.subbuffers[in_flight_index].write() {
                        *buffer = mutex_guard.staging_buffer.clone();
                        mutex_guard.subbuffer_validity[in_flight_index] = true;
                    }
                }
            }
            Err(e) => {
                println!("Uniform buffer mutex could not be locked! {e}");
            }
        }

        builder.bind_descriptor_sets(vulkano::pipeline::PipelineBindPoint::Graphics, pipeline_layout.clone(), 0, self.descriptor_sets[in_flight_index].clone());
    }
}
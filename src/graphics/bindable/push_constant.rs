use std::{
    mem::size_of,
    sync::{Arc, Mutex},
};

use vulkano::{
    buffer::BufferContents,
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        PrimaryAutoCommandBuffer,
    },
    pipeline::{layout::PushConstantRange, PipelineLayout},
    shader::ShaderStages,
};

use crate::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;

pub struct PushConstant<T>
where
    T: BufferContents,
{
    push_constant_range: PushConstantRange,
    data: Mutex<T>,
}

impl<T> PushConstant<T>
where
    T: BufferContents + Clone,
{
    pub fn new(_gfx: &Graphics, offset: u32, data: T, stages: ShaderStages) -> Arc<Self> {
        let range = PushConstantRange {
            stages: stages,
            offset: offset,
            size: size_of::<T>() as u32,
        };

        Arc::new(Self {
            push_constant_range: range,
            data: Mutex::new(data),
        })
    }

    pub fn access_data(&self, accessing_function: impl FnOnce(&mut T)) {
        match self.data.lock() {
            Ok(mut guard) => accessing_function(&mut *guard),
            Err(_e) => println!("Push Constant access failed!"),
        }
    }
}

impl<T> Bindable for PushConstant<T>
where
    T: BufferContents + Clone,
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, _index_count: &mut u32) {
        builder
            .push_constant_ranges
            .push(self.push_constant_range.clone());
    }
    fn bind(
        &self,
        _gfx: &Graphics,
        builder: &mut AutoCommandBufferBuilder<
            PrimaryAutoCommandBuffer,
            StandardCommandBufferAllocator,
        >,
        pipeline_layout: Arc<PipelineLayout>,
    ) {
        builder.push_constants(
            pipeline_layout.clone(),
            self.push_constant_range.offset,
            self.data.lock().unwrap().clone(),
        );
    }
}

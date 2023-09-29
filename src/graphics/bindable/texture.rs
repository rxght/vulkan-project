use std::{sync::Arc, io::Cursor};

use vulkano::{
    image::{ImmutableImage, view::ImageView, ImageDimensions},
    sampler::{Sampler, SamplerCreateInfo}, descriptor_set::{PersistentDescriptorSet, layout::{DescriptorSetLayout, DescriptorSetLayoutCreateInfo, DescriptorSetLayoutBinding, DescriptorType}, WriteDescriptorSet}, format::Format, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract}, sync::{GpuFuture, self, future, fence::{Fence, FenceCreateInfo}}, shader::ShaderStages
};

use crate::graphics::{pipeline::PipelineBuilder, Graphics};

use super::Bindable;

pub struct Texture
{
    pub image: Arc<ImageView<ImmutableImage>>,
    pub sampler: Arc<Sampler>,
    layout: Arc<DescriptorSetLayout>,
    descriptor_set: Arc<PersistentDescriptorSet>,
    set_num: u32,
}

impl Bindable for Texture
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder,index_count: &mut u32) {
        builder.desriptor_set_layouts.push(self.layout.clone());
    }

    fn bind(&self, _gfx: &Graphics,
            builder: &mut AutoCommandBufferBuilder<vulkano::command_buffer::PrimaryAutoCommandBuffer, vulkano::command_buffer::allocator::StandardCommandBufferAllocator>,
            pipeline_layout: Arc<vulkano::pipeline::PipelineLayout>
        ) {
        builder.bind_descriptor_sets(vulkano::pipeline::PipelineBindPoint::Graphics,
            pipeline_layout, self.set_num, self.descriptor_set.clone());
    }
}

impl Texture
{
    pub fn new(gfx: &Graphics, path: &str, set_num: u32, binding: u32) -> Arc<Self>
    {
        let mut uploads = AutoCommandBufferBuilder::primary(
            gfx.get_cmd_allocator(),
            gfx.graphics_queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let image = {
            let bytes = std::fs::read(path).expect("Texture file not found.");
            let cursor = Cursor::new(bytes);
            let decoder = png::Decoder::new(cursor);
            let mut reader = decoder.read_info().unwrap();
            let info = reader.info();
            let dimensions = ImageDimensions::Dim2d {
                width: info.width,
                height: info.height,
                array_layers: 1,
            };
            let mut image_data = vec![0; (info.width * info.height * 4) as usize];
            reader.next_frame(&mut image_data).unwrap();

            let image = ImmutableImage::from_iter(
                gfx.get_allocator(),
                image_data,
                dimensions,
                vulkano::image::MipmapsCount::One,
                Format::R8G8B8A8_SRGB,
                &mut uploads
            ).unwrap();
            ImageView::new_default(image).unwrap()
        };

        let fence =  uploads
            .build().unwrap()
            .execute(gfx.graphics_queue()).unwrap()
            .then_signal_fence_and_flush().unwrap();

        let sampler = Sampler::new(
            gfx.get_device(),
            SamplerCreateInfo::simple_repeat_linear(),
        ).unwrap();
            
        let layout = DescriptorSetLayout::new(
            gfx.get_device(),
            DescriptorSetLayoutCreateInfo{
                bindings: [(binding, DescriptorSetLayoutBinding{
                    stages: ShaderStages::FRAGMENT,
                    descriptor_count: 1, variable_descriptor_count: false,
                    immutable_samplers: vec![sampler.clone()],
                    ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::CombinedImageSampler)
                })].into(),
                ..Default::default()
            }
        ).unwrap();

        fence.wait(None).unwrap();

        let set = PersistentDescriptorSet::new(
            gfx.get_descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::image_view(binding, image.clone())]
        ).unwrap();
        
        Arc::new(Self {
            image: image,
            sampler: sampler,
            layout: layout,
            descriptor_set: set,
            set_num: set_num,
        })
    }
}
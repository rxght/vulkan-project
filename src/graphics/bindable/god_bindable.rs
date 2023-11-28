use std::sync::Arc;

use super::*;

type Builder = AutoCommandBufferBuilder<PrimaryAutoCommandBuffer, StandardCommandBufferAllocator>;

/// Can do anything but requires a bit of manual work to use.
/// Mainly intended for testing things without having to implement them first.
pub struct GodBindable<BindClosure, BindToPipelineClosure>
where
    BindClosure: Fn(&mut Builder, Arc<PipelineLayout>),
    BindToPipelineClosure: Fn(&mut PipelineBuilder, &mut u32),
{
    bind_closure: BindClosure,
    bind_to_pipeline_closure: BindToPipelineClosure,
}

impl<B, BP> Bindable for GodBindable<B, BP>
where
    B: Fn(&mut Builder, Arc<PipelineLayout>),
    BP: Fn(&mut PipelineBuilder, &mut u32),
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32) {
        (self.bind_to_pipeline_closure)(builder, index_count)
    }
    fn bind(&self, _gfx: &Graphics, builder: &mut Builder, pipeline_layout: Arc<PipelineLayout>) {
        (self.bind_closure)(builder, pipeline_layout)
    }
}

impl<BindClosure, BindToPipelineClosure> GodBindable<BindClosure, BindToPipelineClosure>
where
    BindClosure: Fn(&mut Builder, Arc<PipelineLayout>),
    BindToPipelineClosure: Fn(&mut PipelineBuilder, &mut u32),
{
    pub fn new(
        bind_closure: BindClosure,
        bind_to_pipeline_closure: BindToPipelineClosure,
    ) -> Arc<Self> {
        Arc::new(Self {
            bind_closure: bind_closure,
            bind_to_pipeline_closure: bind_to_pipeline_closure,
        })
    }
}

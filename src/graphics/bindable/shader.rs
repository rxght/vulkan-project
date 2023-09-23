
use std::sync::Arc;

use super::*;

pub struct VertexShader
{
    module: Arc<ShaderModule>
}

impl Bindable for VertexShader
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32) {
        builder.vertex_shader = Some(self.module.clone());
    }
}

impl VertexShader
{
    pub fn from_module(module: Arc<ShaderModule>) -> Arc<Self> { Arc::new(Self { module: module }) }
}

pub struct FragmentShader
{
    module: Arc<ShaderModule>
}

impl Bindable for FragmentShader
{
    fn bind_to_pipeline(&self, builder: &mut PipelineBuilder, index_count: &mut u32) {
        builder.fragment_shader = Some(self.module.clone());
    }
}

impl FragmentShader
{
    pub fn from_module(module: Arc<ShaderModule>) -> Arc<Self> { Arc::new(Self { module: module }) }
}
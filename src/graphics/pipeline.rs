
use std::sync::Arc;
use vulkano::{
    pipeline::{
        GraphicsPipeline,
        graphics::{
            vertex_input::VertexBufferDescription, input_assembly::InputAssemblyState,
            viewport::ViewportState, color_blend::ColorBlendState, rasterization::{RasterizationState, PolygonMode, CullMode, FrontFace, DepthBiasState, DepthBias},
            depth_stencil::DepthStencilState, discard_rectangle::DiscardRectangleState,
            multisample::MultisampleState, tessellation::TessellationState,
            render_pass::PipelineRenderPassType
        },
        PipelineLayout,
        layout::{PipelineLayoutCreateInfo, PushConstantRange}, StateMode
    },
    device::Device,
    render_pass::Subpass,
    shader::ShaderModule, 
    descriptor_set::layout::DescriptorSetLayout
};

use super::Graphics;

pub struct PipelineBuilder
{
    pub subpass: Subpass,
    pub vertex_buffer_description: Option<VertexBufferDescription>,
    pub input_assembly_state: InputAssemblyState,
    pub vertex_shader: Option<Arc<ShaderModule>>,
    pub fragment_shader: Option<Arc<ShaderModule>>,
    pub viewport_state: ViewportState,
    pub color_blend_state: ColorBlendState,
    pub rasterization_state: RasterizationState,
    pub depth_stencil_state: DepthStencilState,
    pub discard_rectangle_state: DiscardRectangleState,
    pub multisample_state: MultisampleState,
    pub tessellation_state: TessellationState,

    pub desriptor_set_layouts: Vec<Arc<DescriptorSetLayout>>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

impl PipelineBuilder
{
    pub fn new(gfx: &Graphics) -> Self
    {
        Self {
            subpass: Subpass::from(gfx.get_main_render_pass(), 0).unwrap(),
            vertex_buffer_description: None,
            input_assembly_state: InputAssemblyState::new(),
            vertex_shader: None,
            fragment_shader: None,
            viewport_state: ViewportState::viewport_dynamic_scissor_irrelevant(),
            color_blend_state: ColorBlendState::default(),
            rasterization_state: RasterizationState {
                cull_mode: StateMode::Fixed(CullMode::Back),
                front_face: StateMode::Fixed(FrontFace::Clockwise),
                depth_bias: None,
                ..Default::default()
            },
            depth_stencil_state: DepthStencilState::disabled(),
            discard_rectangle_state: DiscardRectangleState::new(),
            multisample_state: MultisampleState::new(),
            tessellation_state: TessellationState::new(),

            desriptor_set_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }

    pub fn build(self, device: Arc<Device>) -> (Arc<GraphicsPipeline>, Arc<PipelineLayout>)
    {
        let vertex_shader_entry = self.vertex_shader.as_ref()
            .expect("No vertex shader supplied.")
            .entry_point("main").unwrap();

        let fragment_shader_entry = self.fragment_shader.as_ref()
            .expect("No fragment shader supplied.")
            .entry_point("main").unwrap();

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts: self.desriptor_set_layouts,
                push_constant_ranges: self.push_constant_ranges,
                ..Default::default()
            }
        ).unwrap();

        (
            GraphicsPipeline::start()
                .render_pass(PipelineRenderPassType::BeginRenderPass(self.subpass))
                .vertex_input_state(self.vertex_buffer_description.unwrap())
                .input_assembly_state(self.input_assembly_state)
                .vertex_shader(vertex_shader_entry, ())
                .fragment_shader(fragment_shader_entry, ())
                .viewport_state(self.viewport_state)
                .color_blend_state(self.color_blend_state)
                .rasterization_state(self.rasterization_state)
                .depth_stencil_state(self.depth_stencil_state)
                .discard_rectangle_state(self.discard_rectangle_state)
                .multisample_state(self.multisample_state)
                .tessellation_state(self.tessellation_state)
                .with_pipeline_layout(device.clone(), layout.clone()).expect("Failed to create pipeline!"),
            layout
        )
    }
}

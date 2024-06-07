// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

use super::*;

vulkan_create_info_lifetime!(
    PipelineShaderStageCreateInfo,
    VkPipelineShaderStageCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO
);

impl<'a> PipelineShaderStageCreateInfo<'a> {
    pub const fn new_init(stage: ShaderStageFlags, module: ShaderModule, name: &'a CStr) -> Self {
        Self::new()
            .with_stage(stage)
            .with_module(module)
            .with_name(name)
    }

    pub const fn with_stage(mut self, stage: ShaderStageFlags) -> Self {
        self.inner.stage = stage.bits() as VkShaderStageFlagBits;
        self
    }

    pub const fn with_module(mut self, module: ShaderModule) -> Self {
        self.inner.module = module.as_raw();
        self
    }

    pub const fn with_name(mut self, name: &'a CStr) -> Self {
        self.inner.pName = name.as_ptr();
        self
    }
}

vulkan_struct!(
    VertexInputBindingDescription,
    VkVertexInputBindingDescription
);

impl VertexInputBindingDescription {
    pub const fn with_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    pub const fn with_stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }

    pub const fn with_input_rate(mut self, input_rate: VertexInputRate) -> Self {
        self.inner.inputRate = input_rate.as_raw();
        self
    }
}

vulkan_struct!(
    VertexInputAttributeDescription,
    VkVertexInputAttributeDescription
);

impl VertexInputAttributeDescription {
    pub const fn with_location(mut self, location: u32) -> Self {
        self.inner.location = location;
        self
    }

    pub const fn with_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    pub const fn with_format(mut self, format: Format) -> Self {
        self.inner.format = format.as_raw();
        self
    }

    pub const fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
}

vulkan_create_info_lifetime!(
    PipelineVertexInputStateCreateInfo,
    VkPipelineVertexInputStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO
);

impl<'a> PipelineVertexInputStateCreateInfo<'a> {
    pub const fn with_vertex_binding_descriptions(
        mut self,
        vertex_binding_descriptions: &'a [VertexInputBindingDescription],
    ) -> Self {
        self.inner.vertexBindingDescriptionCount = vertex_binding_descriptions.len() as u32;
        self.inner.pVertexBindingDescriptions = vertex_binding_descriptions.as_ptr().cast();
        self
    }

    pub const fn with_vertex_attribute_descriptions(
        mut self,
        vertex_attribute_descriptions: &'a [VertexInputAttributeDescription],
    ) -> Self {
        self.inner.vertexAttributeDescriptionCount = vertex_attribute_descriptions.len() as u32;
        self.inner.pVertexAttributeDescriptions = vertex_attribute_descriptions.as_ptr().cast();
        self
    }
}

vulkan_create_info!(
    PipelineInputAssemblyStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO
);

impl PipelineInputAssemblyStateCreateInfo {
    pub const fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.inner.topology = topology.as_raw();
        self
    }

    pub const fn with_primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.inner.primitiveRestartEnable = primitive_restart_enable as u32;
        self
    }
}

vulkan_create_info_lifetime!(
    PipelineTessellationStateCreateInfo,
    VkPipelineTessellationStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO
);

vulkan_create_info_lifetime!(
    PipelineViewportStateCreateInfo,
    VkPipelineViewportStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO
);

impl<'a> PipelineViewportStateCreateInfo<'a> {
    pub const fn with_viewports(mut self, viewports: &'a [Viewport]) -> Self {
        self.inner.viewportCount = viewports.len() as u32;
        self.inner.pViewports = viewports.as_ptr().cast();
        self
    }

    pub const fn with_scissors(mut self, scissors: &'a [Rect2D]) -> Self {
        self.inner.scissorCount = scissors.len() as u32;
        self.inner.pScissors = scissors.as_ptr().cast();
        self
    }

    pub const fn with_dynamic_viewport(mut self, count: u32) -> Self {
        self.inner.viewportCount = count;
        self.inner.pViewports = std::ptr::null();
        self
    }

    pub const fn with_dynamic_scissor(mut self, count: u32) -> Self {
        self.inner.scissorCount = count;
        self.inner.pScissors = std::ptr::null();
        self
    }
}

vulkan_create_info!(
    PipelineRasterizationStateCreateInfo,
    VkPipelineRasterizationStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO
);

impl PipelineRasterizationStateCreateInfo {
    pub const fn with_depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.inner.depthClampEnable = depth_clamp_enable as u32;
        self
    }

    pub const fn with_rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.inner.rasterizerDiscardEnable = rasterizer_discard_enable as u32;
        self
    }

    pub const fn with_polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
        self.inner.polygonMode = polygon_mode.as_raw();
        self
    }

    pub const fn with_cull_mode(mut self, cull_mode: CullModeFlags) -> Self {
        self.inner.cullMode = cull_mode.bits();
        self
    }

    pub const fn with_front_face(mut self, front_face: FrontFace) -> Self {
        self.inner.frontFace = front_face.as_raw();
        self
    }

    pub const fn with_depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.inner.depthBiasEnable = depth_bias_enable as u32;
        self
    }

    pub const fn with_depth_bias_constant_factor(
        mut self,
        depth_bias_constant_factor: f32,
    ) -> Self {
        self.inner.depthBiasConstantFactor = depth_bias_constant_factor;
        self
    }

    pub const fn with_depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depthBiasClamp = depth_bias_clamp;
        self
    }

    pub const fn with_depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depthBiasSlopeFactor = depth_bias_slope_factor;
        self
    }

    pub const fn with_line_width(mut self, line_width: f32) -> Self {
        self.inner.lineWidth = line_width;
        self
    }

    pub const FILL_CULL_CCW: Self = Self::new()
        .with_polygon_mode(PolygonMode::FILL)
        .with_cull_mode(CullModeFlags::BACK)
        .with_front_face(FrontFace::COUNTER_CLOCKWISE)
        .with_line_width(1.0);
}

vulkan_create_info_lifetime!(
    PipelineMultisampleStateCreateInfo,
    VkPipelineMultisampleStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO
);

impl<'a> PipelineMultisampleStateCreateInfo<'a> {
    pub const fn with_rasterization_samples(
        mut self,
        rasterization_samples: SampleCountFlags,
    ) -> Self {
        self.inner.rasterizationSamples = rasterization_samples.bits() as VkSampleCountFlagBits;
        self
    }

    pub const fn with_sample_shading_enable(mut self, sample_shading_enable: bool) -> Self {
        self.inner.sampleShadingEnable = sample_shading_enable as u32;
        self
    }

    pub const fn with_min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.minSampleShading = min_sample_shading;
        self
    }

    pub const fn with_sample_mask(mut self, sample_mask: &'a [SampleMask]) -> Self {
        self.inner.pSampleMask = sample_mask.as_ptr().cast();
        self
    }

    pub const fn with_alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alphaToCoverageEnable = alpha_to_coverage_enable as u32;
        self
    }

    pub const fn with_alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.inner.alphaToOneEnable = alpha_to_one_enable as u32;
        self
    }
}

vulkan_create_info!(
    PipelineDepthStencilStateCreateInfo,
    VkPipelineDepthStencilStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO
);

impl PipelineDepthStencilStateCreateInfo {
    pub const fn with_depth_test_enable(mut self, depth_test_enable: bool) -> Self {
        self.inner.depthTestEnable = depth_test_enable as u32;
        self
    }

    pub const fn with_depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.inner.depthWriteEnable = depth_write_enable as u32;
        self
    }

    pub const fn with_depth_compare_op(mut self, depth_compare_op: CompareOp) -> Self {
        self.inner.depthCompareOp = depth_compare_op.as_raw();
        self
    }

    pub const fn with_depth_bounds_test_enable(mut self, depth_bounds_test_enable: bool) -> Self {
        self.inner.depthBoundsTestEnable = depth_bounds_test_enable as u32;
        self
    }

    pub const fn with_stencil_test_enable(mut self, stencil_test_enable: bool) -> Self {
        self.inner.stencilTestEnable = stencil_test_enable as u32;
        self
    }

    pub const fn with_front(mut self, front: StencilOpState) -> Self {
        self.inner.front = front;
        self
    }

    pub const fn with_back(mut self, back: StencilOpState) -> Self {
        self.inner.back = back;
        self
    }

    pub const fn with_depth_bounds(mut self, min: f32, max: f32) -> Self {
        self.inner.minDepthBounds = min;
        self.inner.maxDepthBounds = max;
        self
    }
}

vulkan_struct!(
    PipelineColorBlendAttachmentState,
    VkPipelineColorBlendAttachmentState
);

impl PipelineColorBlendAttachmentState {
    pub const fn with_blend_enable(mut self, blend_enable: bool) -> Self {
        self.inner.blendEnable = blend_enable as u32;
        self
    }

    pub const fn with_src_color_blend_factor(
        mut self,
        src_color_blend_factor: BlendFactor,
    ) -> Self {
        self.inner.srcColorBlendFactor = src_color_blend_factor.as_raw();
        self
    }

    pub const fn with_dst_color_blend_factor(
        mut self,
        dst_color_blend_factor: BlendFactor,
    ) -> Self {
        self.inner.dstColorBlendFactor = dst_color_blend_factor.as_raw();
        self
    }

    pub const fn with_color_blend_op(mut self, color_blend_op: BlendOp) -> Self {
        self.inner.colorBlendOp = color_blend_op.as_raw();
        self
    }

    pub const fn with_src_alpha_blend_factor(
        mut self,
        src_alpha_blend_factor: BlendFactor,
    ) -> Self {
        self.inner.srcAlphaBlendFactor = src_alpha_blend_factor.as_raw();
        self
    }

    pub const fn with_dst_alpha_blend_factor(
        mut self,
        dst_alpha_blend_factor: BlendFactor,
    ) -> Self {
        self.inner.dstAlphaBlendFactor = dst_alpha_blend_factor.as_raw();
        self
    }

    pub const fn with_alpha_blend_op(mut self, alpha_blend_op: BlendOp) -> Self {
        self.inner.alphaBlendOp = alpha_blend_op.as_raw();
        self
    }

    pub const fn with_color_write_mask(mut self, color_write_mask: ColorComponentFlags) -> Self {
        self.inner.colorWriteMask = color_write_mask.bits();
        self
    }

    pub const BLEND_ALPHA_RGBA: Self = Self::new()
        .with_blend_enable(true)
        .with_src_color_blend_factor(BlendFactor::SRC_ALPHA)
        .with_dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
        .with_color_blend_op(BlendOp::ADD)
        .with_src_alpha_blend_factor(BlendFactor::ONE)
        .with_dst_alpha_blend_factor(BlendFactor::ZERO)
        .with_alpha_blend_op(BlendOp::ADD)
        .with_color_write_mask(ColorComponentFlags::all());
}

vulkan_create_info_lifetime!(
    PipelineColorBlendStateCreateInfo,
    VkPipelineColorBlendStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO
);

impl<'a> PipelineColorBlendStateCreateInfo<'a> {
    pub const fn with_logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.inner.logicOpEnable = logic_op_enable as u32;
        self
    }

    pub const fn with_logic_op(mut self, logic_op: LogicOp) -> Self {
        self.inner.logicOp = logic_op.as_raw();
        self
    }

    pub const fn with_attachments(
        mut self,
        attachments: &'a [PipelineColorBlendAttachmentState],
    ) -> Self {
        self.inner.attachmentCount = attachments.len() as u32;
        self.inner.pAttachments = attachments.as_ptr().cast();
        self
    }

    pub const fn with_blend_constants(mut self, blend_constants: [f32; 4]) -> Self {
        self.inner.blendConstants = blend_constants;
        self
    }
}

vulkan_create_info_lifetime!(
    PipelineDynamicStateCreateInfo,
    VkPipelineDynamicStateCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO
);

impl<'a> PipelineDynamicStateCreateInfo<'a> {
    pub const fn with_dynamic_states(mut self, dynamic_states: &'a [DynamicState]) -> Self {
        self.inner.dynamicStateCount = dynamic_states.len() as u32;
        self.inner.pDynamicStates = dynamic_states.as_ptr().cast();
        self
    }
}

vulkan_create_info_lifetime!(
    GraphicsPipelineCreateInfo,
    VkGraphicsPipelineCreateInfo,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO
);

impl<'a> GraphicsPipelineCreateInfo<'a> {
    pub const fn with_flags(mut self, flags: PipelineCreateFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    pub const fn with_stages(mut self, stages: &'a [PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stageCount = stages.len() as u32;
        self.inner.pStages = stages.as_ptr().cast();
        self
    }

    pub const fn with_vertex_input_state(
        mut self,
        vertex_input_state: &'a PipelineVertexInputStateCreateInfo<'a>,
    ) -> Self {
        self.inner.pVertexInputState = vertex_input_state.as_raw();
        self
    }

    pub const fn with_input_assembly_state(
        mut self,
        input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo,
    ) -> Self {
        self.inner.pInputAssemblyState = input_assembly_state.as_raw();
        self
    }

    pub const fn with_tessellation_state(
        mut self,
        tessellation_state: &'a PipelineTessellationStateCreateInfo<'a>,
    ) -> Self {
        self.inner.pTessellationState = tessellation_state.as_raw();
        self
    }

    pub const fn with_viewport_state(
        mut self,
        viewport_state: &'a PipelineViewportStateCreateInfo<'a>,
    ) -> Self {
        self.inner.pViewportState = viewport_state.as_raw();
        self
    }

    pub const fn with_rasterization_state(
        mut self,
        rasterization_state: &'a PipelineRasterizationStateCreateInfo,
    ) -> Self {
        self.inner.pRasterizationState = rasterization_state.as_raw();
        self
    }

    pub const fn with_multisample_state(
        mut self,
        multisample_state: &'a PipelineMultisampleStateCreateInfo,
    ) -> Self {
        self.inner.pMultisampleState = multisample_state.as_raw();
        self
    }

    pub const fn with_depth_stencil_state(
        mut self,
        depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo,
    ) -> Self {
        self.inner.pDepthStencilState = depth_stencil_state.as_raw();
        self
    }

    pub const fn with_color_blend_state(
        mut self,
        color_blend_state: &'a PipelineColorBlendStateCreateInfo<'a>,
    ) -> Self {
        self.inner.pColorBlendState = color_blend_state.as_raw();
        self
    }

    pub const fn with_dynamic_state(
        mut self,
        dynamic_state: &'a PipelineDynamicStateCreateInfo<'a>,
    ) -> Self {
        self.inner.pDynamicState = dynamic_state.as_raw();
        self
    }

    pub const fn with_layout(mut self, layout: PipelineLayout) -> Self {
        self.inner.layout = layout.as_raw();
        self
    }

    pub const fn with_render_pass(mut self, render_pass: RenderPass, subpass: u32) -> Self {
        self.inner.renderPass = render_pass.as_raw();
        self.inner.subpass = subpass;
        self
    }

    pub const fn with_base_pipeline(mut self, base_pipeline: Pipeline) -> Self {
        self.inner.basePipelineHandle = base_pipeline.as_raw();
        self.inner.basePipelineIndex = -1;
        self
    }

    pub const fn with_base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.basePipelineIndex = base_pipeline_index;
        self.inner.basePipelineHandle = std::ptr::null_mut();
        self
    }
}

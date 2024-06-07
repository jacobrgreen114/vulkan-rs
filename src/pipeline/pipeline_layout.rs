// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(PipelineLayout, VkPipelineLayout);

vulkan_create_info_lifetime!(
    PipelineLayoutCreateInfo,
    VkPipelineLayoutCreateInfo,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO
);

impl<'a> PipelineLayoutCreateInfo<'a> {
    pub const fn with_set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
        self.inner.setLayoutCount = set_layouts.len() as u32;
        self.inner.pSetLayouts = set_layouts.as_ptr().cast();
        self
    }

    pub const fn with_push_constant_ranges(
        mut self,
        push_constant_ranges: &'a [PushConstantRange],
    ) -> Self {
        self.inner.pushConstantRangeCount = push_constant_ranges.len() as u32;
        self.inner.pPushConstantRanges = push_constant_ranges.as_ptr().cast();
        self
    }
}

vulkan_struct!(PushConstantRange, VkPushConstantRange);

impl PushConstantRange {
    pub const fn new_init(stage_flags: ShaderStageFlags, offset: u32, size: u32) -> Self {
        Self {
            inner: VkPushConstantRange {
                stageFlags: stage_flags.bits(),
                offset,
                size,
            },
        }
    }

    pub const fn with_stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
        self.inner.stageFlags = stage_flags.bits();
        self
    }

    pub const fn with_offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    pub const fn with_size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
}

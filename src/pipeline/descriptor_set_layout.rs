// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(DescriptorSetLayout, VkDescriptorSetLayout);

vulkan_struct_lifetime!(DescriptorSetLayoutBinding, VkDescriptorSetLayoutBinding);

impl<'a> DescriptorSetLayoutBinding<'a> {
    pub const fn with_binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }

    pub const fn with_descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
        self.inner.descriptorType = descriptor_type.as_raw();
        self
    }

    pub const fn with_descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptorCount = descriptor_count;
        self
    }

    pub const fn with_stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
        self.inner.stageFlags = stage_flags.bits();
        self
    }

    pub const fn new_init(
        binding: u32,
        descriptor_type: DescriptorType,
        descriptor_count: u32,
        stage_flags: ShaderStageFlags,
    ) -> Self {
        Self {
            inner: VkDescriptorSetLayoutBinding {
                binding,
                descriptorType: descriptor_type.as_raw(),
                descriptorCount: descriptor_count,
                stageFlags: stage_flags.bits(),
                pImmutableSamplers: std::ptr::null(),
            },
            phantom: std::marker::PhantomData,
        }
    }

    // pub const fn with_immutable_samplers(mut self, immutable_samplers: &'a [Sampler]) -> Self {
    //     self.inner.pImmutableSamplers = immutable_samplers.as_ptr();
    //     self
    // }
}

vulkan_create_info_referential!(
    DescriptorSetLayoutCreateInfo,
    VkDescriptorSetLayoutCreateInfo,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO
);

impl<'a> DescriptorSetLayoutCreateInfo<'a> {
    pub const fn with_bindings(mut self, bindings: &'a [DescriptorSetLayoutBinding]) -> Self {
        self.inner.bindingCount = bindings.len() as u32;
        self.inner.pBindings = bindings.as_ptr().cast();
        self
    }
}

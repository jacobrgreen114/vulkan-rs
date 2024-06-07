// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(ShaderModule, VkShaderModule);

vulkan_create_info_lifetime!(
    ShaderModuleCreateInfo,
    VkShaderModuleCreateInfo,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO
);

impl<'a> ShaderModuleCreateInfo<'a> {
    pub const fn with_code(mut self, code: &'a [u8]) -> Self {
        self.inner.codeSize = code.len();
        self.inner.pCode = code.as_ptr().cast();
        self
    }

    pub const fn with_code_u32(mut self, code: &'a [u32]) -> Self {
        self.inner.codeSize = code.len() * std::mem::size_of::<u32>();
        self.inner.pCode = code.as_ptr();
        self
    }
}

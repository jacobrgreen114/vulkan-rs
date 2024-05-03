// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Framebuffer, VkFramebuffer);

vulkan_create_info_referential!(
    FramebufferCreateInfo,
    VkFramebufferCreateInfo,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO
);

impl<'a> FramebufferCreateInfo<'a> {
    pub const fn with_render_pass(mut self, render_pass: RenderPass) -> Self {
        self.inner.renderPass = render_pass.as_raw();
        self
    }

    pub const fn with_attachments(mut self, attachments: &'a [ImageView]) -> Self {
        self.inner.attachmentCount = attachments.len() as u32;
        self.inner.pAttachments = attachments.as_ptr().cast();
        self
    }

    pub const fn with_width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }

    pub const fn with_height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }

    pub const fn with_layers(mut self, layers: u32) -> Self {
        self.inner.layers = layers;
        self
    }
}

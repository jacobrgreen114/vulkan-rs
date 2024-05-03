// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_create_info!(
    CommandBufferAllocateInfo,
    VkCommandBufferAllocateInfo,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
);

impl crate::CommandBufferAllocateInfo {
    pub const fn with_command_pool(mut self, command_pool: CommandPool) -> Self {
        self.inner.commandPool = command_pool.as_raw();
        self
    }

    pub const fn with_level(mut self, level: CommandBufferLevel) -> Self {
        self.inner.level = level as i32;
        self
    }

    pub const fn with_command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.inner.commandBufferCount = command_buffer_count;
        self
    }
}

vulkan_handle!(CommandBuffer, VkCommandBuffer);

vulkan_create_info_referential!(
    CommandBufferBeginInfo,
    VkCommandBufferBeginInfo,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
);

impl<'a> crate::CommandBufferBeginInfo<'a> {
    pub const fn with_flags(mut self, flags: CommandBufferUsageFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    //pub const fn with_inheritance_info(mut self, inheritance_info: &'a CommandBufferInheritanceInfo) -> Self {
    //    self.inner.pInheritanceInfo = inheritance_info.as_raw();
    //    self
    //}
}

vulkan_create_info_referential!(
    RenderPassBeginInfo,
    VkRenderPassBeginInfo,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO
);

impl<'a> crate::RenderPassBeginInfo<'a> {
    pub const fn with_render_pass(mut self, render_pass: RenderPass) -> Self {
        self.inner.renderPass = render_pass.as_raw();
        self
    }

    pub const fn with_framebuffer(mut self, framebuffer: Framebuffer) -> Self {
        self.inner.framebuffer = framebuffer.as_raw();
        self
    }

    pub const fn with_render_area(mut self, render_area: Rect2D) -> Self {
        self.inner.renderArea = render_area;
        self
    }

    pub const fn with_clear_values(mut self, clear_values: &'a [ClearValue]) -> Self {
        self.inner.clearValueCount = clear_values.len() as u32;
        self.inner.pClearValues = clear_values.as_ptr().cast();
        self
    }
}

impl CommandBuffer {
    pub fn reset(&self, flags: CommandBufferResetFlags) -> vulkan_sys::wrapper::Result<()> {
        self.reset_fn(vkResetCommandBuffer, flags)
    }

    pub fn reset_fn(
        &self,
        proc: PFN_vkResetCommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> vulkan_sys::wrapper::Result<()> {
        reset_command_buffer(proc, self.as_raw(), flags.bits())
    }

    pub fn begin(
        &self,
        begin_info: &crate::CommandBufferBeginInfo,
    ) -> vulkan_sys::wrapper::Result<()> {
        self.begin_fn(vkBeginCommandBuffer, begin_info)
    }

    pub fn begin_fn(
        &self,
        proc: PFN_vkBeginCommandBuffer,
        begin_info: &crate::CommandBufferBeginInfo,
    ) -> vulkan_sys::wrapper::Result<()> {
        begin_command_buffer(proc, self.as_raw(), begin_info.as_raw())
    }

    pub fn end(&self) -> vulkan_sys::wrapper::Result<()> {
        end_command_buffer(vkEndCommandBuffer, self.as_raw())
    }

    pub fn end_fn(&self, proc: PFN_vkEndCommandBuffer) -> vulkan_sys::wrapper::Result<()> {
        end_command_buffer(proc, self.as_raw())
    }

    pub fn cmd_begin_render_pass(
        &self,
        render_pass_begin_info: &crate::RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        self.cmd_begin_render_pass_fn(vkCmdBeginRenderPass, render_pass_begin_info, contents)
    }

    pub fn cmd_begin_render_pass_fn(
        &self,
        proc: PFN_vkCmdBeginRenderPass,
        render_pass_begin_info: &crate::RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        cmd_begin_render_pass(
            vkCmdBeginRenderPass,
            self.as_raw(),
            render_pass_begin_info.as_raw(),
            contents.as_raw(),
        )
    }

    pub fn cmd_end_render_pass(&self) {
        self.cmd_end_render_pass_fn(vkCmdEndRenderPass)
    }

    pub fn cmd_end_render_pass_fn(&self, proc: PFN_vkCmdEndRenderPass) {
        cmd_end_render_pass(proc, self.as_raw())
    }
}

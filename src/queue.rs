// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Queue, VkQueue);

vulkan_create_info_referential!(SubmitInfo, VkSubmitInfo, VK_STRUCTURE_TYPE_SUBMIT_INFO);

impl<'a> SubmitInfo<'a> {
    pub const fn with_wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
        self.inner.waitSemaphoreCount = wait_semaphores.len() as u32;
        self.inner.pWaitSemaphores = wait_semaphores.as_ptr().cast();
        self
    }

    pub const fn with_wait_dst_stage_mask(
        mut self,
        wait_dst_stage_mask: &'a [PipelineStageFlags],
    ) -> Self {
        self.inner.pWaitDstStageMask = wait_dst_stage_mask.as_ptr() as *const _;
        self
    }

    pub const fn with_command_buffers(mut self, command_buffers: &'a [CommandBuffer]) -> Self {
        self.inner.commandBufferCount = command_buffers.len() as u32;
        self.inner.pCommandBuffers = command_buffers.as_ptr() as *const _;
        self
    }

    pub const fn with_signal_semaphores(mut self, signal_semaphores: &'a [Semaphore]) -> Self {
        self.inner.signalSemaphoreCount = signal_semaphores.len() as u32;
        self.inner.pSignalSemaphores = signal_semaphores.as_ptr().cast();
        self
    }
}

vulkan_create_info_referential!(
    PresentInfoKHR,
    VkPresentInfoKHR,
    VK_STRUCTURE_TYPE_PRESENT_INFO_KHR
);

impl<'a> PresentInfoKHR<'a> {
    pub const fn with_wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
        self.inner.waitSemaphoreCount = wait_semaphores.len() as u32;
        self.inner.pWaitSemaphores = wait_semaphores.as_ptr().cast();
        self
    }

    pub const fn with_swapchains(mut self, swapchains: &'a [SwapchainKHR]) -> Self {
        self.inner.swapchainCount = swapchains.len() as u32;
        self.inner.pSwapchains = swapchains.as_ptr().cast();
        self
    }

    pub const fn with_image_indices(mut self, image_indices: &'a [u32]) -> Self {
        self.inner.pImageIndices = image_indices.as_ptr();
        self
    }

    // pub const fn with_results(mut self, results: &'a [Result]) -> Self {
    //     self.inner.pResults = results.as_ptr();
    //     self
    // }
}

impl Queue {
    pub fn submit(&self, submits: &[SubmitInfo], fence: Option<Fence>) -> Result<()> {
        unsafe {
            queue_submit(
                vkQueueSubmit,
                self.handle,
                transmute(submits),
                transmute(fence),
            )
        }
    }

    pub fn present_khr(&self, present_info: &PresentInfoKHR) -> Result<()> {
        queue_present_khr(vkQueuePresentKHR, self.as_raw(), present_info.as_raw())
    }

    pub fn wait_idle(&self) -> Result<()> {
        queue_wait_idle(vkQueueWaitIdle, self.as_raw())
    }
}

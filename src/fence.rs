// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_create_info!(
    FenceCreateInfo,
    VkFenceCreateInfo,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO
);

impl FenceCreateInfo {
    pub const fn with_flags(mut self, flags: FenceCreateFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    pub const SIGNALED: Self = Self::new().with_flags(FenceCreateFlags::SIGNALED);
}

vulkan_handle!(Fence, VkFence);

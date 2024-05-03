// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

use bitflags::bitflags;

bitflags! {
    pub struct CommandPoolCreateFlags: u32 {
        const TRANSIENT = VK_COMMAND_POOL_CREATE_TRANSIENT_BIT as u32;
        const RESET_COMMAND_BUFFER = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32;
    }
}

vulkan_create_info!(
    CommandPoolCreateInfo,
    VkCommandPoolCreateInfo,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO
);

impl crate::CommandPoolCreateInfo {
    pub const fn with_flags(mut self, flags: crate::CommandPoolCreateFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    pub const fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queueFamilyIndex = queue_family_index;
        self
    }
}

vulkan_handle!(CommandPool, VkCommandPool);

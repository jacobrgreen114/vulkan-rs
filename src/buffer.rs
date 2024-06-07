// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Buffer, VkBuffer);

vulkan_create_info_lifetime!(
    BufferCreateInfo,
    VkBufferCreateInfo,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO
);

impl<'a> BufferCreateInfo<'a> {
    pub const fn with_size(mut self, size: DeviceSize) -> Self {
        self.inner.size = size;
        self
    }

    pub const fn with_usage(mut self, usage: BufferUsageFlags) -> Self {
        self.inner.usage = usage.bits();
        self
    }

    pub const fn with_exclusive(mut self) -> Self {
        self.inner.sharingMode = SharingMode::EXCLUSIVE.as_raw();
        self
    }

    pub const fn with_sharing(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.inner.sharingMode = SharingMode::CONCURRENT.as_raw();
        self.inner.queueFamilyIndexCount = queue_family_indices.len() as u32;
        self.inner.pQueueFamilyIndices = queue_family_indices.as_ptr();
        self
    }
}

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Semaphore, VkSemaphore);

vulkan_create_info!(
    SemaphoreCreateInfo,
    VkSemaphoreCreateInfo,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO
);

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Image, VkImage);

vulkan_create_info_referential!(
    ImageCreateInfo,
    VkImageCreateInfo,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO
);

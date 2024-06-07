// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(ImageView, VkImageView);

vulkan_create_info!(
    ImageViewCreateInfo,
    VkImageViewCreateInfo,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO
);

impl ImageViewCreateInfo {
    pub const fn with_image(mut self, image: Image) -> Self {
        self.inner.image = image.as_raw();
        self
    }

    pub const fn with_view_type(mut self, view_type: ImageViewType) -> Self {
        self.inner.viewType = view_type as i32;
        self
    }

    pub const fn with_format(mut self, format: Format) -> Self {
        self.inner.format = format as i32;
        self
    }

    pub const fn with_components(mut self, components: ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }

    pub const fn with_subresource_range(
        mut self,
        subresource_range: ImageSubresourceRange,
    ) -> Self {
        self.inner.subresourceRange = *subresource_range.as_raw();
        self
    }
}

vulkan_struct!(ImageSubresourceRange, VkImageSubresourceRange);

impl ImageSubresourceRange {
    pub const fn new_init(
        aspect_mask: ImageAspectFlags,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Self {
        Self {
            inner: VkImageSubresourceRange {
                aspectMask: aspect_mask.bits(),
                baseMipLevel: base_mip_level,
                levelCount: level_count,
                baseArrayLayer: base_array_layer,
                layerCount: layer_count,
            },
        }
    }

    pub const fn with_aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
        self.inner.aspectMask = aspect_mask.bits();
        self
    }

    pub const fn with_mip_level(mut self, base: u32, count: u32) -> Self {
        self.inner.baseMipLevel = base;
        self.inner.levelCount = count;
        self
    }

    pub const fn with_array_layer(mut self, base: u32, count: u32) -> Self {
        self.inner.baseArrayLayer = base;
        self.inner.layerCount = count;
        self
    }
}

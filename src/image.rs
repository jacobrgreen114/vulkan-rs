// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Image, VkImage);

vulkan_create_info_lifetime!(
    ImageCreateInfo,
    VkImageCreateInfo,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO
);

impl<'a> ImageCreateInfo<'a> {
    pub fn with_image_type(mut self, type_: ImageType) -> Self {
        self.inner.imageType = type_.as_raw();
        self
    }

    pub fn with_image_format(mut self, format: Format) -> Self {
        self.inner.format = format.as_raw();
        self
    }

    pub fn with_extent(mut self, extent: Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }

    pub fn with_mip_levels(mut self, levels: u32) -> Self {
        self.inner.mipLevels = levels;
        self
    }

    pub fn with_array_layers(mut self, layers: u32) -> Self {
        self.inner.arrayLayers = layers;
        self
    }

    pub fn with_samples(mut self, samples: SampleCountFlags) -> Self {
        self.inner.samples = samples.bits() as _;
        self
    }

    pub fn with_tiling(mut self, tiling: ImageTiling) -> Self {
        self.inner.tiling = tiling.as_raw();
        self
    }

    pub fn with_usage(mut self, usage: ImageUsageFlags) -> Self {
        self.inner.usage = usage.bits() as _;
        self
    }

    pub fn with_initial_layout(mut self, layout: ImageLayout) -> Self {
        self.inner.initialLayout = layout.as_raw();
        self
    }
}

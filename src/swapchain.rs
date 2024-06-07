// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

/*
   Swapchain
*/

vulkan_handle!(SwapchainKHR, VkSwapchainKHR);

vulkan_create_info_lifetime!(
    SwapchainCreateInfoKHR,
    VkSwapchainCreateInfoKHR,
    VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR
);

impl<'a> SwapchainCreateInfoKHR<'a> {
    pub const fn with_surface(mut self, surface: SurfaceKHR) -> Self {
        self.inner.surface = surface.as_raw();
        self
    }

    pub const fn with_min_image_count(mut self, min_image_count: u32) -> Self {
        self.inner.minImageCount = min_image_count;
        self
    }

    pub const fn with_image_format(mut self, image_format: Format) -> Self {
        self.inner.imageFormat = image_format as i32;
        self
    }

    pub const fn with_image_color_space(mut self, image_color_space: ColorSpaceKHR) -> Self {
        self.inner.imageColorSpace = image_color_space as i32;
        self
    }

    pub const fn with_image_extent(mut self, image_extent: Extent2D) -> Self {
        self.inner.imageExtent = image_extent;
        self
    }

    pub const fn with_image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.inner.imageArrayLayers = image_array_layers;
        self
    }

    pub const fn with_image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
        self.inner.imageUsage = image_usage.bits();
        self
    }

    pub const fn with_image_sharing_mode(mut self, image_sharing_mode: SharingMode) -> Self {
        self.inner.imageSharingMode = image_sharing_mode as i32 as VkSharingMode;
        self
    }

    pub const fn with_queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.inner.queueFamilyIndexCount = queue_family_indices.len() as u32;
        self.inner.pQueueFamilyIndices = queue_family_indices.as_ptr();
        self
    }

    pub const fn with_pre_transform(mut self, pre_transform: SurfaceTransformFlagsKHR) -> Self {
        self.inner.preTransform = pre_transform.bits() as i32;
        self
    }

    pub const fn with_composite_alpha(mut self, composite_alpha: CompositeAlphaFlagsKHR) -> Self {
        self.inner.compositeAlpha = composite_alpha.bits() as i32;
        self
    }

    pub const fn with_present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.inner.presentMode = present_mode as i32;
        self
    }

    pub const fn with_clipped(mut self, clipped: bool) -> Self {
        self.inner.clipped = if clipped { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn with_old_swapchain(mut self, old_swapchain: Option<SwapchainKHR>) -> Self {
        self.inner.oldSwapchain = old_swapchain
            .map(|s| s.as_raw())
            .unwrap_or(std::ptr::null_mut());
        self
    }
}

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(Sampler, VkSampler);

vulkan_create_info_lifetime!(
    SamplerCreateInfo,
    VkSamplerCreateInfo,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO
);

impl<'a> SamplerCreateInfo<'a> {
    pub const fn with_flags(mut self, flags: SamplerCreateFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    pub const fn with_mag_filter(mut self, mag: Filter) -> Self {
        self.inner.magFilter = mag.as_raw();
        self
    }

    pub const fn with_min_filter(mut self, min: Filter) -> Self {
        self.inner.minFilter = min.as_raw();
        self
    }

    pub const fn with_mipmap_mode(mut self, mode: SamplerMipmapMode) -> Self {
        self.inner.mipmapMode = mode.as_raw();
        self
    }

    pub const fn with_address_mode(self, mode: SamplerAddressMode) -> Self {
        self.with_address_mode_uvw(mode, mode, mode)
    }

    pub const fn with_address_mode_uvw(
        mut self,
        u: SamplerAddressMode,
        v: SamplerAddressMode,
        w: SamplerAddressMode,
    ) -> Self {
        self.inner.addressModeU = u.as_raw();
        self.inner.addressModeV = v.as_raw();
        self.inner.addressModeW = w.as_raw();
        self
    }

    pub const fn with_mip_lod_bias(mut self, bias: f32) -> Self {
        self.inner.mipLodBias = bias;
        self
    }

    pub const fn with_anisotropy_enable(mut self, enable: bool) -> Self {
        self.inner.anisotropyEnable = enable as _;
        self
    }

    pub const fn with_max_anisotropy(mut self, max: f32) -> Self {
        self.inner.maxAnisotropy = max;
        self
    }

    pub const fn with_compare_enable(mut self, enable: bool) -> Self {
        self.inner.compareEnable = enable as _;
        self
    }

    pub const fn with_compare_op(mut self, op: CompareOp) -> Self {
        self.inner.compareOp = op.as_raw();
        self
    }

    pub const fn with_min_lod(mut self, lod: f32) -> Self {
        self.inner.minLod = lod;
        self
    }

    pub const fn with_max_lod(mut self, lod: f32) -> Self {
        self.inner.maxLod = lod;
        self
    }

    pub const fn with_border_color(mut self, color: BorderColor) -> Self {
        self.inner.borderColor = color.as_raw();
        self
    }

    pub const fn with_unnormalized_coordinates(mut self, unnormalized: bool) -> Self {
        self.inner.unnormalizedCoordinates = unnormalized as _;
        self
    }
}

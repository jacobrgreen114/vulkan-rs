// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(DescriptorPool, VkDescriptorPool);

pub struct DescriptorPoolSize {
    inner: VkDescriptorPoolSize,
}

vulkan_struct_custom!(DescriptorPoolSize, VkDescriptorPoolSize);

impl DescriptorPoolSize {
    pub const fn new(type_: DescriptorType, descriptor_count: u32) -> Self {
        Self {
            inner: VkDescriptorPoolSize {
                type_: type_.as_raw(),
                descriptorCount: descriptor_count,
            },
        }
    }
}

vulkan_create_info_lifetime!(
    DescriptorPoolCreateInfo,
    VkDescriptorPoolCreateInfo,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO
);

impl<'a> DescriptorPoolCreateInfo<'a> {
    pub const fn with_max_sets(mut self, max_sets: u32) -> Self {
        self.inner.maxSets = max_sets;
        self
    }

    pub const fn with_pool_sizes(mut self, pool_sizes: &[DescriptorPoolSize]) -> Self {
        self.inner.poolSizeCount = pool_sizes.len() as u32;
        self.inner.pPoolSizes = pool_sizes.as_ptr().cast();
        self
    }
}

vulkan_create_info_lifetime!(
    DescriptorSetAllocateInfo,
    VkDescriptorSetAllocateInfo,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO
);

impl<'a> DescriptorSetAllocateInfo<'a> {
    pub const fn with_descriptor_pool(mut self, descriptor_pool: DescriptorPool) -> Self {
        self.inner.descriptorPool = descriptor_pool.as_raw();
        self
    }

    pub const fn with_set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
        self.inner.descriptorSetCount = set_layouts.len() as u32;
        self.inner.pSetLayouts = set_layouts.as_ptr().cast();
        self
    }
}

vulkan_handle!(DescriptorSet, VkDescriptorSet);

vulkan_create_info_lifetime!(
    WriteDescriptorSet,
    VkWriteDescriptorSet,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET
);

impl<'a> WriteDescriptorSet<'a> {
    pub const fn with_dst_set(mut self, dst_set: DescriptorSet) -> Self {
        self.inner.dstSet = dst_set.as_raw();
        self
    }

    pub const fn with_dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dstBinding = dst_binding;
        self
    }

    pub const fn with_dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dstArrayElement = dst_array_element;
        self
    }

    pub const fn with_descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
        self.inner.descriptorType = descriptor_type.as_raw();
        self
    }

    pub const fn with_image_infos(mut self, image_info: &'a [DescriptorImageInfo]) -> Self {
        self.inner.descriptorCount = image_info.len() as u32;
        self.inner.pImageInfo = image_info.as_ptr().cast();
        self
    }

    pub const fn with_buffer_infos(mut self, buffer_info: &'a [DescriptorBufferInfo]) -> Self {
        self.inner.descriptorCount = buffer_info.len() as u32;
        self.inner.pBufferInfo = buffer_info.as_ptr().cast();
        self
    }

    // pub const fn with_texel_buffer_view(mut self, texel_buffer_view: &'a [VkBufferView]) -> Self {
    //     self.inner.descriptorCount = texel_buffer_view.len() as u32;
    //     self.inner.pTexelBufferView = texel_buffer_view.as_ptr();
    //     self
    // }
}

vulkan_struct_no_new!(DescriptorImageInfo, VkDescriptorImageInfo);

impl DescriptorImageInfo {
    pub fn new(
        sampler: Option<Sampler>,
        image_view: Option<ImageView>,
        image_layout: ImageLayout,
    ) -> Self {
        Self {
            inner: VkDescriptorImageInfo {
                sampler: sampler.map(|s| s.as_raw()).unwrap_or(std::ptr::null_mut()),
                imageView: image_view
                    .map(|v| v.as_raw())
                    .unwrap_or(std::ptr::null_mut()),
                imageLayout: image_layout.as_raw(),
            },
        }
    }
}

vulkan_struct_no_new!(DescriptorBufferInfo, VkDescriptorBufferInfo);

impl DescriptorBufferInfo {
    pub const fn new(buffer: Buffer, offset: DeviceSize, range: DeviceSize) -> Self {
        Self {
            inner: VkDescriptorBufferInfo {
                buffer: buffer.as_raw(),
                offset,
                range,
            },
        }
    }
}

vulkan_create_info_lifetime!(
    CopyDescriptorSet,
    VkCopyDescriptorSet,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET
);

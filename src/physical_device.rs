// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use std::ffi::CStr;
use std::intrinsics::transmute;
use std::ops::Deref;
use vulkan_sys::*;

use vulkan_sys::wrapper as vk_wrap;

/*
   Physical Device
*/

vulkan_handle!(PhysicalDevice, VkPhysicalDevice);

impl PhysicalDevice {
    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        unsafe {
            transmute(vk_wrap::get_physical_device_properties(
                vkGetPhysicalDeviceProperties,
                self.as_raw(),
            ))
        }
    }

    pub fn get_features(&self) -> PhysicalDeviceFeatures {
        unsafe {
            transmute(vk_wrap::get_physical_device_features(
                vkGetPhysicalDeviceFeatures,
                self.as_raw(),
            ))
        }
    }

    pub fn get_queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        unsafe {
            transmute(vk_wrap::get_physical_device_queue_family_properties(
                vkGetPhysicalDeviceQueueFamilyProperties,
                self.as_raw(),
            ))
        }
    }

    pub fn get_memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            transmute(vk_wrap::get_physical_device_memory_properties(
                vkGetPhysicalDeviceMemoryProperties,
                self.as_raw(),
            ))
        }
    }

    pub fn get_surface_support(
        &self,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> vk_wrap::Result<bool> {
        vk_wrap::get_physical_device_surface_support_khr(
            vkGetPhysicalDeviceSurfaceSupportKHR,
            self.as_raw(),
            queue_family_index,
            surface.as_raw(),
        )
    }

    pub fn get_surface_capabilities(
        &self,
        surface: SurfaceKHR,
    ) -> vk_wrap::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            transmute(vk_wrap::get_physical_device_surface_capabilities_khr(
                vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
                self.as_raw(),
                surface.as_raw(),
            ))
        }
    }

    pub fn get_surface_formats(
        &self,
        surface: SurfaceKHR,
    ) -> vk_wrap::Result<Vec<SurfaceFormatKHR>> {
        unsafe {
            transmute(vk_wrap::get_physical_device_surface_formats_khr(
                vkGetPhysicalDeviceSurfaceFormatsKHR,
                self.as_raw(),
                surface.as_raw(),
            ))
        }
    }

    pub fn get_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> vk_wrap::Result<Vec<PresentModeKHR>> {
        unsafe {
            transmute(vk_wrap::get_physical_device_surface_present_modes_khr(
                vkGetPhysicalDeviceSurfacePresentModesKHR,
                self.as_raw(),
                surface.as_raw(),
            ))
        }
    }
}

/*
   Physical Device Properties
*/

vulkan_struct!(PhysicalDeviceProperties, VkPhysicalDeviceProperties);

impl PhysicalDeviceProperties {
    pub const fn api_version(&self) -> ApiVersion {
        ApiVersion::from_raw(self.inner.apiVersion)
    }

    pub const fn driver_version(&self) -> u32 {
        self.inner.driverVersion
    }

    pub const fn vendor_id(&self) -> u32 {
        self.inner.vendorID
    }

    pub const fn device_id(&self) -> u32 {
        self.inner.deviceID
    }

    pub const fn device_type(&self) -> PhysicalDeviceType {
        PhysicalDeviceType::from_raw(self.inner.deviceType)
    }

    pub fn device_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.inner.deviceName.as_ptr()) }
    }

    pub const fn limits(&self) -> &PhysicalDeviceLimits {
        unsafe { transmute(&self.inner.limits) }
    }
}

/*
    Physical Device Limits
*/

vulkan_struct!(PhysicalDeviceLimits, VkPhysicalDeviceLimits);

impl Deref for PhysicalDeviceLimits {
    type Target = VkPhysicalDeviceLimits;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/*
   Physical Device Features
*/

vulkan_struct!(PhysicalDeviceFeatures, VkPhysicalDeviceFeatures);

// impl Deref for PhysicalDeviceFeatures {
//     type Target = VkPhysicalDeviceFeatures;
//
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

impl PhysicalDeviceFeatures {
    pub const fn robust_buffer_access(&self) -> bool {
        self.inner.robustBufferAccess != 0
    }

    pub const fn full_draw_index_uint32(&self) -> bool {
        self.inner.fullDrawIndexUint32 != 0
    }

    pub const fn image_cube_array(&self) -> bool {
        self.inner.imageCubeArray != 0
    }

    pub const fn independent_blend(&self) -> bool {
        self.inner.independentBlend != 0
    }

    pub const fn geometry_shader(&self) -> bool {
        self.inner.geometryShader != 0
    }

    pub const fn tessellation_shader(&self) -> bool {
        self.inner.tessellationShader != 0
    }

    pub const fn sample_rate_shading(&self) -> bool {
        self.inner.sampleRateShading != 0
    }

    pub const fn dual_src_blend(&self) -> bool {
        self.inner.dualSrcBlend != 0
    }

    pub const fn logic_op(&self) -> bool {
        self.inner.logicOp != 0
    }

    pub const fn multi_draw_indirect(&self) -> bool {
        self.inner.multiDrawIndirect != 0
    }

    pub const fn draw_indirect_first_instance(&self) -> bool {
        self.inner.drawIndirectFirstInstance != 0
    }

    pub const fn depth_clamp(&self) -> bool {
        self.inner.depthClamp != 0
    }

    pub const fn depth_bias_clamp(&self) -> bool {
        self.inner.depthBiasClamp != 0
    }

    pub const fn fill_mode_non_solid(&self) -> bool {
        self.inner.fillModeNonSolid != 0
    }

    pub const fn depth_bounds(&self) -> bool {
        self.inner.depthBounds != 0
    }

    pub const fn wide_lines(&self) -> bool {
        self.inner.wideLines != 0
    }

    pub const fn large_points(&self) -> bool {
        self.inner.largePoints != 0
    }

    pub const fn alpha_to_one(&self) -> bool {
        self.inner.alphaToOne != 0
    }

    pub const fn multi_viewport(&self) -> bool {
        self.inner.multiViewport != 0
    }

    pub const fn sampler_anisotropy(&self) -> bool {
        self.inner.samplerAnisotropy != 0
    }

    pub const fn texture_compression_etc2(&self) -> bool {
        self.inner.textureCompressionETC2 != 0
    }

    pub const fn texture_compression_astc_ldr(&self) -> bool {
        self.inner.textureCompressionASTC_LDR != 0
    }

    pub const fn texture_compression_bc(&self) -> bool {
        self.inner.textureCompressionBC != 0
    }

    pub const fn occlusion_query_precise(&self) -> bool {
        self.inner.occlusionQueryPrecise != 0
    }

    pub const fn pipeline_statistics_query(&self) -> bool {
        self.inner.pipelineStatisticsQuery != 0
    }

    pub const fn vertex_pipeline_stores_and_atomics(&self) -> bool {
        self.inner.vertexPipelineStoresAndAtomics != 0
    }

    pub const fn fragment_stores_and_atomics(&self) -> bool {
        self.inner.fragmentStoresAndAtomics != 0
    }

    pub const fn shader_tessellation_and_geometry_point_size(&self) -> bool {
        self.inner.shaderTessellationAndGeometryPointSize != 0
    }

    pub const fn shader_image_gather_extended(&self) -> bool {
        self.inner.shaderImageGatherExtended != 0
    }

    pub const fn shader_storage_image_extended_formats(&self) -> bool {
        self.inner.shaderStorageImageExtendedFormats != 0
    }

    pub const fn shader_storage_image_multisample(&self) -> bool {
        self.inner.shaderStorageImageMultisample != 0
    }

    pub const fn shader_storage_image_read_without_format(&self) -> bool {
        self.inner.shaderStorageImageReadWithoutFormat != 0
    }

    pub const fn shader_storage_image_write_without_format(&self) -> bool {
        self.inner.shaderStorageImageWriteWithoutFormat != 0
    }

    pub const fn shader_uniform_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner.shaderUniformBufferArrayDynamicIndexing != 0
    }

    pub const fn shader_sampled_image_array_dynamic_indexing(&self) -> bool {
        self.inner.shaderSampledImageArrayDynamicIndexing != 0
    }

    pub const fn shader_storage_buffer_array_dynamic_indexing(&self) -> bool {
        self.inner.shaderStorageBufferArrayDynamicIndexing != 0
    }

    pub const fn shader_storage_image_array_dynamic_indexing(&self) -> bool {
        self.inner.shaderStorageImageArrayDynamicIndexing != 0
    }

    pub const fn shader_clip_distance(&self) -> bool {
        self.inner.shaderClipDistance != 0
    }

    pub const fn shader_cull_distance(&self) -> bool {
        self.inner.shaderCullDistance != 0
    }

    pub const fn shader_float64(&self) -> bool {
        self.inner.shaderFloat64 != 0
    }

    pub const fn shader_int64(&self) -> bool {
        self.inner.shaderInt64 != 0
    }

    pub const fn shader_int16(&self) -> bool {
        self.inner.shaderInt16 != 0
    }

    pub const fn shader_resource_residency(&self) -> bool {
        self.inner.shaderResourceResidency != 0
    }

    pub const fn shader_resource_min_lod(&self) -> bool {
        self.inner.shaderResourceMinLod != 0
    }

    pub const fn sparse_binding(&self) -> bool {
        self.inner.sparseBinding != 0
    }

    pub const fn sparse_residency_buffer(&self) -> bool {
        self.inner.sparseResidencyBuffer != 0
    }

    pub const fn sparse_residency_image_2d(&self) -> bool {
        self.inner.sparseResidencyImage2D != 0
    }

    pub const fn sparse_residency_image_3d(&self) -> bool {
        self.inner.sparseResidencyImage3D != 0
    }

    pub const fn sparse_residency_2_samples(&self) -> bool {
        self.inner.sparseResidency2Samples != 0
    }

    pub const fn sparse_residency_4_samples(&self) -> bool {
        self.inner.sparseResidency4Samples != 0
    }

    pub const fn sparse_residency_8_samples(&self) -> bool {
        self.inner.sparseResidency8Samples != 0
    }

    pub const fn sparse_residency_16_samples(&self) -> bool {
        self.inner.sparseResidency16Samples != 0
    }

    pub const fn sparse_residency_aliased(&self) -> bool {
        self.inner.sparseResidencyAliased != 0
    }

    pub const fn variable_multisample_rate(&self) -> bool {
        self.inner.variableMultisampleRate != 0
    }

    pub const fn inherited_queries(&self) -> bool {
        self.inner.inheritedQueries != 0
    }
}

/*
   Queue Family Properties
*/

vulkan_struct!(QueueFamilyProperties, VkQueueFamilyProperties);

impl QueueFamilyProperties {
    pub const fn queue_flags(&self) -> QueueFlags {
        unsafe { transmute(self.inner.queueFlags) }
    }

    pub const fn queue_count(&self) -> u32 {
        self.inner.queueCount
    }

    pub const fn timestamp_valid_bits(&self) -> u32 {
        self.inner.timestampValidBits
    }

    pub const fn min_image_transfer_granularity(&self) -> Extent3D {
        unsafe { transmute(self.inner.minImageTransferGranularity) }
    }
}

/*
   Physical Device Memory Properties
*/

vulkan_struct!(MemoryType, VkMemoryType);

impl MemoryType {
    pub const fn property_flags(&self) -> MemoryPropertyFlags {
        unsafe { transmute(self.inner.propertyFlags) }
    }

    pub const fn heap_index(&self) -> u32 {
        self.inner.heapIndex
    }
}

vulkan_struct!(MemoryHeap, VkMemoryHeap);

impl MemoryHeap {
    pub const fn size(&self) -> u64 {
        self.inner.size
    }

    pub const fn flags(&self) -> MemoryHeapFlags {
        unsafe { transmute(self.inner.flags) }
    }
}

vulkan_struct!(
    PhysicalDeviceMemoryProperties,
    VkPhysicalDeviceMemoryProperties
);

impl PhysicalDeviceMemoryProperties {
    pub const fn memory_type_count(&self) -> u32 {
        self.inner.memoryTypeCount
    }

    pub const fn memory_types(&self) -> &[MemoryType] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.memoryTypes.as_ptr().cast(),
                self.inner.memoryTypeCount as usize,
            )
        }
    }

    pub const fn memory_heap_count(&self) -> u32 {
        self.inner.memoryHeapCount
    }

    pub const fn memory_heaps(&self) -> &[MemoryHeap] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.memoryHeaps.as_ptr().cast(),
                self.inner.memoryHeapCount as usize,
            )
        }
    }
}

/*
   Surface Capabilities
*/

vulkan_struct!(SurfaceCapabilitiesKHR, VkSurfaceCapabilitiesKHR);

impl SurfaceCapabilitiesKHR {
    pub const fn min_image_count(&self) -> u32 {
        self.inner.minImageCount
    }

    pub const fn max_image_count(&self) -> u32 {
        self.inner.maxImageCount
    }

    pub const fn current_extent(&self) -> Extent2D {
        unsafe { transmute(self.inner.currentExtent) }
    }

    pub const fn min_image_extent(&self) -> Extent2D {
        unsafe { transmute(self.inner.minImageExtent) }
    }

    pub const fn max_image_extent(&self) -> Extent2D {
        unsafe { transmute(self.inner.maxImageExtent) }
    }

    pub const fn max_image_array_layers(&self) -> u32 {
        self.inner.maxImageArrayLayers
    }

    pub const fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        unsafe { transmute(self.inner.supportedTransforms) }
    }

    pub const fn current_transform(&self) -> SurfaceTransformFlagsKHR {
        unsafe { transmute(self.inner.currentTransform) }
    }

    pub const fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        unsafe { transmute(self.inner.supportedCompositeAlpha) }
    }

    pub const fn supported_usage_flags(&self) -> ImageUsageFlags {
        unsafe { transmute(self.inner.supportedUsageFlags) }
    }
}

/*
   Surface Format
*/

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}

vulkan_struct_custom!(SurfaceFormatKHR, VkSurfaceFormatKHR);

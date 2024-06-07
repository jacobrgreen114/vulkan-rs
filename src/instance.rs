// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::*;
use std::ffi::{c_char, CStr};
use std::intrinsics::transmute;
use vulkan_sys::wrapper::*;
use vulkan_sys::*;

use crate::macros::*;

/*
   Application Info
*/

vulkan_create_info_lifetime!(
    ApplicationInfo,
    VkApplicationInfo,
    VK_STRUCTURE_TYPE_APPLICATION_INFO
);

impl<'a> ApplicationInfo<'a> {
    pub const fn with_application_name(mut self, name: &'a CStr) -> Self {
        self.inner.pApplicationName = name.as_ptr();
        self
    }

    pub const fn with_engine_name(mut self, name: &'a CStr) -> Self {
        self.inner.pEngineName = name.as_ptr();
        self
    }

    pub const fn with_api_version(mut self, version: ApiVersion) -> Self {
        self.inner.apiVersion = version.as_raw();
        self
    }
}

/*
   Instance Create Info
*/

vulkan_create_info_lifetime!(
    InstanceCreateInfo,
    VkInstanceCreateInfo,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
);

impl<'a> InstanceCreateInfo<'a> {
    pub const fn with_application_info(mut self, app_info: &'a ApplicationInfo) -> Self {
        self.inner.pApplicationInfo = app_info.as_raw();
        self
    }

    pub const fn with_enabled_layers(mut self, layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabledLayerCount = layer_names.len() as u32;
        self.inner.ppEnabledLayerNames = layer_names.as_ptr();
        self
    }

    pub const fn with_enabled_extensions(mut self, extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabledExtensionCount = extension_names.len() as u32;
        self.inner.ppEnabledExtensionNames = extension_names.as_ptr();
        self
    }
}

/*
   Instance
*/

vulkan_handle!(Instance, VkInstance);

impl Instance {
    pub fn create(
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Self> {
        create_instance(
            vkCreateInstance,
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(Self::from_raw)
    }

    pub fn destroy(&self, allocator: Option<&AllocationCallbacks>) {
        destroy_instance(
            vkDestroyInstance,
            self.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn enumerate_physical_devices(&self) -> vulkan_sys::wrapper::Result<Vec<PhysicalDevice>> {
        enumerate_physical_devices(vkEnumeratePhysicalDevices, self.as_raw())
            .map(|devices| unsafe { transmute(devices) })
    }

    pub fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        PhysicalDeviceProperties::from_raw(get_physical_device_properties(
            vkGetPhysicalDeviceProperties,
            physical_device.as_raw(),
        ))
    }

    pub fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        PhysicalDeviceFeatures::from_raw(get_physical_device_features(
            vkGetPhysicalDeviceFeatures,
            physical_device.as_raw(),
        ))
    }

    #[cfg(target_os = "windows")]
    pub fn create_win32_surface_khr(
        &self,
        create_info: &Win32SurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<SurfaceKHR> {
        create_win32_surface_khr(
            vkCreateWin32SurfaceKHR,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(SurfaceKHR::from_raw)
    }

    pub fn destroy_surface_khr(
        &self,
        surface: SurfaceKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_surface_khr(
            vkDestroySurfaceKHR,
            self.as_raw(),
            surface.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }
}

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;

use vulkan_sys::*;

/*
   Surface
*/

vulkan_create_info!(
    Win32SurfaceCreateInfoKHR,
    VkWin32SurfaceCreateInfoKHR,
    VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR
);

impl crate::Win32SurfaceCreateInfoKHR {
    pub const fn with_hinstance(
        mut self,
        hinstance: windows::Win32::Foundation::HINSTANCE,
    ) -> Self {
        self.inner.hinstance = hinstance;
        self
    }

    pub const fn with_hwnd(mut self, hwnd: windows::Win32::Foundation::HWND) -> Self {
        self.inner.hwnd = hwnd;
        self
    }
}

vulkan_handle!(SurfaceKHR, VkSurfaceKHR);

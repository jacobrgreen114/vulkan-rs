// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#[macro_use]
extern crate static_assertions;

mod macros;

mod enums;
pub use enums::*;

mod instance;
pub use instance::*;

mod physical_device;
pub use physical_device::*;

mod surface;
pub use surface::*;

mod device;
pub use device::*;

mod queue;
pub use queue::*;

mod swapchain;
pub use swapchain::*;

mod buffer;
pub use buffer::*;

mod image;
pub use image::*;

mod image_view;
pub use image_view::*;

mod sampler;
pub use sampler::*;

mod render_pass;
pub use render_pass::*;

mod framebuffer;
pub use framebuffer::*;

mod pipeline;
pub use pipeline::*;

mod command;
pub use command::*;

mod fence;
pub use fence::*;

mod semaphore;
pub use semaphore::*;

use sys::*;
pub use vulkan_sys as sys;

pub use sys::wrapper::Error;
pub use sys::wrapper::Result;

use sys::wrapper::*;

use bitfield::bitfield;
use std::ffi::{c_char, CStr};
use std::mem::transmute;

use macros::*;

pub type Offset2D = VkOffset2D;
pub type Offset3D = VkOffset3D;
pub type Extent2D = VkExtent2D;
pub type Extent3D = VkExtent3D;
pub type Rect2D = VkRect2D;
pub type Viewport = VkViewport;

pub type ComponentMapping = VkComponentMapping;

pub type SampleMask = VkSampleMask;

pub type StencilOpState = VkStencilOpState;

pub type DeviceSize = VkDeviceSize;

pub type ClearValue = VkClearValue;
pub type ClearColorValue = VkClearColorValue;
pub type ClearDepthStencilValue = VkClearDepthStencilValue;

pub const WHOLE_SIZE: DeviceSize = VK_WHOLE_SIZE as DeviceSize;

/*
   Allocation Callbacks
*/

pub struct InternalAllocationCallbacks {
    pub internal_allocation: PFN_vkInternalAllocationNotification,
    pub internal_free: PFN_vkInternalFreeNotification,
}

pub struct AllocationCallbacks<'a> {
    pub user_data: &'a ::std::os::raw::c_void,
    pub allocation: PFN_vkAllocationFunction,
    pub reallocation: PFN_vkReallocationFunction,
    pub free: PFN_vkFreeFunction,
    pub internal_callbacks: Option<InternalAllocationCallbacks>,
}

assert_eq_size!(AllocationCallbacks, VkAllocationCallbacks);

impl<'a> AllocationCallbacks<'a> {
    pub const fn as_raw(&self) -> &VkAllocationCallbacks {
        unsafe { transmute(self) }
    }

    //fn validate(&self) {
    //    macro_rules! validate_fn {
    //        ($fn:expr) => {
    //            assert_ne!(
    //                unsafe { std::mem::transmute::<_, *const std::ffi::c_void>($fn) },
    //                std::ptr::null::<std::ffi::c_void>()
    //            );
    //        };
    //    }
    //
    //    validate_fn!(self.allocation);
    //    validate_fn!(self.reallocation);
    //    validate_fn!(self.free);
    //}
}

macro_rules! make_api_version {
    ($variant:expr, $major:expr, $minor:expr, $patch:expr) => {
        ((($variant as u32) << 29)
            | (($major as u32) << 22)
            | (($minor as u32) << 12)
            | ($patch as u32))
    };
}

// macro_rules! api_version_variant {
//     ($version:expr) => {
//         ($version >> 29)
//     };
// }
//
// macro_rules! api_version_major {
//     ($version:expr) => {
//         (($version >> 22) & 0x7F)
//     };
// }
//
// macro_rules! api_version_minor {
//     ($version:expr) => {
//         (($version >> 12) & 0x3FF)
//     };
// }
//
// macro_rules! api_version_patch {
//     ($version:expr) => {
//         ($version & 0xFFF)
//     };
// }

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ApiVersion(u32);
    u8, variant, set_variant: 31, 29;
    u8, major, set_major: 28, 22;
    u16, minor, set_minor: 21, 12;
    u16, patch, set_patch: 11, 0;
}

impl ApiVersion {
    pub const fn make(variant: u8, major: u8, minor: u16, patch: u16) -> Self {
        Self(make_api_version!(variant, major, minor, patch))
    }

    pub const fn from_raw(version: u32) -> Self {
        Self(version)
    }

    pub const fn as_raw(&self) -> u32 {
        self.0
    }

    pub const VERSION_1_0: Self = Self::make(0, 1, 0, 0);
    pub const VERSION_1_1: Self = Self::make(0, 1, 1, 0);
    pub const VERSION_1_2: Self = Self::make(0, 1, 2, 0);
    pub const VERSION_1_3: Self = Self::make(0, 1, 3, 0);
}

impl std::fmt::Debug for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiVersion")
            .field("variant", &self.variant())
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .finish()
    }
}

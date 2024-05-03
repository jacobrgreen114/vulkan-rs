// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

mod descriptor_set_layout;
pub use descriptor_set_layout::*;

mod descriptor_set;
pub use descriptor_set::*;

mod pipeline_layout;
pub use pipeline_layout::*;

mod shader;
pub use shader::*;

mod graphics;
pub use graphics::*;

vulkan_handle!(Pipeline, VkPipeline);
vulkan_handle!(PipelineCache, VkPipelineCache);

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use bindgen;
use bindgen::callbacks::EnumVariantValue;
use std::collections::HashMap;
use std::env::var;
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

macro_rules! cargo_warning {
    ($($arg:tt)*) => {
        println!("cargo:warning={}", format!($($arg)*));
    };
}

// macro_rules! cargo_panic {
//     ($($arg:tt)*) => {
//         cargo_warning!($($arg)*);
//         panic!($($arg)*);
//     };
// }

type Enum = (String, EnumVariantValue);
type EnumVec = Vec<Enum>;

type EnumMap = HashMap<String, EnumVec>;

fn main() {
    let vulkan_sdk_path = PathBuf::from(var("VULKAN_SDK").expect("VULKAN_SDK not set"));
    let vulkan_include_dir: PathBuf = vulkan_sdk_path.join("Include");

    let vulkan_header_rel_path = PathBuf::from("vulkan").join("vulkan.h");
    let vulkan_header_path = vulkan_include_dir.join(vulkan_header_rel_path);

    (!vulkan_header_path.exists()).then(|| {
        panic!("Vulkan header not found at {:?}", vulkan_header_path);
    });

    let enum_map = Arc::new(Mutex::new(EnumMap::new()));

    bindgen::builder()
        .parse_callbacks(Box::new(FormatCallback {
            enum_map: enum_map.clone(),
            cargo_callbacks: bindgen::CargoCallbacks::new(),
        }))
        .clang_args(&["-I", vulkan_include_dir.to_str().unwrap()])
        .header("./src/wrapper.h")
        .allowlist_recursively(false)
        .allowlist_file(".*vulkan.*")
        .allowlist_file(".*vk_video.*")
        .allowlist_type("Vk.*")
        .blocklist_function(".*")
        .blocklist_var(".*")
        .blocklist_type(".*")
        .blocklist_item(".*")
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());

    let prefix_map = build_config_map();

    {
        let enums_path = out_path.join("enums.rs");
        let mut enums_file = std::fs::File::create(&enums_path).unwrap();

        for (enum_name, enum_vec) in enum_map
            .lock()
            .unwrap()
            .iter()
            .map(|kv| (kv.0.as_str(), kv.1))
        {
            let config = match prefix_map.get(enum_name) {
                Some(config) => config,
                None => {
                    cargo_warning!("No prefix found for enum: {}", enum_name);
                    continue;
                }
            };

            if !config.is_flags {
                write_enum(&mut enums_file, config, enum_vec.iter(), None);
            } else {
                write_flags(&mut enums_file, config, enum_vec.iter());
            }
        }
    }
}

fn format_enum_name(name: &str) -> String {
    name.trim_start_matches("Vk").to_string()
}

fn format_enum_variant_name(prefix: &str, name: &str) -> Option<String> {
    let mut formatted = match name.strip_prefix(prefix) {
        Some(stripped) => stripped,
        None => {
            cargo_warning!("Failed to strip prefix from enum: {}", name);
            return None;
        }
    };
    if formatted.chars().next().unwrap().is_digit(10) {
        formatted = name.strip_prefix(&prefix[..prefix.len() - 1]).unwrap();
    }
    Some(formatted.to_string())
}

fn format_flag_enum_name(name: &str) -> String {
    name.trim_start_matches("Vk")
        .to_string()
        .replace("FlagBits", "Flags")
}

fn format_flag_variant_name(prefix: &str, name: &str) -> Option<String> {
    let mut formatted = match name.strip_prefix(prefix) {
        Some(stripped) => stripped,
        None => return None,
    };
    if formatted.chars().next().unwrap().is_digit(10) {
        formatted = name.strip_prefix(&prefix[..prefix.len() - 1]).unwrap();
    }
    let formatted = formatted.replace("_BIT", "");
    Some(formatted.to_string())
}

fn filter_enum_variant(name: &&str) -> bool {
    !name.contains("MAX_ENUM")
}

fn write_enum<'a, W: Write, I: Iterator<Item = &'a Enum>>(
    writer: &mut W,
    enum_config: &EnumConfig,
    variants: I,
    skip: Option<&[&str]>,
) {
    let new_name = enum_config
        .custom_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| format_enum_name(enum_config.name));

    writeln!(writer, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").unwrap();
    writeln!(writer, "#[repr(i32)]").unwrap();
    writeln!(writer, "pub enum {} {{", new_name).unwrap();
    for variant in variants.map(|e| e.0.as_str()).filter(filter_enum_variant) {
        // cargo_warning!("{}: {}", enum_name, variant);
        if let Some(skip) = skip {
            if skip.contains(&variant) {
                continue;
            }
        }

        let formatted = match format_enum_variant_name(enum_config.prefix, variant) {
            Some(formatted) => formatted,
            None => continue,
        };

        writeln!(writer, "    {} = {},", formatted, variant).unwrap();
    }
    writeln!(writer, "}}").unwrap();
    writeln!(
        writer,
        "assert_eq_size!({}, {});",
        new_name, enum_config.name
    )
    .unwrap();

    writeln!(writer, "impl {} {{", new_name).unwrap();
    writeln!(
        writer,
        "    pub const fn from_raw(value: i32) -> Self {{ unsafe {{ std::mem::transmute(value) }} }}"
    )
    .unwrap();
    writeln!(
        writer,
        "    pub const fn as_raw(&self) -> i32 {{ *self as i32 }}"
    )
    .unwrap();
    writeln!(writer, "}}").unwrap();
    writeln!(writer).unwrap();
}

fn write_flags<'a, W: Write, I: Iterator<Item = &'a Enum>>(
    writer: &mut W,
    enum_config: &EnumConfig,
    variants: I,
) {
    let enum_name = enum_config
        .custom_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| format_flag_enum_name(enum_config.name));

    writeln!(writer, "bitflags! {{").unwrap();
    writeln!(writer, "    #[derive(Default, Clone, Copy, PartialEq, Eq)]").unwrap();
    writeln!(writer, "    pub struct {}: u32 {{", enum_name).unwrap();

    for variant in variants.map(|e| e.0.as_str()).filter(filter_enum_variant) {
        // cargo_warning!("{}: {}", enum_name, variant);
        if variant.contains("MAX_ENUM") {
            continue;
        }

        let formatted = match format_flag_variant_name(enum_config.prefix, variant) {
            Some(formatted) => formatted,
            None => continue,
        };

        writeln!(writer, "        const {} = {} as u32;", formatted, variant).unwrap();
    }

    writeln!(writer, "    }}").unwrap();
    writeln!(writer, "}}").unwrap();

    // writeln!(writer, "impl {} {{", enum_name).unwrap();
    // writeln!(
    //     writer,
    //     "    pub const fn from_raw(value: u32) -> Self {{ Self::from_bits_truncate(value) }}"
    // )
    writeln!(
        writer,
        "assert_eq_size!({}, {});",
        enum_name, enum_config.name
    )
    .unwrap();
    writeln!(writer).unwrap();
}

#[derive(Debug)]
struct FormatCallback {
    enum_map: Arc<Mutex<EnumMap>>,
    cargo_callbacks: bindgen::CargoCallbacks,
}

fn push_enum_variant(vec: &mut EnumVec, variant: &str, value: EnumVariantValue) {
    if vec.iter().map(|e| e.1).any(|e| e == value) {
        // cargo_warning!("Duplicate value found for variant: {}", variant);
        return;
    }

    vec.push((variant.to_string(), value));
}

impl bindgen::callbacks::ParseCallbacks for FormatCallback {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        variant_value: EnumVariantValue,
    ) -> Option<String> {
        // macro_rules! push_to_vec {
        let trimmed_enum_name = match &enum_name {
            Some(name) => name.trim_start_matches("enum "),
            None => {
                cargo_warning!("No enum name found for variant: {}", original_variant_name);
                return None;
            }
        };

        if !trimmed_enum_name.starts_with("Vk") {
            return None;
        }

        let mut map = self.enum_map.lock().unwrap();
        let vec = map
            .entry(trimmed_enum_name.to_string())
            .or_insert_with(Vec::new);
        push_enum_variant(vec, original_variant_name, variant_value);

        None
    }

    fn header_file(&self, filename: &str) {
        self.cargo_callbacks.header_file(filename);
    }

    fn include_file(&self, filename: &str) {
        self.cargo_callbacks.include_file(filename);
    }
}

#[derive(Debug, Clone)]
struct EnumConfig<'a> {
    name: &'a str,
    custom_name: Option<&'a str>,
    prefix: &'a str,
    is_flags: bool,
}

impl Default for EnumConfig<'_> {
    fn default() -> Self {
        Self {
            name: "",
            custom_name: None,
            prefix: "",
            is_flags: false,
        }
    }
}

fn build_config_map() -> HashMap<&'static str, EnumConfig<'static>> {
    let configs: &[EnumConfig<'static>] = &[
        EnumConfig {
            name: "VkFormat",
            prefix: "VK_FORMAT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageLayout",
            prefix: "VK_IMAGE_LAYOUT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkObjectType",
            prefix: "VK_OBJECT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageTiling",
            prefix: "VK_IMAGE_TILING_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageType",
            prefix: "VK_IMAGE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPhysicalDeviceType",
            prefix: "VK_PHYSICAL_DEVICE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryType",
            prefix: "VK_QUERY_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSharingMode",
            prefix: "VK_SHARING_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkComponentSwizzle",
            prefix: "VK_COMPONENT_SWIZZLE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageViewType",
            prefix: "VK_IMAGE_VIEW_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkBlendFactor",
            prefix: "VK_BLEND_FACTOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkBlendOp",
            prefix: "VK_BLEND_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkCompareOp",
            prefix: "VK_COMPARE_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDynamicState",
            prefix: "VK_DYNAMIC_STATE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkFrontFace",
            prefix: "VK_FRONT_FACE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVertexInputRate",
            prefix: "VK_VERTEX_INPUT_RATE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPrimitiveTopology",
            prefix: "VK_PRIMITIVE_TOPOLOGY_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPolygonMode",
            prefix: "VK_POLYGON_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkStencilOp",
            prefix: "VK_STENCIL_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkLogicOp",
            prefix: "VK_LOGIC_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkBorderColor",
            prefix: "VK_BORDER_COLOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkFilter",
            prefix: "VK_FILTER_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkColorSpaceKHR",
            prefix: "VK_COLOR_SPACE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPresentModeKHR",
            prefix: "VK_PRESENT_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageUsageFlagBits",
            prefix: "VK_IMAGE_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSurfaceTransformFlagBitsKHR",
            prefix: "VK_SURFACE_TRANSFORM_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceQueueCreateFlagBits",
            prefix: "VK_DEVICE_QUEUE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkMemoryPropertyFlagBits",
            prefix: "VK_MEMORY_PROPERTY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkMemoryHeapFlagBits",
            prefix: "VK_MEMORY_HEAP_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkMemoryAllocateFlagBits",
            prefix: "VK_MEMORY_ALLOCATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineStageFlagBits",
            prefix: "VK_PIPELINE_STAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccessFlagBits",
            prefix: "VK_ACCESS_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDependencyFlagBits",
            prefix: "VK_DEPENDENCY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCommandPoolCreateFlagBits",
            prefix: "VK_COMMAND_POOL_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCommandPoolResetFlagBits",
            prefix: "VK_COMMAND_POOL_RESET_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCommandBufferUsageFlagBits",
            prefix: "VK_COMMAND_BUFFER_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCommandBufferResetFlagBits",
            prefix: "VK_COMMAND_BUFFER_RESET_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorType",
            prefix: "VK_DESCRIPTOR_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorPoolCreateFlagBits",
            prefix: "VK_DESCRIPTOR_POOL_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorPoolResetFlagBits",
            prefix: "VK_DESCRIPTOR_POOL_RESET_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorSetLayoutCreateFlagBits",
            prefix: "VK_DESCRIPTOR_SET_LAYOUT_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAttachmentDescriptionFlagBits",
            prefix: "VK_ATTACHMENT_DESCRIPTION_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAttachmentLoadOp",
            prefix: "VK_ATTACHMENT_LOAD_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkAttachmentStoreOp",
            prefix: "VK_ATTACHMENT_STORE_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineBindPoint",
            prefix: "VK_PIPELINE_BIND_POINT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineShaderStageCreateFlagBits",
            prefix: "VK_PIPELINE_SHADER_STAGE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineInputAssemblyStateCreateFlagBits",
            prefix: "VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineViewportStateCreateFlagBits",
            prefix: "VK_PIPELINE_VIEWPORT_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineMultisampleStateCreateFlagBits",
            prefix: "VK_PIPELINE_MULTISAMPLE_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineDepthStencilStateCreateFlagBits",
            prefix: "VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineColorBlendStateCreateFlagBits",
            prefix: "VK_PIPELINE_COLOR_BLEND_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineDynamicStateCreateFlagBits",
            prefix: "VK_PIPELINE_DYNAMIC_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineLayoutCreateFlagBits",
            prefix: "VK_PIPELINE_LAYOUT_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerCreateFlagBits",
            prefix: "VK_SAMPLER_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFramebufferCreateFlagBits",
            prefix: "VK_FRAMEBUFFER_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkRenderPassCreateFlagBits",
            prefix: "VK_RENDER_PASS_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCommandBufferLevel",
            prefix: "VK_COMMAND_BUFFER_LEVEL_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSubpassContents",
            prefix: "VK_SUBPASS_CONTENTS_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkInstanceCreateFlagBits",
            prefix: "VK_INSTANCE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceCreateFlagBits",
            prefix: "VK_DEVICE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryControlFlagBits",
            prefix: "VK_QUERY_CONTROL_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryResultFlagBits",
            prefix: "VK_QUERY_RESULT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBufferCreateFlagBits",
            prefix: "VK_BUFFER_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBufferUsageFlagBits",
            prefix: "VK_BUFFER_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBufferViewCreateFlagBits",
            prefix: "VK_BUFFER_VIEW_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageCreateFlagBits",
            prefix: "VK_IMAGE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageViewCreateFlagBits",
            prefix: "VK_IMAGE_VIEW_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderModuleCreateFlagBits",
            prefix: "VK_SHADER_MODULE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineCacheCreateFlagBits",
            prefix: "VK_PIPELINE_CACHE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineCreateFlagBits",
            prefix: "VK_PIPELINE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineVertexInputStateCreateFlagBits",
            prefix: "VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineTessellationStateCreateFlagBits",
            prefix: "VK_PIPELINE_TESSELLATION_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineRasterizationStateCreateFlagBits",
            prefix: "VK_PIPELINE_RASTERIZATION_STATE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSparseMemoryBindFlagBits",
            prefix: "VK_SPARSE_MEMORY_BIND_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalSemaphoreHandleTypeFlagBits",
            prefix: "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryPipelineStatisticFlagBits",
            prefix: "VK_QUERY_PIPELINE_STATISTIC_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSemaphoreType",
            prefix: "VK_SEMAPHORE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSubpassMergeStatusEXT",
            prefix: "VK_SUBPASS_MERGE_STATUS_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkFragmentShadingRateNV",
            prefix: "VK_FRAGMENT_SHADING_RATE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalSemaphoreFeatureFlagBits",
            prefix: "VK_EXTERNAL_SEMAPHORE_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorBindingFlagBits",
            prefix: "VK_DESCRIPTOR_BINDING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkValidationCheckEXT",
            prefix: "VK_VALIDATION_CHECK_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoDecodeCapabilityFlagBitsKHR",
            prefix: "VK_VIDEO_DECODE_CAPABILITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkValidationFeatureDisableEXT",
            prefix: "VK_VALIDATION_FEATURE_DISABLE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBuildMicromapModeEXT",
            prefix: "VK_BUILD_MICROMAP_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalFenceHandleTypeFlagBits",
            prefix: "VK_EXTERNAL_FENCE_HANDLE_TYPE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderCorePropertiesFlagBitsAMD",
            prefix: "VK_SHADER_CORE_PROPERTIES_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerYcbcrRange",
            prefix: "VK_SAMPLER_YCBCR_RANGE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDisplayPowerStateEXT",
            prefix: "VK_DISPLAY_POWER_STATE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkLatencyMarkerNV",
            prefix: "VK_LATENCY_MARKER_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkRasterizationOrderAMD",
            prefix: "VK_RASTERIZATION_ORDER_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkRenderingFlagBits",
            prefix: "VK_RENDERING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceDiagnosticsConfigFlagBitsNV",
            prefix: "VK_DEVICE_DIAGNOSTICS_CONFIG_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureMotionInstanceTypeNV",
            prefix: "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkGeometryFlagBitsKHR",
            prefix: "VK_GEOMETRY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowGridSizeFlagBitsNV",
            prefix: "VK_OPTICAL_FLOW_GRID_SIZE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderInfoTypeAMD",
            prefix: "VK_SHADER_INFO_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDisplayEventTypeEXT",
            prefix: "VK_DISPLAY_EVENT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPeerMemoryFeatureFlagBits",
            prefix: "VK_PEER_MEMORY_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderGroupShaderKHR",
            prefix: "VK_SHADER_GROUP_SHADER_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkCoverageReductionModeNV",
            prefix: "VK_COVERAGE_REDUCTION_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceCounterUnitKHR",
            prefix: "VK_PERFORMANCE_COUNTER_UNIT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceCounterStorageKHR",
            prefix: "VK_PERFORMANCE_COUNTER_STORAGE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH265RateControlFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H265_RATE_CONTROL_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDisplayPlaneAlphaFlagBitsKHR",
            prefix: "VK_DISPLAY_PLANE_ALPHA_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryResultStatusKHR",
            prefix: "VK_QUERY_RESULT_STATUS_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH265TransformBlockSizeFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceEventTypeEXT",
            prefix: "VK_DEVICE_EVENT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH265CapabilityFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H265_CAPABILITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalMemoryFeatureFlagBitsNV",
            prefix: "VK_EXTERNAL_MEMORY_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceFaultAddressTypeEXT",
            prefix: "VK_DEVICE_FAULT_ADDRESS_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpacityMicromapSpecialIndexEXT",
            prefix: "VK_OPACITY_MICROMAP_SPECIAL_INDEX_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkLineRasterizationModeEXT",
            prefix: "VK_LINE_RASTERIZATION_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeFeedbackFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_FEEDBACK_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPresentScalingFlagBitsEXT",
            prefix: "VK_PRESENT_SCALING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceFaultVendorBinaryHeaderVersionEXT",
            prefix: "VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSampleCountFlagBits",
            prefix: "VK_SAMPLE_COUNT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkGeometryTypeKHR",
            prefix: "VK_GEOMETRY_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDebugReportObjectTypeEXT",
            prefix: "VK_DEBUG_REPORT_OBJECT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkComponentTypeKHR",
            prefix: "VK_COMPONENT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkCubicFilterWeightsQCOM",
            prefix: "VK_CUBIC_FILTER_WEIGHTS_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderStageFlagBits",
            prefix: "VK_SHADER_STAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceAddressBindingTypeEXT",
            prefix: "VK_DEVICE_ADDRESS_BINDING_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowUsageFlagBitsNV",
            prefix: "VK_OPTICAL_FLOW_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFenceImportFlagBits",
            prefix: "VK_FENCE_IMPORT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpacityMicromapFormatEXT",
            prefix: "VK_OPACITY_MICROMAP_FORMAT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoSessionCreateFlagBitsKHR",
            prefix: "VK_VIDEO_SESSION_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageCompressionFixedRateFlagBitsEXT",
            prefix: "VK_IMAGE_COMPRESSION_FIXED_RATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFenceCreateFlagBits",
            prefix: "VK_FENCE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureMemoryRequirementsTypeNV",
            prefix: "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerAddressMode",
            prefix: "VK_SAMPLER_ADDRESS_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerMipmapMode",
            prefix: "VK_SAMPLER_MIPMAP_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkConservativeRasterizationModeEXT",
            prefix: "VK_CONSERVATIVE_RASTERIZATION_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkGeometryInstanceFlagBitsKHR",
            prefix: "VK_GEOMETRY_INSTANCE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSubmitFlagBits",
            prefix: "VK_SUBMIT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkIndirectCommandsTokenTypeNV",
            prefix: "VK_INDIRECT_COMMANDS_TOKEN_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkFullScreenExclusiveEXT",
            prefix: "VK_FULL_SCREEN_EXCLUSIVE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoDecodeH264PictureLayoutFlagBitsKHR",
            prefix: "VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkLayeredDriverUnderlyingApiMSFT",
            prefix: "VK_LAYERED_DRIVER_UNDERLYING_API_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDebugUtilsMessageTypeFlagBitsEXT",
            prefix: "VK_DEBUG_UTILS_MESSAGE_TYPE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowPerformanceLevelNV",
            prefix: "VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkStencilFaceFlagBits",
            prefix: "VK_STENCIL_FACE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowExecuteFlagBitsNV",
            prefix: "VK_OPTICAL_FLOW_EXECUTE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSurfaceCounterFlagBitsEXT",
            prefix: "VK_SURFACE_COUNTER_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceCounterScopeKHR",
            prefix: "VK_PERFORMANCE_COUNTER_SCOPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkCopyAccelerationStructureModeKHR",
            prefix: "VK_COPY_ACCELERATION_STRUCTURE_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoCodecOperationFlagBitsKHR",
            prefix: "VK_VIDEO_CODEC_OPERATION_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDriverId",
            prefix: "VK_DRIVER_ID_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDebugUtilsMessageSeverityFlagBitsEXT",
            prefix: "VK_DEBUG_UTILS_MESSAGE_SEVERITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSemaphoreImportFlagBits",
            prefix: "VK_SEMAPHORE_IMPORT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineExecutableStatisticFormatKHR",
            prefix: "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoCodingControlFlagBitsKHR",
            prefix: "VK_VIDEO_CODING_CONTROL_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkRayTracingShaderGroupTypeKHR",
            prefix: "VK_RAY_TRACING_SHADER_GROUP_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalMemoryHandleTypeFlagBitsNV",
            prefix: "VK_EXTERNAL_MEMORY_HANDLE_TYPE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkTessellationDomainOrigin",
            prefix: "VK_TESSELLATION_DOMAIN_ORIGIN_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDirectDriverLoadingModeLUNARG",
            prefix: "VK_DIRECT_DRIVER_LOADING_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkChromaLocation",
            prefix: "VK_CHROMA_LOCATION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineCreationFeedbackFlagBits",
            prefix: "VK_PIPELINE_CREATION_FEEDBACK_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeRateControlModeFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkIndexType",
            prefix: "VK_INDEX_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeCapabilityFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_CAPABILITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoComponentBitDepthFlagBitsKHR",
            prefix: "VK_VIDEO_COMPONENT_BIT_DEPTH_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoDecodeUsageFlagBitsKHR",
            prefix: "VK_VIDEO_DECODE_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkResolveModeFlagBits",
            prefix: "VK_RESOLVE_MODE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkRayTracingInvocationReorderModeNV",
            prefix: "VK_RAY_TRACING_INVOCATION_REORDER_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPresentGravityFlagBitsEXT",
            prefix: "VK_PRESENT_GRAVITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowSessionCreateFlagBitsNV",
            prefix: "VK_OPTICAL_FLOW_SESSION_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSystemAllocationScope",
            prefix: "VK_SYSTEM_ALLOCATION_SCOPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoChromaSubsamplingFlagBitsKHR",
            prefix: "VK_VIDEO_CHROMA_SUBSAMPLING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalMemoryFeatureFlagBits",
            prefix: "VK_EXTERNAL_MEMORY_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDiscardRectangleModeEXT",
            prefix: "VK_DISCARD_RECTANGLE_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceConfigurationTypeINTEL",
            prefix: "VK_PERFORMANCE_CONFIGURATION_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkBuildMicromapFlagBitsEXT",
            prefix: "VK_BUILD_MICROMAP_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeUsageFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFrameBoundaryFlagBitsEXT",
            prefix: "VK_FRAME_BOUNDARY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPointClippingBehavior",
            prefix: "VK_POINT_CLIPPING_BEHAVIOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceAddressBindingFlagBitsEXT",
            prefix: "VK_DEVICE_ADDRESS_BINDING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderFloatControlsIndependence",
            prefix: "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkOpticalFlowSessionBindingPointNV",
            prefix: "VK_OPTICAL_FLOW_SESSION_BINDING_POINT_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueueFlagBits",
            prefix: "VK_QUEUE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkIndirectStateFlagBitsNV",
            prefix: "VK_INDIRECT_STATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureCompatibilityKHR",
            prefix: "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkLayerSettingTypeEXT",
            prefix: "VK_LAYER_SETTING_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceParameterTypeINTEL",
            prefix: "VK_PERFORMANCE_PARAMETER_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkGraphicsPipelineLibraryFlagBitsEXT",
            prefix: "VK_GRAPHICS_PIPELINE_LIBRARY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCullModeFlagBits",
            prefix: "VK_CULL_MODE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFragmentShadingRateTypeNV",
            prefix: "VK_FRAGMENT_SHADING_RATE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkViewportCoordinateSwizzleNV",
            prefix: "VK_VIEWPORT_COORDINATE_SWIZZLE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkOutOfBandQueueTypeNV",
            prefix: "VK_OUT_OF_BAND_QUEUE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkTimeDomainKHR",
            prefix: "VK_TIME_DOMAIN_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeTuningModeKHR",
            prefix: "VK_VIDEO_ENCODE_TUNING_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDebugReportFlagBitsEXT",
            prefix: "VK_DEBUG_REPORT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkIndirectCommandsLayoutUsageFlagBitsNV",
            prefix: "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoCapabilityFlagBitsKHR",
            prefix: "VK_VIDEO_CAPABILITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFragmentShadingRateCombinerOpKHR",
            prefix: "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerYcbcrModelConversion",
            prefix: "VK_SAMPLER_YCBCR_MODEL_CONVERSION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderCodeTypeEXT",
            prefix: "VK_SHADER_CODE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceMemoryReportEventTypeEXT",
            prefix: "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureCreateFlagBitsKHR",
            prefix: "VK_ACCELERATION_STRUCTURE_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCopyMicromapModeEXT",
            prefix: "VK_COPY_MICROMAP_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceCounterDescriptionFlagBitsKHR",
            prefix: "VK_PERFORMANCE_COUNTER_DESCRIPTION_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkConditionalRenderingFlagBitsEXT",
            prefix: "VK_CONDITIONAL_RENDERING_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH264CapabilityFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H264_CAPABILITY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalMemoryHandleTypeFlagBits",
            prefix: "VK_EXTERNAL_MEMORY_HANDLE_TYPE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCompositeAlphaFlagBitsKHR",
            prefix: "VK_COMPOSITE_ALPHA_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCoarseSampleOrderTypeNV",
            prefix: "VK_COARSE_SAMPLE_ORDER_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkMicromapTypeEXT",
            prefix: "VK_MICROMAP_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineRobustnessImageBehaviorEXT",
            prefix: "VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureBuildTypeKHR",
            prefix: "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkScopeKHR",
            prefix: "VK_SCOPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceOverrideTypeINTEL",
            prefix: "VK_PERFORMANCE_OVERRIDE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkToolPurposeFlagBits",
            prefix: "VK_TOOL_PURPOSE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBlockMatchWindowCompareModeQCOM",
            prefix: "VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkAcquireProfilingLockFlagBitsKHR",
            prefix: "VK_ACQUIRE_PROFILING_LOCK_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueueGlobalPriorityKHR",
            prefix: "VK_QUEUE_GLOBAL_PRIORITY_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkValidationFeatureEnableEXT",
            prefix: "VK_VALIDATION_FEATURE_ENABLE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageAspectFlagBits",
            prefix: "VK_IMAGE_ASPECT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkMemoryOverallocationBehaviorAMD",
            prefix: "VK_MEMORY_OVERALLOCATION_BEHAVIOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVendorId",
            prefix: "VK_VENDOR_ID_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkInternalAllocationType",
            prefix: "VK_INTERNAL_ALLOCATION_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH265StdFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H265_STD_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH265CtbSizeFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H265_CTB_SIZE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkExternalFenceFeatureFlagBits",
            prefix: "VK_EXTERNAL_FENCE_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBuildAccelerationStructureModeKHR",
            prefix: "VK_BUILD_ACCELERATION_STRUCTURE_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH264StdFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H264_STD_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSemaphoreWaitFlagBits",
            prefix: "VK_SEMAPHORE_WAIT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineCompilerControlFlagBitsAMD",
            prefix: "VK_PIPELINE_COMPILER_CONTROL_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkValidationCacheHeaderVersionEXT",
            prefix: "VK_VALIDATION_CACHE_HEADER_VERSION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkShaderCreateFlagBitsEXT",
            prefix: "VK_SHADER_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkCoverageModulationModeNV",
            prefix: "VK_COVERAGE_MODULATION_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSubgroupFeatureFlagBits",
            prefix: "VK_SUBGROUP_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkColorComponentFlagBits",
            prefix: "VK_COLOR_COMPONENT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSparseImageFormatFlagBits",
            prefix: "VK_SPARSE_IMAGE_FORMAT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkProvokingVertexModeEXT",
            prefix: "VK_PROVOKING_VERTEX_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkImageCompressionFlagBitsEXT",
            prefix: "VK_IMAGE_COMPRESSION_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDepthBiasRepresentationEXT",
            prefix: "VK_DEPTH_BIAS_REPRESENTATION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkQueryPoolSamplingModeINTEL",
            prefix: "VK_QUERY_POOL_SAMPLING_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineCacheHeaderVersion",
            prefix: "VK_PIPELINE_CACHE_HEADER_VERSION_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkPerformanceValueTypeINTEL",
            prefix: "VK_PERFORMANCE_VALUE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkShadingRatePaletteEntryNV",
            prefix: "VK_SHADING_RATE_PALETTE_ENTRY_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkBuildAccelerationStructureFlagBitsKHR",
            prefix: "VK_BUILD_ACCELERATION_STRUCTURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSwapchainCreateFlagBitsKHR",
            prefix: "VK_SWAPCHAIN_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkDescriptorUpdateTemplateType",
            prefix: "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeH264RateControlFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_H264_RATE_CONTROL_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkSubpassDescriptionFlagBits",
            prefix: "VK_SUBPASS_DESCRIPTION_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkVideoEncodeContentFlagBitsKHR",
            prefix: "VK_VIDEO_ENCODE_CONTENT_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkAccelerationStructureTypeKHR",
            prefix: "VK_ACCELERATION_STRUCTURE_TYPE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkEventCreateFlagBits",
            prefix: "VK_EVENT_CREATE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkBlendOverlapEXT",
            prefix: "VK_BLEND_OVERLAP_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkHostImageCopyFlagBitsEXT",
            prefix: "VK_HOST_IMAGE_COPY_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkPipelineRobustnessBufferBehaviorEXT",
            prefix: "VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkSamplerReductionMode",
            prefix: "VK_SAMPLER_REDUCTION_MODE_",
            ..Default::default()
        },
        EnumConfig {
            name: "VkDeviceGroupPresentModeFlagBitsKHR",
            prefix: "VK_DEVICE_GROUP_PRESENT_MODE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkFormatFeatureFlagBits",
            prefix: "VK_FORMAT_FEATURE_",
            is_flags: true,
            ..Default::default()
        },
        EnumConfig {
            name: "VkMicromapCreateFlagBitsEXT",
            prefix: "VK_MICROMAP_CREATE_",
            is_flags: true,
            ..Default::default()
        },
    ];

    let mut map = HashMap::new();
    for config in configs {
        map.insert(config.name, config.clone()).inspect(|c| {
            cargo_warning!("Duplicate enum config found: {}", c.name);
        });
    }
    map
}

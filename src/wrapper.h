// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#if defined(_WIN32) || defined(_WIN64)
#define VK_USE_PLATFORM_WIN32_KHR 1
#elif defined(__linux__)
#define VK_USE_PLATFORM_WAYLAND_KHR 1
#elif defined(__APPLE__)
#define VK_USE_PLATFORM_MACOS_MVK 1
#endif

#include <vulkan/vulkan.h>
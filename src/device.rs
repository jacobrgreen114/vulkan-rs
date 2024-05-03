// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;

use vulkan_sys::*;

/*
   Device Queue Create Info
*/

vulkan_create_info_referential!(
    DeviceQueueCreateInfo,
    VkDeviceQueueCreateInfo,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO
);

assert_eq_size!(DeviceQueueCreateInfo, VkDeviceQueueCreateInfo);

impl<'a> DeviceQueueCreateInfo<'a> {
    pub const fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queueFamilyIndex = queue_family_index;
        self
    }

    pub const fn with_queue_priorities(mut self, queue_priorities: &'a [f32]) -> Self {
        self.inner.queueCount = queue_priorities.len() as u32;
        self.inner.pQueuePriorities = queue_priorities.as_ptr();
        self
    }
}

/*
   Device Create Info
*/

vulkan_create_info_referential!(
    DeviceCreateInfo,
    VkDeviceCreateInfo,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
);

impl<'a> DeviceCreateInfo<'a> {
    pub const fn with_queue_create_infos(
        mut self,
        queue_create_infos: &'a [DeviceQueueCreateInfo],
    ) -> Self {
        self.inner.queueCreateInfoCount = queue_create_infos.len() as u32;
        self.inner.pQueueCreateInfos = queue_create_infos.as_ptr().cast();
        self
    }

    pub const fn with_enabled_extensions(
        mut self,
        enabled_extension_names: &'a [*const c_char],
    ) -> Self {
        self.inner.enabledExtensionCount = enabled_extension_names.len() as u32;
        self.inner.ppEnabledExtensionNames = enabled_extension_names.as_ptr();
        self
    }

    pub const fn with_enabled_features(
        mut self,
        enabled_features: &'a PhysicalDeviceFeatures,
    ) -> Self {
        self.inner.pEnabledFeatures = enabled_features.as_raw();
        self
    }
}

vulkan_handle!(Device, VkDevice);

impl Device {
    pub fn create(
        physical_device: PhysicalDevice,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Self> {
        create_device(
            vkCreateDevice,
            physical_device.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(Self::from_raw)
    }

    pub fn destroy(&self, allocator: Option<&AllocationCallbacks>) {
        destroy_device(
            vkDestroyDevice,
            self.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        Queue::from_raw(get_device_queue(
            vkGetDeviceQueue,
            self.as_raw(),
            queue_family_index,
            queue_index,
        ))
    }

    pub fn create_swapchain_khr(
        &self,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<SwapchainKHR> {
        create_swapchain_khr(
            vkCreateSwapchainKHR,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(SwapchainKHR::from_raw)
    }

    pub fn destroy_swapchain_khr(
        &self,
        swapchain: SwapchainKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_swapchain_khr(
            vkDestroySwapchainKHR,
            self.as_raw(),
            swapchain.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn get_swapchain_images_khr(
        &self,
        swapchain: SwapchainKHR,
    ) -> vulkan_sys::wrapper::Result<Vec<Image>> {
        get_swapchain_images_khr(vkGetSwapchainImagesKHR, self.as_raw(), swapchain.as_raw())
            .map(|images| unsafe { transmute(images) })
    }

    pub fn acquire_next_image_khr(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Option<Semaphore>,
        fence: Option<Fence>,
    ) -> vulkan_sys::wrapper::Result<u32> {
        acquire_next_image_khr(
            vkAcquireNextImageKHR,
            self.as_raw(),
            swapchain.as_raw(),
            timeout,
            semaphore.as_ref().map(Semaphore::as_raw),
            fence.as_ref().map(Fence::as_raw),
        )
    }

    pub fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<ImageView> {
        create_image_view(
            vkCreateImageView,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(ImageView::from_raw)
    }

    pub fn destroy_image_view(
        &self,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_image_view(
            vkDestroyImageView,
            self.as_raw(),
            image_view.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<RenderPass> {
        create_render_pass(
            vkCreateRenderPass,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(RenderPass::from_raw)
    }

    pub fn destroy_render_pass(
        &self,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_render_pass(
            vkDestroyRenderPass,
            self.as_raw(),
            render_pass.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Framebuffer> {
        create_framebuffer(
            vkCreateFramebuffer,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(Framebuffer::from_raw)
    }

    pub fn destroy_framebuffer(
        &self,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_framebuffer(
            vkDestroyFramebuffer,
            self.as_raw(),
            framebuffer.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<DescriptorSetLayout> {
        create_descriptor_set_layout(
            vkCreateDescriptorSetLayout,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(DescriptorSetLayout::from_raw)
    }

    pub fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_descriptor_set_layout(
            vkDestroyDescriptorSetLayout,
            self.as_raw(),
            descriptor_set_layout.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<PipelineLayout> {
        create_pipeline_layout(
            vkCreatePipelineLayout,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(PipelineLayout::from_raw)
    }

    pub fn destroy_pipeline_layout(
        &self,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_pipeline_layout(
            vkDestroyPipelineLayout,
            self.as_raw(),
            pipeline_layout.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<ShaderModule> {
        create_shader_module(
            vkCreateShaderModule,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(ShaderModule::from_raw)
    }

    pub fn destroy_shader_module(
        &self,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_shader_module(
            vkDestroyShaderModule,
            self.as_raw(),
            shader_module.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn create_graphics_pipelines(
        &self,
        pipeline_cache: Option<PipelineCache>,
        create_infos: &[GraphicsPipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Vec<Pipeline>> {
        unsafe {
            transmute(create_graphics_pipelines(
                vkCreateGraphicsPipelines,
                self.as_raw(),
                pipeline_cache
                    .map(|c| c.as_raw())
                    .unwrap_or(std::ptr::null_mut()),
                transmute(create_infos),
                transmute(allocator),
            ))
        }
    }

    pub fn destroy_pipeline(&self, pipeline: Pipeline, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            transmute(destroy_pipeline(
                vkDestroyPipeline,
                self.as_raw(),
                pipeline.as_raw(),
                transmute(allocator),
            ))
        }
    }

    pub fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<CommandPool> {
        unsafe {
            transmute(create_command_pool(
                vkCreateCommandPool,
                self.as_raw(),
                create_info.as_raw(),
                transmute(allocator),
            ))
        }
    }

    pub fn destroy_command_pool(
        &self,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        destroy_command_pool(
            vkDestroyCommandPool,
            self.as_raw(),
            command_pool.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
    ) -> vulkan_sys::wrapper::Result<Vec<CommandBuffer>> {
        allocate_command_buffers(
            vkAllocateCommandBuffers,
            self.as_raw(),
            allocate_info.as_raw(),
        )
        .map(|buffers| buffers.into_iter().map(CommandBuffer::from_raw).collect())
    }

    pub fn create_fence(
        &self,
        create_info: &FenceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Fence> {
        create_fence(
            vkCreateFence,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(Fence::from_raw)
    }

    pub fn destroy_fence(&self, fence: Fence, allocator: Option<&AllocationCallbacks>) {
        destroy_fence(
            vkDestroyFence,
            self.as_raw(),
            fence.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }

    pub fn wait_for_fences(
        &self,
        fences: &[Fence],
        wait_all: bool,
        timeout: u64,
    ) -> vulkan_sys::wrapper::Result<()> {
        wait_for_fences(
            vkWaitForFences,
            self.as_raw(),
            unsafe { transmute(fences) },
            wait_all,
            timeout,
        )
    }

    pub fn reset_fences(&self, fences: &[Fence]) -> vulkan_sys::wrapper::Result<()> {
        reset_fences(vkResetFences, self.as_raw(), unsafe { transmute(fences) })
    }

    pub fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> vulkan_sys::wrapper::Result<Semaphore> {
        create_semaphore(
            vkCreateSemaphore,
            self.as_raw(),
            create_info.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
        .map(Semaphore::from_raw)
    }

    pub fn destroy_semaphore(&self, semaphore: Semaphore, allocator: Option<&AllocationCallbacks>) {
        destroy_semaphore(
            vkDestroySemaphore,
            self.as_raw(),
            semaphore.as_raw(),
            allocator.map(AllocationCallbacks::as_raw),
        )
    }
}

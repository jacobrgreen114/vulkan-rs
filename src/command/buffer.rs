// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_create_info!(
    CommandBufferAllocateInfo,
    VkCommandBufferAllocateInfo,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
);

impl crate::CommandBufferAllocateInfo {
    pub const fn with_command_pool(mut self, command_pool: CommandPool) -> Self {
        self.inner.commandPool = command_pool.as_raw();
        self
    }

    pub const fn with_level(mut self, level: CommandBufferLevel) -> Self {
        self.inner.level = level as i32;
        self
    }

    pub const fn with_command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.inner.commandBufferCount = command_buffer_count;
        self
    }
}

vulkan_create_info_lifetime!(
    CommandBufferBeginInfo,
    VkCommandBufferBeginInfo,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
);

impl<'a> crate::CommandBufferBeginInfo<'a> {
    pub const fn with_flags(mut self, flags: CommandBufferUsageFlags) -> Self {
        self.inner.flags = flags.bits();
        self
    }

    //pub const fn with_inheritance_info(mut self, inheritance_info: &'a CommandBufferInheritanceInfo) -> Self {
    //    self.inner.pInheritanceInfo = inheritance_info.as_raw();
    //    self
    //}
}

vulkan_create_info_lifetime!(
    RenderPassBeginInfo,
    VkRenderPassBeginInfo,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO
);

impl<'a> crate::RenderPassBeginInfo<'a> {
    pub const fn with_render_pass(mut self, render_pass: RenderPass) -> Self {
        self.inner.renderPass = render_pass.as_raw();
        self
    }

    pub const fn with_framebuffer(mut self, framebuffer: Framebuffer) -> Self {
        self.inner.framebuffer = framebuffer.as_raw();
        self
    }

    pub const fn with_render_area(mut self, render_area: Rect2D) -> Self {
        self.inner.renderArea = render_area;
        self
    }

    pub const fn with_clear_values(mut self, clear_values: &'a [ClearValue]) -> Self {
        self.inner.clearValueCount = clear_values.len() as u32;
        self.inner.pClearValues = clear_values.as_ptr().cast();
        self
    }
}

vulkan_handle!(CommandBuffer, VkCommandBuffer);

impl CommandBuffer {
    pub fn reset(&self, flags: CommandBufferResetFlags) -> vulkan_sys::wrapper::Result<()> {
        reset_command_buffer(vkResetCommandBuffer, self.as_raw(), flags.bits())
    }

    pub fn begin(
        &self,
        begin_info: &crate::CommandBufferBeginInfo,
    ) -> vulkan_sys::wrapper::Result<()> {
        begin_command_buffer(vkBeginCommandBuffer, self.as_raw(), begin_info.as_raw())
    }

    pub fn end(&self) -> vulkan_sys::wrapper::Result<()> {
        end_command_buffer(vkEndCommandBuffer, self.as_raw())
    }

    pub fn cmd_begin_render_pass(
        &self,
        render_pass_begin_info: &crate::RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        cmd_begin_render_pass(
            vkCmdBeginRenderPass,
            self.as_raw(),
            render_pass_begin_info.as_raw(),
            contents.as_raw(),
        )
    }

    pub fn cmd_next_subpass(&self, contents: SubpassContents) {
        cmd_next_subpass(vkCmdNextSubpass, self.as_raw(), contents.as_raw())
    }

    pub fn cmd_end_render_pass(&self) {
        cmd_end_render_pass(vkCmdEndRenderPass, self.as_raw())
    }

    pub fn cmd_begin_query(&self, query_pool: QueryPool, query: u32, flags: QueryControlFlags) {
        cmd_begin_query(
            vkCmdBeginQuery,
            self.as_raw(),
            query_pool.as_raw(),
            query,
            flags.bits(),
        )
    }

    pub fn cmd_bind_descriptor_sets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            cmd_bind_descriptor_sets(
                vkCmdBindDescriptorSets,
                self.as_raw(),
                pipeline_bind_point.as_raw(),
                layout.as_raw(),
                first_set,
                transmute(descriptor_sets),
                transmute(dynamic_offsets),
            )
        }
    }

    pub fn cmd_bind_vertex_buffers(
        &self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            cmd_bind_vertex_buffers(
                vkCmdBindVertexBuffers,
                self.as_raw(),
                first_binding,
                transmute(buffers),
                transmute(offsets),
            )
        }
    }

    pub fn cmd_bind_index_buffer(&self, buffer: Buffer, offset: DeviceSize, index_type: IndexType) {
        cmd_bind_index_buffer(
            vkCmdBindIndexBuffer,
            self.as_raw(),
            buffer.as_raw(),
            offset,
            index_type.as_raw(),
        )
    }

    pub fn cmd_bind_pipeline(&self, pipeline_bind_point: PipelineBindPoint, pipeline: Pipeline) {
        cmd_bind_pipeline(
            vkCmdBindPipeline,
            self.as_raw(),
            pipeline_bind_point.as_raw(),
            pipeline.as_raw(),
        )
    }

    pub fn cmd_blit_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        unsafe {
            cmd_blit_image(
                vkCmdBlitImage,
                self.as_raw(),
                src_image.as_raw(),
                src_image_layout.as_raw(),
                dst_image.as_raw(),
                dst_image_layout.as_raw(),
                transmute(regions),
                filter.as_raw(),
            )
        }
    }

    pub fn cmd_clear_attachments(&self, attachments: &[ClearAttachment], rects: &[ClearRect]) {
        unsafe {
            cmd_clear_attachments(
                vkCmdClearAttachments,
                self.as_raw(),
                transmute(attachments),
                transmute(rects),
            )
        }
    }

    pub fn cmd_clear_color_image(
        &self,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            cmd_clear_color_image(
                vkCmdClearColorImage,
                self.as_raw(),
                image.as_raw(),
                image_layout.as_raw(),
                color,
                transmute(ranges),
            )
        }
    }

    pub fn cmd_clear_depth_stencil_image(
        &self,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            cmd_clear_depth_stencil_image(
                vkCmdClearDepthStencilImage,
                self.as_raw(),
                image.as_raw(),
                image_layout.as_raw(),
                depth_stencil,
                transmute(ranges),
            )
        }
    }

    pub fn cmd_copy_buffer(&self, src_buffer: Buffer, dst_buffer: Buffer, regions: &[BufferCopy]) {
        unsafe {
            cmd_copy_buffer(
                vkCmdCopyBuffer,
                self.as_raw(),
                src_buffer.as_raw(),
                dst_buffer.as_raw(),
                transmute(regions),
            )
        }
    }

    pub fn cmd_copy_buffer_to_image(
        &self,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            cmd_copy_buffer_to_image(
                vkCmdCopyBufferToImage,
                self.as_raw(),
                src_buffer.as_raw(),
                dst_image.as_raw(),
                dst_image_layout.as_raw(),
                transmute(regions),
            )
        }
    }

    pub fn cmd_copy_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        unsafe {
            cmd_copy_image(
                vkCmdCopyImage,
                self.as_raw(),
                src_image.as_raw(),
                src_image_layout.as_raw(),
                dst_image.as_raw(),
                dst_image_layout.as_raw(),
                transmute(regions),
            )
        }
    }

    pub fn cmd_copy_image_to_buffer(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            cmd_copy_image_to_buffer(
                vkCmdCopyImageToBuffer,
                self.as_raw(),
                src_image.as_raw(),
                src_image_layout.as_raw(),
                dst_buffer.as_raw(),
                transmute(regions),
            )
        }
    }

    pub fn cmd_dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        cmd_dispatch(
            vkCmdDispatch,
            self.as_raw(),
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }

    pub fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        cmd_dispatch_indirect(
            vkCmdDispatchIndirect,
            command_buffer.as_raw(),
            buffer.as_raw(),
            offset,
        )
    }

    pub fn cmd_draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        cmd_draw(
            vkCmdDraw,
            self.as_raw(),
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }

    pub fn cmd_draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        cmd_draw_indexed(
            vkCmdDrawIndexed,
            self.as_raw(),
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }

    pub fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        cmd_draw_indexed_indirect(
            vkCmdDrawIndexedIndirect,
            command_buffer.as_raw(),
            buffer.as_raw(),
            offset,
            draw_count,
            stride,
        )
    }

    pub fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        cmd_draw_indirect(
            vkCmdDrawIndirect,
            command_buffer.as_raw(),
            buffer.as_raw(),
            offset,
            draw_count,
            stride,
        )
    }

    // end_query

    pub fn cmd_execute_commands(&self, command_buffers: &[CommandBuffer]) {
        unsafe {
            cmd_execute_commands(
                vkCmdExecuteCommands,
                self.as_raw(),
                transmute(command_buffers),
            )
        }
    }

    pub fn cmd_fill_buffer(&self, buffer: Buffer, offset: DeviceSize, size: DeviceSize, data: u32) {
        cmd_fill_buffer(
            vkCmdFillBuffer,
            self.as_raw(),
            buffer.as_raw(),
            offset,
            size,
            data,
        )
    }

    pub fn cmd_pipeline_barrier(
        &self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            cmd_pipeline_barrier(
                vkCmdPipelineBarrier,
                self.as_raw(),
                src_stage_mask.bits(),
                dst_stage_mask.bits(),
                dependency_flags.bits(),
                transmute(memory_barriers),
                transmute(buffer_memory_barriers),
                transmute(image_memory_barriers),
            )
        }
    }

    pub fn cmd_push_constants(
        &self,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        values: &[u8],
    ) {
        unsafe {
            cmd_push_constants(
                vkCmdPushConstants,
                self.as_raw(),
                layout.as_raw(),
                stage_flags.bits(),
                offset,
                size,
                values.as_ptr().cast(),
            )
        }
    }

    pub fn cmd_reset_event(&self, event: Event, stage_mask: PipelineStageFlags) {
        cmd_reset_event(
            vkCmdResetEvent,
            self.as_raw(),
            event.as_raw(),
            stage_mask.bits(),
        )
    }

    pub fn cmd_reset_query_pool(&self, query_pool: QueryPool, first_query: u32, query_count: u32) {
        cmd_reset_query_pool(
            vkCmdResetQueryPool,
            self.as_raw(),
            query_pool.as_raw(),
            first_query,
            query_count,
        )
    }

    pub fn cmd_resolve_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        unsafe {
            cmd_resolve_image(
                vkCmdResolveImage,
                self.as_raw(),
                src_image.as_raw(),
                src_image_layout.as_raw(),
                dst_image.as_raw(),
                dst_image_layout.as_raw(),
                transmute(regions),
            )
        }
    }

    pub fn cmd_set_blend_constants(&self, blend_constants: [f32; 4]) {
        cmd_set_blend_constants(vkCmdSetBlendConstants, self.as_raw(), blend_constants)
    }

    pub fn cmd_set_depth_bias(&self, constant_factor: f32, clamp: f32, slope_factor: f32) {
        cmd_set_depth_bias(
            vkCmdSetDepthBias,
            self.as_raw(),
            constant_factor,
            clamp,
            slope_factor,
        )
    }

    pub fn cmd_set_depth_bounds(&self, min: f32, max: f32) {
        cmd_set_depth_bounds(vkCmdSetDepthBounds, self.as_raw(), min, max)
    }

    pub fn cmd_set_event(&self, event: Event, stage_mask: PipelineStageFlags) {
        cmd_set_event(
            vkCmdSetEvent,
            self.as_raw(),
            event.as_raw(),
            stage_mask.bits(),
        )
    }

    pub fn cmd_set_line_width(&self, line_width: f32) {
        cmd_set_line_width(vkCmdSetLineWidth, self.as_raw(), line_width)
    }

    pub fn cmd_set_scissor(&self, first_scissor: u32, scissors: &[Rect2D]) {
        unsafe {
            cmd_set_scissor(
                vkCmdSetScissor,
                self.as_raw(),
                first_scissor,
                transmute(scissors),
            )
        }
    }

    pub fn cmd_set_stencil_compare_mask(&self, face_mask: StencilFaceFlags, compare_mask: u32) {
        cmd_set_stencil_compare_mask(
            vkCmdSetStencilCompareMask,
            self.as_raw(),
            face_mask.bits(),
            compare_mask,
        )
    }

    pub fn cmd_set_stencil_reference(&self, face_mask: StencilFaceFlags, reference: u32) {
        cmd_set_stencil_reference(
            vkCmdSetStencilReference,
            self.as_raw(),
            face_mask.bits(),
            reference,
        )
    }

    pub fn cmd_set_stencil_write_mask(&self, face_mask: StencilFaceFlags, write_mask: u32) {
        cmd_set_stencil_write_mask(
            vkCmdSetStencilWriteMask,
            self.as_raw(),
            face_mask.bits(),
            write_mask,
        )
    }

    pub fn cmd_set_viewport(&self, first_viewport: u32, viewports: &[Viewport]) {
        unsafe {
            cmd_set_viewport(
                vkCmdSetViewport,
                self.as_raw(),
                first_viewport,
                transmute(viewports),
            )
        }
    }

    pub fn cmd_update_buffer(&self, buffer: Buffer, offset: DeviceSize, data: &[u8]) {
        cmd_update_buffer(
            vkCmdUpdateBuffer,
            self.as_raw(),
            buffer.as_raw(),
            offset,
            data.len() as DeviceSize,
            data.as_ptr().cast(),
        )
    }

    pub fn cmd_wait_events(
        &self,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            cmd_wait_events(
                vkCmdWaitEvents,
                self.as_raw(),
                transmute(events),
                src_stage_mask.bits(),
                dst_stage_mask.bits(),
                transmute(memory_barriers),
                transmute(buffer_memory_barriers),
                transmute(image_memory_barriers),
            )
        }
    }

    pub fn cmd_write_timestamp(
        &self,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        cmd_write_timestamp(
            vkCmdWriteTimestamp,
            self.as_raw(),
            pipeline_stage.bits() as _,
            query_pool.as_raw(),
            query,
        )
    }
}

vulkan_create_info_lifetime!(
    MemoryBarrier,
    VkMemoryBarrier,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER
);

impl<'a> MemoryBarrier<'a> {
    pub const fn with_src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
        self.inner.srcAccessMask = src_access_mask.bits();
        self
    }

    pub const fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
        self.inner.dstAccessMask = dst_access_mask.bits();
        self
    }
}

vulkan_create_info_lifetime!(
    BufferMemoryBarrier,
    VkBufferMemoryBarrier,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER
);

impl<'a> BufferMemoryBarrier<'a> {
    pub const fn with_src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
        self.inner.srcAccessMask = src_access_mask.bits();
        self
    }

    pub const fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
        self.inner.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub const fn with_src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.srcQueueFamilyIndex = src_queue_family_index;
        self
    }

    pub const fn with_dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dstQueueFamilyIndex = dst_queue_family_index;
        self
    }

    pub const fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.inner.buffer = buffer.as_raw();
        self
    }

    pub const fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }

    pub const fn with_size(mut self, size: DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}

vulkan_create_info_lifetime!(
    ImageMemoryBarrier,
    VkImageMemoryBarrier,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER
);

impl<'a> ImageMemoryBarrier<'a> {
    pub const fn with_src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
        self.inner.srcAccessMask = src_access_mask.bits();
        self
    }

    pub const fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
        self.inner.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub const fn with_old_layout(mut self, old_layout: ImageLayout) -> Self {
        self.inner.oldLayout = old_layout as i32;
        self
    }

    pub const fn with_new_layout(mut self, new_layout: ImageLayout) -> Self {
        self.inner.newLayout = new_layout as i32;
        self
    }

    pub const fn with_src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.srcQueueFamilyIndex = src_queue_family_index;
        self
    }

    pub const fn with_dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dstQueueFamilyIndex = dst_queue_family_index;
        self
    }

    pub const fn with_image(mut self, image: Image) -> Self {
        self.inner.image = image.as_raw();
        self
    }

    pub const fn with_subresource_range(
        mut self,
        subresource_range: ImageSubresourceRange,
    ) -> Self {
        self.inner.subresourceRange = unsafe { transmute(subresource_range) };
        self
    }
}

vulkan_struct!(ImageBlit, VkImageBlit);
vulkan_struct!(ClearAttachment, VkClearAttachment);
vulkan_struct!(ClearRect, VkClearRect);
vulkan_struct!(BufferCopy, VkBufferCopy);

vulkan_struct!(BufferImageCopy, VkBufferImageCopy);
impl BufferImageCopy {
    pub const fn with_buffer_offset(mut self, buffer_offset: DeviceSize) -> Self {
        self.inner.bufferOffset = buffer_offset;
        self
    }

    pub const fn with_buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.inner.bufferRowLength = buffer_row_length;
        self
    }

    pub const fn with_buffer_image_height(mut self, buffer_image_height: u32) -> Self {
        self.inner.bufferImageHeight = buffer_image_height;
        self
    }

    pub const fn with_image_subresource(
        mut self,
        image_subresource: ImageSubresourceLayers,
    ) -> Self {
        self.inner.imageSubresource = unsafe { transmute(image_subresource) };
        self
    }

    pub const fn with_image_offset(mut self, image_offset: Offset3D) -> Self {
        self.inner.imageOffset = image_offset;
        self
    }

    pub const fn with_image_extent(mut self, image_extent: Extent3D) -> Self {
        self.inner.imageExtent = image_extent;
        self
    }
}

vulkan_struct!(ImageCopy, VkImageCopy);
vulkan_struct!(ImageResolve, VkImageResolve);
vulkan_handle!(Event, VkEvent);
vulkan_handle!(QueryPool, VkQueryPool);

vulkan_struct!(ImageSubresourceLayers, VkImageSubresourceLayers);
impl ImageSubresourceLayers {
    pub const fn new_init(
        aspect_mask: ImageAspectFlags,
        mip_level: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Self {
        Self {
            inner: VkImageSubresourceLayers {
                aspectMask: aspect_mask.bits(),
                mipLevel: mip_level,
                baseArrayLayer: base_array_layer,
                layerCount: layer_count,
            },
        }
    }

    pub const fn with_aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
        self.inner.aspectMask = aspect_mask.bits();
        self
    }

    pub const fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.inner.mipLevel = mip_level;
        self
    }

    pub const fn with_base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.inner.baseArrayLayer = base_array_layer;
        self
    }

    pub const fn with_layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layerCount = layer_count;
        self
    }
}

// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::macros::*;
use crate::*;
use vulkan_sys::*;

vulkan_handle!(RenderPass, VkRenderPass);

vulkan_create_info_lifetime!(
    RenderPassCreateInfo,
    VkRenderPassCreateInfo,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO
);

impl<'a> RenderPassCreateInfo<'a> {
    pub const fn with_attachments(mut self, attachments: &'a [AttachmentDescription]) -> Self {
        self.inner.attachmentCount = attachments.len() as u32;
        self.inner.pAttachments = attachments.as_ptr() as *const _;
        self
    }

    pub const fn with_subpasses(mut self, subpasses: &'a [SubpassDescription]) -> Self {
        self.inner.subpassCount = subpasses.len() as u32;
        self.inner.pSubpasses = subpasses.as_ptr() as *const _;
        self
    }

    pub const fn with_dependencies(mut self, dependencies: &'a [SubpassDependency]) -> Self {
        self.inner.dependencyCount = dependencies.len() as u32;
        self.inner.pDependencies = dependencies.as_ptr() as *const _;
        self
    }
}

vulkan_struct!(AttachmentDescription, VkAttachmentDescription);

impl AttachmentDescription {
    pub const fn with_format(mut self, format: Format) -> Self {
        self.inner.format = format as i32;
        self
    }

    pub const fn with_samples(mut self, samples: SampleCountFlags) -> Self {
        self.inner.samples = samples.bits() as i32;
        self
    }

    pub const fn with_load_op(mut self, load_op: AttachmentLoadOp) -> Self {
        self.inner.loadOp = load_op as i32;
        self
    }

    pub const fn with_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        self.inner.storeOp = store_op as i32;
        self
    }

    pub const fn with_stencil_load_op(mut self, stencil_load_op: AttachmentLoadOp) -> Self {
        self.inner.stencilLoadOp = stencil_load_op as i32;
        self
    }

    pub const fn with_stencil_store_op(mut self, stencil_store_op: AttachmentStoreOp) -> Self {
        self.inner.stencilStoreOp = stencil_store_op as i32;
        self
    }

    pub const fn with_initial_layout(mut self, initial_layout: ImageLayout) -> Self {
        self.inner.initialLayout = initial_layout as i32;
        self
    }

    pub const fn with_final_layout(mut self, final_layout: ImageLayout) -> Self {
        self.inner.finalLayout = final_layout as i32;
        self
    }
}

vulkan_struct!(AttachmentReference, VkAttachmentReference);

impl AttachmentReference {
    pub const fn with_attachment(mut self, attachment: u32) -> Self {
        self.inner.attachment = attachment;
        self
    }

    pub const fn with_layout(mut self, layout: ImageLayout) -> Self {
        self.inner.layout = layout as i32;
        self
    }
}

vulkan_struct_lifetime!(SubpassDescription, VkSubpassDescription);

impl<'a> SubpassDescription<'a> {
    pub const fn with_pipeline_bind_point(
        mut self,
        pipeline_bind_point: PipelineBindPoint,
    ) -> Self {
        self.inner.pipelineBindPoint = pipeline_bind_point as i32;
        self
    }

    pub const fn with_input_attachments(
        mut self,
        input_attachments: &'a [AttachmentReference],
    ) -> Self {
        self.inner.inputAttachmentCount = input_attachments.len() as u32;
        self.inner.pInputAttachments = input_attachments.as_ptr().cast();
        self
    }

    pub const fn with_color_attachments(
        mut self,
        color_attachments: &'a [AttachmentReference],
    ) -> Self {
        self.inner.colorAttachmentCount = color_attachments.len() as u32;
        self.inner.pColorAttachments = color_attachments.as_ptr().cast();
        self
    }

    pub const fn with_resolve_attachments(
        mut self,
        resolve_attachments: &'a [AttachmentReference],
    ) -> Self {
        self.inner.pResolveAttachments = resolve_attachments.as_ptr().cast();
        self
    }

    pub const fn with_depth_stencil_attachment(
        mut self,
        depth_stencil_attachment: &'a AttachmentReference,
    ) -> Self {
        self.inner.pDepthStencilAttachment = depth_stencil_attachment as *const _ as *const _;
        self
    }

    pub const fn with_preserve_attachments(mut self, preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserveAttachmentCount = preserve_attachments.len() as u32;
        self.inner.pPreserveAttachments = preserve_attachments.as_ptr();
        self
    }
}

vulkan_struct!(SubpassDependency, VkSubpassDependency);

impl SubpassDependency {
    pub const fn with_src_subpass(mut self, src_subpass: u32) -> Self {
        self.inner.srcSubpass = src_subpass;
        self
    }

    pub const fn with_dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.inner.dstSubpass = dst_subpass;
        self
    }

    pub const fn with_src_stage_mask(mut self, src_stage_mask: PipelineStageFlags) -> Self {
        self.inner.srcStageMask = src_stage_mask.bits();
        self
    }

    pub const fn with_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags) -> Self {
        self.inner.dstStageMask = dst_stage_mask.bits();
        self
    }

    pub const fn with_src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
        self.inner.srcAccessMask = src_access_mask.bits();
        self
    }

    pub const fn with_dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
        self.inner.dstAccessMask = dst_access_mask.bits();
        self
    }

    pub const fn with_dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
        self.inner.dependencyFlags = dependency_flags.bits();
        self
    }
}

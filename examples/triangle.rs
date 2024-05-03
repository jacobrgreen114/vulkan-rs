// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use std::ffi::{c_char, CStr};
use vulkan::*;

use glfw::*;
use vulkan_sys::vkGetDeviceProcAddr;

const MAX_FRAMES_IN_FLIGHT: usize = 3;

const APP_NAME: &CStr = c"Hello Triangle";

const INSTANCE_LAYERS: &[*const c_char] = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];
const INSTANCE_EXTENSIONS: &[*const c_char] =
    &[c"VK_KHR_surface".as_ptr(), c"VK_KHR_win32_surface".as_ptr()];

static APP_INFO: ApplicationInfo<'static> = ApplicationInfo::new()
    .with_api_version(ApiVersion::VERSION_1_0)
    .with_application_name(APP_NAME);

static INSTANCE_CREATE_INFO: InstanceCreateInfo<'static> = InstanceCreateInfo::new()
    .with_application_info(&APP_INFO)
    .with_enabled_layers(INSTANCE_LAYERS)
    .with_enabled_extensions(INSTANCE_EXTENSIONS);

const DEVICE_EXTENSIONS: &[*const c_char] = &[c"VK_KHR_swapchain".as_ptr()];

const QUEUE_FAMILY_INDEX: u32 = 0;
const QUEUE_PRIORITY: f32 = 1.0;

const QUEUE_CREATE_INFO: DeviceQueueCreateInfo<'static> = DeviceQueueCreateInfo::new()
    .with_queue_family_index(QUEUE_FAMILY_INDEX)
    .with_queue_priorities(&[QUEUE_PRIORITY]);

static DEVICE_CREATE_INFO: DeviceCreateInfo<'static> = DeviceCreateInfo::new()
    .with_queue_create_infos(&[QUEUE_CREATE_INFO])
    .with_enabled_extensions(DEVICE_EXTENSIONS);

// static COMMAND_POOL_CREATE_INFO: CommandPoolCreateInfo = CommandPoolCreateInfo::new()
//     .with_flags(CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
//     .with_queue_family_index(QUEUE_FAMILY_INDEX);

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(800, 600, "Vulkan Triangle", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let instance = Instance::create(&INSTANCE_CREATE_INFO, None).unwrap();

    let physical_devices = instance.enumerate_physical_devices().unwrap();
    let physical_device = physical_devices[0];

    let properties = instance.get_physical_device_properties(physical_device);
    let features = instance.get_physical_device_features(physical_device);

    let surface = {
        let create_info = Win32SurfaceCreateInfoKHR::new().with_hwnd(
            windows::Win32::Foundation::HWND(window.get_win32_window() as isize),
        );
        instance
            .create_win32_surface_khr(&create_info, None)
            .unwrap()
    };

    let device = Device::create(
        physical_device,
        &DEVICE_CREATE_INFO.clone().with_enabled_features(&features),
        None,
    )
    .unwrap();
    let queue = device.get_device_queue(QUEUE_FAMILY_INDEX, 0);

    let swapchain = {
        let create_info = SwapchainCreateInfoKHR::new()
            .with_surface(surface)
            .with_min_image_count(3)
            .with_image_format(Format::B8G8R8A8_UNORM)
            .with_image_color_space(ColorSpaceKHR::SRGB_NONLINEAR_KHR)
            .with_image_extent(Extent2D {
                width: 800,
                height: 600,
            })
            .with_image_array_layers(1)
            .with_image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .with_image_sharing_mode(SharingMode::EXCLUSIVE)
            .with_pre_transform(SurfaceTransformFlagsKHR::IDENTITY_KHR)
            .with_composite_alpha(CompositeAlphaFlagsKHR::OPAQUE_KHR)
            .with_present_mode(PresentModeKHR::MAILBOX_KHR)
            .with_clipped(true);

        device.create_swapchain_khr(&create_info, None).unwrap()
    };

    let images = device.get_swapchain_images_khr(swapchain).unwrap();

    let views = images
        .iter()
        .copied()
        .map(|image| {
            let create_info = ImageViewCreateInfo::new()
                .with_image(image)
                .with_view_type(ImageViewType::_2D)
                .with_format(Format::B8G8R8A8_UNORM)
                .with_components(ComponentMapping::default())
                .with_subresource_range(
                    ImageSubresourceRange::new()
                        .with_aspect_mask(ImageAspectFlags::COLOR)
                        .with_mip_level(0, 1)
                        .with_array_layer(0, 1),
                );

            device.create_image_view(&create_info, None).unwrap()
        })
        .collect::<Vec<_>>();

    let render_pass = {
        let attachments = [AttachmentDescription::new()
            .with_format(Format::B8G8R8A8_UNORM)
            .with_samples(SampleCountFlags::_1)
            .with_load_op(AttachmentLoadOp::CLEAR)
            .with_store_op(AttachmentStoreOp::STORE)
            .with_stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .with_stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .with_initial_layout(ImageLayout::UNDEFINED)
            .with_final_layout(ImageLayout::PRESENT_SRC_KHR)];

        let subpass_references = [AttachmentReference::new()
            .with_attachment(0)
            .with_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpasses = [SubpassDescription::new()
            .with_pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .with_color_attachments(&subpass_references)];

        let create_info = RenderPassCreateInfo::new()
            .with_attachments(&attachments)
            .with_subpasses(&subpasses);

        device.create_render_pass(&create_info, None).unwrap()
    };

    let framebuffers = views
        .iter()
        .copied()
        .map(|view| {
            let views = [view];

            let create_info = FramebufferCreateInfo::new()
                .with_render_pass(render_pass)
                .with_attachments(&views)
                .with_width(800)
                .with_height(600)
                .with_layers(1);

            device.create_framebuffer(&create_info, None).unwrap()
        })
        .collect::<Vec<_>>();

    let command_pool = {
        let create_info = CommandPoolCreateInfo::new()
            .with_flags(
                CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            )
            .with_queue_family_index(QUEUE_FAMILY_INDEX);

        device.create_command_pool(&create_info, None).unwrap()
    };

    let buffers = device
        .allocate_command_buffers(
            &CommandBufferAllocateInfo::new()
                .with_command_pool(command_pool)
                .with_level(CommandBufferLevel::PRIMARY)
                .with_command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32),
        )
        .unwrap();

    let in_flight_fences = (0..MAX_FRAMES_IN_FLIGHT)
        .map(|_| {
            device
                .create_fence(
                    &FenceCreateInfo::new().with_flags(FenceCreateFlags::SIGNALED),
                    None,
                )
                .unwrap()
        })
        .collect::<Vec<_>>();

    let image_available_semaphores = (0..MAX_FRAMES_IN_FLIGHT)
        .map(|_| {
            device
                .create_semaphore(&SemaphoreCreateInfo::new(), None)
                .unwrap()
        })
        .collect::<Vec<_>>();

    let render_finished_semaphores = (0..MAX_FRAMES_IN_FLIGHT)
        .map(|_| {
            device
                .create_semaphore(&SemaphoreCreateInfo::new(), None)
                .unwrap()
        })
        .collect::<Vec<_>>();

    let render_area = Rect2D {
        offset: Offset2D { x: 0, y: 0 },
        extent: Extent2D {
            width: 800,
            height: 600,
        },
    };

    let clear_values = [ClearValue {
        color: ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        },
    }];

    let mut current_frame = 0;
    while !window.should_close() {
        let curr_in_flight = in_flight_fences[current_frame];
        let curr_image_available = image_available_semaphores[current_frame];
        let curr_render_finished = render_finished_semaphores[current_frame];
        let curr_command_buffer = buffers[current_frame];

        device
            .wait_for_fences(&[curr_in_flight], true, u64::MAX)
            .unwrap();
        device.reset_fences(&[curr_in_flight]).unwrap();

        let image_index = device
            .acquire_next_image_khr(swapchain, u64::MAX, Some(curr_image_available), None)
            .unwrap();

        let framebuffer = framebuffers[image_index as usize];

        // recording
        {
            curr_command_buffer
                .reset(CommandBufferResetFlags::empty())
                .unwrap();

            {
                let begin_info = CommandBufferBeginInfo::new()
                    .with_flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
                curr_command_buffer.begin(&begin_info).unwrap();
            }

            {
                let begin_info = RenderPassBeginInfo::new()
                    .with_render_pass(render_pass)
                    .with_framebuffer(framebuffer)
                    .with_render_area(render_area)
                    .with_clear_values(&clear_values);

                curr_command_buffer.cmd_begin_render_pass(&begin_info, SubpassContents::INLINE);
            }

            curr_command_buffer.cmd_end_render_pass();

            curr_command_buffer.end().unwrap();
        }

        // submit and present
        {
            let wait_semaphores = [curr_image_available];
            let wait_stages = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal_semaphores = [curr_render_finished];
            let command_buffers = [curr_command_buffer];

            let submit_info = SubmitInfo::new()
                .with_wait_semaphores(&wait_semaphores)
                .with_wait_dst_stage_mask(&wait_stages)
                .with_command_buffers(&command_buffers)
                .with_signal_semaphores(&signal_semaphores);

            queue.submit(&[submit_info], Some(curr_in_flight)).unwrap();

            {
                let swapchains = [swapchain];
                let image_indices = [image_index];

                let present_info = PresentInfoKHR::new()
                    .with_wait_semaphores(&signal_semaphores)
                    .with_swapchains(&swapchains)
                    .with_image_indices(&image_indices);

                queue.present_khr(&present_info).unwrap();
            }
        }

        current_frame = (current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    device
        .wait_for_fences(&in_flight_fences, true, u64::MAX)
        .unwrap();
    for fence in in_flight_fences {
        device.destroy_fence(fence, None);
    }
    for semaphore in image_available_semaphores {
        device.destroy_semaphore(semaphore, None);
    }
    for semaphore in render_finished_semaphores {
        device.destroy_semaphore(semaphore, None);
    }

    device.destroy_command_pool(command_pool, None);

    for framebuffer in framebuffers {
        device.destroy_framebuffer(framebuffer, None);
    }

    device.destroy_render_pass(render_pass, None);

    for view in views {
        device.destroy_image_view(view, None);
    }

    device.destroy_swapchain_khr(swapchain, None);
    device.destroy(None);
    instance.destroy_surface_khr(surface, None);
    instance.destroy(None);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

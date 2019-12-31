use crate::VulkanObject;
use crate::Renderer;
use crate::PhysicalDevice;

use ash::{
    vk,
    Device,
    version::DeviceV1_0
};

pub struct CommandPool {
    command_pool: vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &Device, physical_device: &PhysicalDevice) -> Self {
        let pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(*physical_device.graphics_index())
            .build();

        let command_pool = unsafe {
            device.create_command_pool(&pool_info, None).unwrap()
        };

        CommandPool {
            command_pool: command_pool
        }
    }
}

impl VulkanObject for CommandPool {
    type Object = vk::CommandPool;

    fn vulkan_object(&self) -> &Self::Object {
        &self.command_pool
    }

    fn cleanup(&self, _renderer: &Renderer) {
        unsafe {
             _renderer.logical_device.vulkan_object().destroy_command_pool(self.command_pool, None);
        }
    }
}
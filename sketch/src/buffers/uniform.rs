use super::Buffer;
use crate::{device::PhysicalDevice, GraphicContext, VulkanObject};

use ash::{vk, Device};
use cgmath::Matrix4;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UniformTestObject {
    pub model: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>,
}

impl UniformTestObject {
    pub fn get_size() -> u64 {
        std::mem::size_of::<UniformTestObject>() as u64
    }
}
pub struct UniformBufferObject {
    buffer: Buffer,
}

impl UniformBufferObject {
    pub fn new(logical_device: &Device, physical_device: &PhysicalDevice) -> UniformBufferObject {
        let buffer_size = UniformTestObject::get_size();
        let buffer = Buffer::new(
            buffer_size,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            logical_device,
            physical_device,
        );

        UniformBufferObject { buffer }
    }

    pub fn update2<A, T: Copy>(&self, context: &GraphicContext, object: &[T]) {
        self.buffer.map_memory::<f32, _>(object, context)
    }
}

impl VulkanObject for UniformBufferObject {
    type Object = vk::Buffer;

    fn vulkan_object(&self) -> &Self::Object {
        &self.buffer.vulkan_object()
    }

    fn cleanup(&self, _context: &GraphicContext) {
        self.buffer.cleanup(_context);
    }
}

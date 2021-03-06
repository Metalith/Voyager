use ash::{version::DeviceV1_0, vk};

use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use super::{device::Device, VulkanObject};

pub struct SyncObjects {
    device: Arc<Device>,
    current_frame: Cell<usize>,
    max_frames: usize,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: RefCell<Vec<vk::Fence>>,
}

impl SyncObjects {
    pub fn new(device: Arc<Device>, max_frames: usize, num_images: usize) -> Self {
        let mut image_available_semaphores: Vec<vk::Semaphore> = Vec::new();
        let mut render_finished_semaphores: Vec<vk::Semaphore> = Vec::new();
        let mut in_flight_fences: Vec<vk::Fence> = Vec::new();
        let images_in_flight: RefCell<Vec<vk::Fence>> = RefCell::new(Vec::new());

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED).build();

        for _ in 0..max_frames {
            unsafe {
                in_flight_fences.push(device.vk().create_fence(&fence_info, None).unwrap());
                image_available_semaphores.push(device.vk().create_semaphore(&semaphore_info, None).unwrap());
                render_finished_semaphores.push(device.vk().create_semaphore(&semaphore_info, None).unwrap());
            }
        }

        images_in_flight.borrow_mut().resize(num_images, vk::Fence::null());

        SyncObjects {
            device,
            current_frame: Cell::new(0),
            max_frames,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            images_in_flight,
        }
    }

    pub fn wait_fence_current(&self) {
        unsafe {
            self.device.vk().wait_for_fences(&[self.get_flight_fence()], true, std::u64::MAX).unwrap();
        }
    }

    pub fn wait_fence_image(&self, image_index: usize) {
        if self.get_image_in_flight(image_index) != vk::Fence::null() {
            unsafe {
                self.device.vk().wait_for_fences(&[self.get_image_in_flight(image_index)], true, std::u64::MAX).unwrap();
            }
        }
        self.set_image_in_flight(image_index, self.get_current_frame());
    }

    pub fn get_render_semaphore(&self) -> &vk::Semaphore {
        &self.render_finished_semaphores[self.get_current_frame()]
    }

    pub fn get_image_semaphore(&self) -> &vk::Semaphore {
        &self.image_available_semaphores[self.get_current_frame()]
    }

    pub fn get_flight_fence(&self) -> vk::Fence {
        self.in_flight_fences[self.get_current_frame()]
    }

    pub fn set_image_in_flight(&self, index: usize, frame: usize) {
        self.images_in_flight.borrow_mut()[index] = self.in_flight_fences[frame];
    }

    pub fn get_image_in_flight(&self, index: usize) -> vk::Fence {
        self.images_in_flight.borrow()[index]
    }

    pub fn get_current_frame(&self) -> usize {
        self.current_frame.get()
    }

    pub fn increment_frame(&self) {
        self.current_frame.set((self.current_frame.get() + 1) % self.max_frames);
    }
}

impl Drop for SyncObjects {
    fn drop(&mut self) {
        trace!("Dropping Sync Objects");
        for i in 0..self.in_flight_fences.len() {
            unsafe {
                self.device.vk().destroy_fence(self.in_flight_fences[i], None);
                self.device.vk().destroy_semaphore(self.image_available_semaphores[i], None);
                self.device.vk().destroy_semaphore(self.render_finished_semaphores[i], None);
            }
        }
    }
}

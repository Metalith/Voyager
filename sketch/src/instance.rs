use crate::DebugMessenger;
use crate::VulkanObject;

use winit::window::Window;
use ash::{
    Entry,
    vk,
    vk_make_version,
    version::{EntryV1_0,InstanceV1_0},
    extensions::{
        khr::{Surface, Win32Surface},
        ext::DebugUtils
    }
};

use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;

pub struct Instance {
    debug_messenger : DebugMessenger,
    instance : ash::Instance,
}

impl Instance {
    pub fn new(entry: &Entry) -> Self {
        let validation_enabled: bool  = if std::env::var("WIND_VK_VALIDATION").is_ok() { std::env::var("WIND_VK_VALIDATION").unwrap().parse().unwrap() } else { false };

        if validation_enabled && !DebugMessenger::check_validation_layer_support(&entry) {
            panic!("Validation layers requested not supported");
        }

        let app_name = CString::new("Hello world").unwrap(); // Generate this somewhere
        let engine_name = CString::new("No engine").unwrap();
        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: vk_make_version!(0, 0, 1),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk_make_version!(0, 0, 1),
            api_version: vk_make_version!(1, 1, 106),
            ..Default::default()
        };

        let (_names, validation_layers)= DebugMessenger::get_validation_layers_vk();

        let mut extensions = Self::required_extension_names();
        if validation_enabled {
            extensions.push(DebugUtils::name().as_ptr());
        }

        let debug_create_info = DebugMessenger::populate_debug_messenger_create_info();

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            p_next: if validation_enabled { &debug_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void } else { ptr::null() },
            pp_enabled_layer_names: if validation_enabled { validation_layers.as_ptr() } else { ptr::null() },
            enabled_layer_count: if validation_enabled { validation_layers.len() } else { 0 } as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            ..Default::default()
        };

        let instance : ash::Instance = unsafe { entry.create_instance(&create_info, None).expect("Failed to create instance") };
        let debug_messenger = DebugMessenger::new(&entry, &instance);

        Instance {
            instance : instance,
            debug_messenger: debug_messenger,
        }
    }

    fn required_extension_names() -> Vec<*const i8> {
        vec![
            Surface::name().as_ptr(),
            Win32Surface::name().as_ptr()
        ]
    }
}

impl VulkanObject for Instance {
    type Object = ash::Instance;

    fn vulkan_object(&self) -> &Self::Object {
        &self.instance
    }

    fn cleanup(&self) {
        unsafe {
            self.debug_messenger.cleanup();
            self.instance.destroy_instance(None);
        }
    }
}
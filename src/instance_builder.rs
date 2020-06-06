use std::sync::Arc;
use vulkano::{
    device::{Device, DeviceExtensions, Features, Queue},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
};

pub fn build_device_and_queue() -> (Arc<Device>, Arc<Queue>) {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    println!("Physical devices:");
    for physical in PhysicalDevice::enumerate(&instance) {
        println!("{:#?}", physical);
    }

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!("\n\nFound a queue family with {:?} queue(s) with id {}. \n - Graphics: {} \n - Computing: {}", 
        family.queues_count(),
        family.id(),
        family.supports_graphics(),
        family.supports_compute());
    }
    println!("\n");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    (device, queue)
}

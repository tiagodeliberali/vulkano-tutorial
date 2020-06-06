use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
    sync::GpuFuture,
};

use crate::instance_builder::build_device_and_queue;

pub fn copy_data_example() {
    let (device, queue) = build_device_and_queue();

    let source_content = 0..64;
    let source =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
            .expect("failed to create buffer");

    let dest_content = (0..64).map(|_| 0);
    let dest =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dest_content)
            .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.copy_buffer(source.clone(), dest.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();

    assert_eq!(&*src_content, &*dest_content);
    println!("Data copied successfully");
}

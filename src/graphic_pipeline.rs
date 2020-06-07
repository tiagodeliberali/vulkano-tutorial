use image::{ImageBuffer, Rgba};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState},
    format::Format,
    framebuffer::{Framebuffer, Subpass},
    image::{Dimensions, StorageImage},
    pipeline::viewport::Viewport,
    pipeline::GraphicsPipeline,
    sync::GpuFuture,
};

use crate::instance_builder::build_device_and_queue;

pub fn draw_triagle() {
    let (device, queue) = build_device_and_queue();

    #[derive(Default, Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    vulkano::impl_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        vec![vertex1, vertex2, vertex3].into_iter(),
    )
    .unwrap();

    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
            #version 450

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }"
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
            #version 450

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }"
        }
    }

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            // Defines what kind of vertex input is expected.
            .vertex_input_single_buffer::<Vertex>()
            // The vertex shader.
            .vertex_shader(vs.main_entry_point(), ())
            // Defines the viewport (explanations below).
            .viewports_dynamic_scissors_irrelevant(1)
            // The fragment shader.
            .fragment_shader(fs.main_entry_point(), ())
            // This graphics pipeline object concerns the first pass of the render pass.
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            // Now that everything is specified, we call `build`.
            .build(device.clone())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [1024.0, 1024.0],
            depth_range: 0.0..1.0,
        }]),
        ..DynamicState::none()
    };

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();

    let framebuffer = Arc::new(
        Framebuffer::start(render_pass.clone())
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let buf = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    let mut builder =
        AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap();

    builder
        .begin_render_pass(
            framebuffer.clone(),
            false,
            vec![[0.0, 0.0, 1.0, 1.0].into()],
        )
        .unwrap()
        .draw(
            pipeline.clone(),
            &dynamic_state,
            vertex_buffer.clone(),
            (),
            (),
        )
        .unwrap()
        .end_render_pass()
        .unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone())
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("triangle.png").unwrap();

    println!("Everything succeeded!");
}

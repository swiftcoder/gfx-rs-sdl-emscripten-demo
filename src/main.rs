
#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::format::{Formatted};
use gfx::{Adapter, CommandQueue, Device, FrameSync, GraphicsPoolExt,
          Surface, Swapchain, SwapchainExt, WindowExt};
use gfx::traits::DeviceExt;

use std::process;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }
 
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
 
const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];
 
const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    let sdl = sdl2::init().expect("sdl initialisation failed");
    let video = sdl.video().expect("sdl video initialisation failed");

    #[cfg(not(target_os = "emscripten"))]
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 2);
    }

    let builder = video.window("gfx-rs-sdl-emscripten-demo", 1280, 720);
    let (window, _gl_context) = gfx_window_sdl::build(builder, ColorFormat::get_format(), DepthFormat::get_format())
        .expect("could not build gfx window with sdl");
    let mut window = gfx_window_sdl::Window::new(window);
    let (mut surface, adapters) = window.get_surface_and_adapters();

    let gfx::Gpu { mut device, mut graphics_queues, .. } =
        adapters[0].open_with(|family, ty| {
            ((ty.supports_graphics() && surface.supports_queue(&family)) as u32, gfx::QueueType::Graphics)
        });
    let mut graphics_queue = graphics_queues.pop().expect("unable to find a graphics queue");

    let config = gfx::SwapchainConfig::new()
                    .with_color::<ColorFormat>();
    let mut swap_chain = surface.build_swapchain(config, &graphics_queue);
    let views = swap_chain.create_color_views(&mut device);

    let pso = device.create_pipeline_simple(
        include_bytes!("../shaders/triangle_150.glslv"),
        include_bytes!("../shaders/triangle_150.glslf"),
        pipe::new()
    ).expect("failed to create pipeline");
    let (vertex_buffer, slice) = device.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut graphics_pool = graphics_queue.create_graphics_pool(1);
    let frame_semaphore = device.create_semaphore();
    let draw_semaphore = device.create_semaphore();

    let frame_fence = device.create_fence(false);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: views[0].clone(),
    };
    let mut events = sdl.event_pump().expect("failed to pump events");

    let mut main_loop = || {       
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                },
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    println!("{}x{}", w as u32, h as u32);
                },
                _ => {}
            }
        }

        // Get next frame
        let frame = swap_chain.acquire_frame(FrameSync::Semaphore(&frame_semaphore));
        data.out = views[frame.id()].clone();

        // draw a frame
        // wait for frame -> draw -> signal -> present
        {
            let mut encoder = graphics_pool.acquire_graphics_encoder();
            encoder.clear(&data.out, CLEAR_COLOR);
            encoder.draw(&slice, &pso, &data);
            encoder.synced_flush(&mut graphics_queue, &[&frame_semaphore], &[&draw_semaphore], Some(&frame_fence))
                   .expect("Could not flush encoder");
        }

        swap_chain.present(&mut graphics_queue, &[&draw_semaphore]);
        device.wait_for_fences(&[&frame_fence], gfx::WaitFor::All, 1_000_000);
        graphics_queue.cleanup();
        graphics_pool.reset();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}


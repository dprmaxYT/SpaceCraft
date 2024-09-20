mod world;
mod chunk;
mod player;
mod renderer;

use vulkano::instance::{Instance, InstanceExtensions, InstanceCreateInfo};
use vulkano::device::{Device, DeviceExtensions, Features};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use std::sync::Arc;

use world::World;
use player::Player;
use renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let instance = Instance::new(
        InstanceCreateInfo {
            enabled_extensions: InstanceExtensions::none(),
            ..InstanceCreateInfo::default()
        }
    ).expect("failed to create instance");

    let physical = vulkano::instance::Instance::enumerate_physical_devices(&instance)
    .next()
    .expect("no device available");


    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = Device::new(
        physical,
        &vulkano::device::DeviceCreateInfo {
            enabled_extensions: DeviceExtensions::none(),
            ..Default::default()
        },
        [(queue_family, 0.5)].iter().cloned()
    ).expect("failed to create device");
    



    let queue = queues.next().unwrap();

    let mut world = World::new();
    let mut player = Player::new();
    let mut renderer = Renderer::new(Arc::clone(&device), Arc::clone(&queue), window.inner_size());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                if let Some(key) = input.virtual_keycode {
                    player.handle_input(key);
                }
            },
            Event::MainEventsCleared => {
                player.update(&world);
                world.update(&player);
                renderer.render(&world, &player);
            },
            _ => (),
        }
    });
}
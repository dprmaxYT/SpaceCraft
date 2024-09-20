use vulkano::device::{Device, Queue};
use vulkano::swapchain::{Swapchain, Surface};
use vulkano::image::SwapchainImage;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::sync::{self, GpuFuture};
use std::sync::Arc;
use winit::window::Window;
use crate::cube::Cube;

pub struct Renderer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    cube: Cube,
}

impl Renderer {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>, surface: Arc<Surface<Window>>) -> Self {
        let (swapchain, images) = {
            let caps = surface.capabilities(device.physical_device()).unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions = surface.window().inner_size().into();
            Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format, dimensions, 1, caps.supported_usage_flags, &queue, vulkano::swapchain::SurfaceTransform::Identity, vulkano::swapchain::CompositeAlpha::Opaque, vulkano::swapchain::PresentMode::Fifo, true, None).unwrap()
        };

        Renderer {
            device,
            queue,
            swapchain,
            images,
            cube: Cube::new(),
        }
    }

    fn is_face_visible(face_normal: [f32; 3], view_direction: [f32; 3]) -> bool {
        let dot_product = face_normal[0] * view_direction[0] + face_normal[1] * view_direction[1] + face_normal[2] * view_direction[2];
        dot_product < 0.0
    }

    fn calculate_normal(&self, face: &[u32; 3]) -> [f32; 3] {
        // Aquí deberías calcular el vector normal de la cara
        // Esto es solo un ejemplo y necesitarás ajustar según tu implementación
        [0.0, 0.0, 1.0]
    }

    pub fn render(&mut self, world: &crate::world::World, player: &crate::player::Player) {
        let (image_num, suboptimal, acquire_future) = vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family()).unwrap();

        // Aquí es donde agregarías tus comandos de renderizado
        // Por ejemplo, podrías limpiar la pantalla y dibujar objetos

        let view_direction = [0.0, 0.0, -1.0]; // Dirección de la vista del jugador, ajusta según sea necesario

        for face in &self.cube.indices {
            let face_normal = self.calculate_normal(face);
            if Renderer::is_face_visible(face_normal, view_direction) {
                // Renderiza la cara
                // Aquí deberías agregar los comandos para renderizar la cara visible
            }
        }

        let command_buffer = builder.build().unwrap();
        let future = acquire_future
            .then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush().unwrap();

        future.wait(None).unwrap();
    }
}
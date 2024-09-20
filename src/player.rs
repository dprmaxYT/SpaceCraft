use nalgebra::Vector3;
use winit::event::VirtualKeyCode;

pub struct Player {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, world: &crate::world::World) {
        self.position += self.velocity;
        self.velocity *= 0.9; // Add some drag
    }

    pub fn handle_input(&mut self, key: VirtualKeyCode) {
        let speed = 0.5;
        match key {
            VirtualKeyCode::W => self.velocity.z -= speed,
            VirtualKeyCode::S => self.velocity.z += speed,
            VirtualKeyCode::A => self.velocity.x -= speed,
            VirtualKeyCode::D => self.velocity.x += speed,
            VirtualKeyCode::Space => self.velocity.y += speed,
            VirtualKeyCode::LShift => self.velocity.y -= speed,
            _ => {},
        }
    }
}

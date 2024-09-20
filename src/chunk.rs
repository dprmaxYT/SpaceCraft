use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};

pub struct Chunk {
    position: Vector3<i32>,
    blocks: [[[u8; 16]; 16]; 16],
}

impl Chunk {
    pub fn generate(position: Vector3<i32>) -> Self {
        let mut chunk = Chunk {
            position,
            blocks: [[[0; 16]; 16]; 16],
        };

        let noise = Perlin::new(0); // Proporciona un seed aquí
        
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let world_x = position.x * 16 + x as i32;
                    let world_y = position.y * 16 + y as i32;
                    let world_z = position.z * 16 + z as i32;
                    
                    let density = noise.get([
                        world_x as f64 * 0.01,
                        world_y as f64 * 0.01,
                        world_z as f64 * 0.01
                    ]);
                    
                    chunk.blocks[x][y][z] = if density > 0.0 { 1 } else { 0 };
                }
            }
        }

        chunk
    }

    pub fn get_block(&self, position: Vector3<usize>) -> Option<u8> {
        self.blocks.get(position.x)?
            .get(position.y)?
            .get(position.z)
            .copied()
    }
}

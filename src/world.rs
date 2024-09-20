use crate::chunk::Chunk;
use nalgebra::Vector3;
use std::collections::HashMap;

pub struct World {
    chunks: HashMap<Vector3<i32>, Chunk>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: HashMap::new(),
        }
    }

    pub fn update(&mut self, player: &crate::player::Player) {
        let player_chunk = player.position.map(|v| (v / 16.0).floor() as i32);
        
        for x in -2..=2 {
            for y in -2..=2 {
                for z in -2..=2 {
                    let chunk_pos = player_chunk + Vector3::new(x, y, z);
                    if !self.chunks.contains_key(&chunk_pos) {
                        self.chunks.insert(chunk_pos, Chunk::generate(chunk_pos));
                    }
                }
            }
        }
    }

    pub fn get_block(&self, position: Vector3<f32>) -> Option<u8> {
        let chunk_pos = position.map(|v| (v / 16.0).floor() as i32);
        let local_pos = position.map(|v| (v % 16.0).floor() as usize);
        
        self.chunks.get(&chunk_pos).and_then(|chunk| chunk.get_block(local_pos))
    }
}

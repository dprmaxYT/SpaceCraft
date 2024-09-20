pub struct Cube {
    pub vertices: [[f32; 3]; 8],
    pub indices: [[u32; 3]; 12],
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            vertices: [
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
            ],
            indices: [
                [0, 1, 2], [2, 3, 0], // Front face
                [4, 5, 6], [6, 7, 4], // Back face
                [0, 1, 5], [5, 4, 0], // Bottom face
                [2, 3, 7], [7, 6, 2], // Top face
                [0, 3, 7], [7, 4, 0], // Left face
                [1, 2, 6], [6, 5, 1], // Right face
            ],
        }
    }
}
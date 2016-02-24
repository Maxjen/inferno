#[derive(Copy, Clone)]
pub struct SpriteVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [u8; 4],
}

implement_vertex!(SpriteVertex, position, tex_coords, color);

#[derive(Copy, Clone)]
pub struct ColorVertex2d {
    pub position: [f32; 2],
    pub color: [u8; 4],
}

implement_vertex!(ColorVertex2d, position, color);

#[derive(Copy, Clone)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [u8; 4],
}

implement_vertex!(ColorVertex, position, normal, color);

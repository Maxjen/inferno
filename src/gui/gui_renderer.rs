use ::resources::Texture;
use ::rendering::{DrawBatch, SpriteVertex};

pub struct Image {
    texture: Texture,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Image {
    pub fn new(texture: Texture, x: f32, y: f32, width: f32, height: f32) -> Image {
        Image {
            texture: texture,
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn add_to_batch(&self, batch: &mut DrawBatch) {
        let (x, y, width, height) = (self.x, self.y, self.width, self.height);
        let (u_min, u_max, v_min, v_max) = (self.texture.uv_min.0, self.texture.uv_max.0,
                                            self.texture.uv_min.1, self.texture.uv_max.1);
        let vertices = [
            SpriteVertex {
                position: [x, y],
                tex_coords: [u_min, v_min],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width, y],
                tex_coords: [u_max, v_min],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x, y - height],
                tex_coords: [u_min, v_max],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width, y - height],
                tex_coords: [u_max, v_max],
                color: [255, 255, 255, 255]
            },
        ];
        let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
        batch.add_sprite_triangles(self.texture.atlas.clone(), &vertices, &indices);
    }
}

pub struct BorderImage {
    texture: Texture,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    border_left: f32,
    border_right: f32,
    border_top: f32,
    border_bottom: f32,
}

impl BorderImage {
    pub fn new(texture: Texture, x: f32, y: f32, width: f32, height: f32,
           border_left: f32, border_right: f32, border_top: f32, border_bottom: f32) -> BorderImage {
        BorderImage {
            texture: texture,
            x: x,
            y: y,
            width: width,
            height: height,
            border_left: border_left,
            border_right: border_right,
            border_top: border_top,
            border_bottom: border_bottom,
        }
    }

    pub fn add_to_batch(&self, batch: &mut DrawBatch) {
        let (x, y, width, height) = (self.x, self.y, self.width, self.height);
        let (u_min, u_max, v_min, v_max) = (self.texture.uv_min.0, self.texture.uv_max.0,
                                            self.texture.uv_min.1, self.texture.uv_max.1);
        let (left, right, top, bottom) = (self.border_left, self.border_right,
                                          self.border_top, self.border_bottom);
        let pixel_dimension = self.texture.pixel_dimension;
        let (left_u, right_u, top_v, bottom_v) = (left * pixel_dimension, right * pixel_dimension,
                                                  top * pixel_dimension, bottom * pixel_dimension);
        let vertices = [
            SpriteVertex {
                position: [x, y],
                tex_coords: [u_min, v_min],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + left, y],
                tex_coords: [u_min + left_u, v_min],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width - right, y],
                tex_coords: [u_max - right_u, v_min],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width, y],
                tex_coords: [u_max, v_min],
                color: [255, 255, 255, 255]
            },

            SpriteVertex {
                position: [x, y - top],
                tex_coords: [u_min, v_min + top_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + left, y - top],
                tex_coords: [u_min + left_u, v_min + top_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width - right, y - top],
                tex_coords: [u_max - right_u, v_min + top_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width,y - top],
                tex_coords: [u_max, v_min + top_v],
                color: [255, 255, 255, 255]
            },

            SpriteVertex {
                position: [x, y - height + bottom],
                tex_coords: [u_min, v_max - bottom_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + left, y - height + bottom],
                tex_coords: [u_min + left_u, v_max - bottom_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width - right, y - height + bottom],
                tex_coords: [u_max - right_u, v_max - bottom_v],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width, y - height + bottom],
                tex_coords: [u_max, v_max - bottom_v],
                color: [255, 255, 255, 255]
            },

            SpriteVertex {
                position: [x, y - height],
                tex_coords: [u_min, v_max],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + left, y - height],
                tex_coords: [u_min + left_u, v_max],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width - right, y - height],
                tex_coords: [u_max - right_u, v_max],
                color: [255, 255, 255, 255]
            },
            SpriteVertex {
                position: [x + width, y - height],
                tex_coords: [u_max, v_max],
                color: [255, 255, 255, 255]
            },
        ];
        let indices: [u32; 54] = [0, 4, 1, 1, 4, 5, 1, 5, 2, 2, 5, 6, 2, 6, 3, 3, 6, 7,
                                 4, 8, 5, 5, 8, 9, 5, 9, 6, 6, 9, 10, 6, 10, 7, 7, 10, 11,
                                 8, 12, 9, 9, 12, 13, 9, 13, 10, 10, 13, 14, 10, 14, 11, 11, 14, 15];
        batch.add_sprite_triangles(self.texture.atlas.clone(), &vertices, &indices);
    }
}

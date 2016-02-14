use ::resources::{Texture, Font};
use ::rendering::{DrawBatch, SpriteVertex};

#[derive(Clone)]
pub struct Image {
    texture: Texture,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Image {
    pub fn new(texture: Texture, x: f32, y: f32, width: f32, height: f32) -> Self {
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
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width, y],
                tex_coords: [u_max, v_min],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x, y - height],
                tex_coords: [u_min, v_max],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width, y - height],
                tex_coords: [u_max, v_max],
                color: [255, 255, 255, 255],
            },
        ];
        let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
        batch.add_sprite_triangles(self.texture.atlas.clone(), &vertices, &indices);
    }
}

#[derive(Clone)]
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
    pub fn new(texture: Texture, border_left: f32, border_right: f32,
               border_top: f32, border_bottom: f32) -> Self {
        BorderImage {
            texture: texture,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            border_left: border_left,
            border_right: border_right,
            border_top: border_top,
            border_bottom: border_bottom,
        }
    }
    pub fn new_with_values(texture: Texture, x: f32, y: f32, width: f32, height: f32,
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

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
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
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + left, y],
                tex_coords: [u_min + left_u, v_min],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width - right, y],
                tex_coords: [u_max - right_u, v_min],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width, y],
                tex_coords: [u_max, v_min],
                color: [255, 255, 255, 255],
            },

            SpriteVertex {
                position: [x, y - top],
                tex_coords: [u_min, v_min + top_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + left, y - top],
                tex_coords: [u_min + left_u, v_min + top_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width - right, y - top],
                tex_coords: [u_max - right_u, v_min + top_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width,y - top],
                tex_coords: [u_max, v_min + top_v],
                color: [255, 255, 255, 255],
            },

            SpriteVertex {
                position: [x, y - height + bottom],
                tex_coords: [u_min, v_max - bottom_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + left, y - height + bottom],
                tex_coords: [u_min + left_u, v_max - bottom_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width - right, y - height + bottom],
                tex_coords: [u_max - right_u, v_max - bottom_v],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width, y - height + bottom],
                tex_coords: [u_max, v_max - bottom_v],
                color: [255, 255, 255, 255],
            },

            SpriteVertex {
                position: [x, y - height],
                tex_coords: [u_min, v_max],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + left, y - height],
                tex_coords: [u_min + left_u, v_max],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width - right, y - height],
                tex_coords: [u_max - right_u, v_max],
                color: [255, 255, 255, 255],
            },
            SpriteVertex {
                position: [x + width, y - height],
                tex_coords: [u_max, v_max],
                color: [255, 255, 255, 255],
            },
        ];
        let indices: [u32; 54] = [0, 4, 1, 1, 4, 5, 1, 5, 2, 2, 5, 6, 2, 6, 3, 3, 6, 7,
                                 4, 8, 5, 5, 8, 9, 5, 9, 6, 6, 9, 10, 6, 10, 7, 7, 10, 11,
                                 8, 12, 9, 9, 12, 13, 9, 13, 10, 10, 13, 14, 10, 14, 11, 11, 14, 15];
        batch.add_sprite_triangles(self.texture.atlas.clone(), &vertices, &indices);
    }
}

#[derive(Clone)]
pub struct Text {
    font: Font,
    text: String,
    x: f32,
    y: f32,
    color: (u8, u8, u8, u8),
    width: f32,
    //height: f32,
}

impl Text {
    pub fn new(font: Font, text: &str) -> Self {
        let mut result = Text {
            font: font,
            text: text.to_string(),
            x: 0.0,
            y: 0.0,
            color: (255, 255, 255, 255),
            width: 0.0,
            //height: 0.0,
        };
        let mut width = 0.0;
        let mut last = None;
        for c in text.chars() {
            let glyphs = result.font.glyphs.borrow();
            let glyph = match glyphs.get(&c) {
                Some(glyph) => glyph,
                None => continue,
            };
            let mut kerning: f32 = 0.0;
            if let Some(last) = last {
                kerning = *result.font.kernings.borrow().get(&(last, c)).unwrap_or(&0.0);
            }
            last = Some(c);
            width += glyph.advance_x + kerning;
        }
        result.width = width;
        result
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color = (r, g, b, a);
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    fn add_letter(&self, letter: char, x: f32, shadow: bool, batch: &mut DrawBatch) {
        let glyphs = self.font.glyphs.borrow();
        let glyph = match glyphs.get(&letter) {
            Some(glyph) => glyph,
            None => return,
        };

        let (r, g, b, a) = match shadow {
            true => (0, 0, 0, 255),
            false => (self.color.0, self.color.1, self.color.2, self.color.3),
        };
        let (u_min, u_max, v_min, v_max) = (glyph.uv_min.0, glyph.uv_max.0,
                                            glyph.uv_min.1, glyph.uv_max.1);
        let (width, height) = (glyph.width, glyph.height);
        let (offset_x, offset_y) = glyph.offset;
        let x = x + offset_x;
        let mut y = self.y + offset_y;

        if shadow {
            y -= 1.0;
        }

        if width != 0.0 {
            let vertices = [
                SpriteVertex {
                    position: [x, y],
                    tex_coords: [u_min, v_min],
                    color: [r, g, b, a],
                },
                SpriteVertex {
                    position: [x + width, y],
                    tex_coords: [u_max, v_min],
                    color: [r, g, b, a],
                },
                SpriteVertex {
                    position: [x, y - height],
                    tex_coords: [u_min, v_max],
                    color: [r, g, b, a],
                },
                SpriteVertex {
                    position: [x + width, y - height],
                    tex_coords: [u_max, v_max],
                    color: [r, g, b, a],
                },
            ];
            let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
            batch.add_font_triangles(self.font.atlas.clone(), &vertices, &indices);
        }
    }

    pub fn add_to_batch(&self, batch: &mut DrawBatch) {
        let mut x = self.x;
        //let mut y;

        let mut last = None;
        for c in self.text.chars() {
            let glyphs = self.font.glyphs.borrow();
            let glyph = match glyphs.get(&c) {
                Some(glyph) => glyph,
                None => continue,
            };

            /*let (r, g, b, a) = (self.color.0, self.color.1, self.color.2, self.color.3);
            let (u_min, u_max, v_min, v_max) = (glyph.uv_min.0, glyph.uv_max.0,
                                                glyph.uv_min.1, glyph.uv_max.1);
            let (width, height) = (glyph.width, glyph.height);
            let (offset_x, offset_y) = glyph.offset;*/

            let mut kerning: f32 = 0.0;
            if let Some(last) = last {
                kerning = *self.font.kernings.borrow().get(&(last, c)).unwrap_or(&0.0);
            }
            x += kerning;
            //y = self.y + offset_y;

            self.add_letter(c, x, true, batch);
            self.add_letter(c, x, false, batch);

            /*if width != 0.0 {
                let x = x + offset_x;
                let vertices = [
                    SpriteVertex {
                        position: [x, y],
                        tex_coords: [u_min, v_min],
                        color: [r, g, b, a],
                    },
                    SpriteVertex {
                        position: [x + width, y],
                        tex_coords: [u_max, v_min],
                        color: [r, g, b, a],
                    },
                    SpriteVertex {
                        position: [x, y - height],
                        tex_coords: [u_min, v_max],
                        color: [r, g, b, a],
                    },
                    SpriteVertex {
                        position: [x + width, y - height],
                        tex_coords: [u_max, v_max],
                        color: [r, g, b, a],
                    },
                ];
                let indices: [u32; 6] = [0, 2, 1, 1, 2, 3];
                batch.add_font_triangles(self.font.atlas.clone(), &vertices, &indices);
            }*/

            last = Some(c);
            x += glyph.advance_x;
        }
    }
}

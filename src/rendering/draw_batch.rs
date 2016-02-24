use glium;
use super::{PolygonMode, Color2dBatch, ColorTriangleBatch, SpriteTriangleBatch, FontTriangleBatch};
use super::vertex::{ColorVertex2d, ColorVertex, SpriteVertex};
use ::resources::TextureAtlas;
use std::rc::Rc;
use std::cell::RefCell;

enum BatchType<'a> {
    Color2d(Color2dBatch<'a>),
    ColorTriangle(ColorTriangleBatch<'a>),
    SpriteTriangle(SpriteTriangleBatch<'a>),
    FontTriangle(FontTriangleBatch<'a>),
}

pub struct DrawBatch<'a> {
    display: &'a glium::backend::glutin_backend::GlutinFacade,
    batches: Vec<BatchType<'a>>,
}

impl<'a> DrawBatch<'a> {
    pub fn new(display: &'a glium::backend::glutin_backend::GlutinFacade) -> DrawBatch<'a> {
        DrawBatch { display: display, batches: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.batches.clear();
    }

    pub fn add_color_2d_points(&mut self, vertices: &[ColorVertex2d]) {
        let mut indices = Vec::new();
        for i in 0..vertices.len() {
            indices.push(i as u32);
        }

        if let Some(&mut BatchType::Color2d(ref mut batch)) = self.batches.last_mut() {
            if batch.get_polygon_mode() == PolygonMode::Point {
                batch.add_color_vertices(vertices, &indices);
                return;
            }
        }
        let mut batch = Color2dBatch::new(self.display, PolygonMode::Point);
        batch.add_color_vertices(vertices, &indices);
        self.batches.push(BatchType::Color2d(batch));
    }

    pub fn add_color_2d_lines(&mut self, vertices: &[ColorVertex2d], indices: &[u32]) {
        if let Some(&mut BatchType::Color2d(ref mut batch)) = self.batches.last_mut() {
            if batch.get_polygon_mode() == PolygonMode::Line {
                batch.add_color_vertices(vertices, indices);
                return;
            }
        }
        let mut batch = Color2dBatch::new(self.display, PolygonMode::Line);
        batch.add_color_vertices(vertices, indices);
        self.batches.push(BatchType::Color2d(batch));
    }

    pub fn add_color_2d_triangles(&mut self, vertices: &[ColorVertex2d], indices: &[u32]) {
        if let Some(&mut BatchType::Color2d(ref mut batch)) = self.batches.last_mut() {
            if batch.get_polygon_mode() == PolygonMode::Triangle {
                batch.add_color_vertices(vertices, indices);
                return;
            }
        }
        let mut batch = Color2dBatch::new(self.display, PolygonMode::Triangle);
        batch.add_color_vertices(vertices, indices);
        self.batches.push(BatchType::Color2d(batch));
    }

    pub fn add_color_triangles(&mut self, vertices: &[ColorVertex], indices: &[u32]) {
        if let Some(&mut BatchType::ColorTriangle(ref mut batch)) = self.batches.last_mut() {
            batch.add_color_triangles(vertices, indices);
            return;
        }
        let mut batch = ColorTriangleBatch::new(self.display, true);
        batch.add_color_triangles(vertices, indices);
        self.batches.push(BatchType::ColorTriangle(batch));
    }

    pub fn add_sprite_triangles(&mut self, atlas: Rc<RefCell<TextureAtlas>>, vertices: &[SpriteVertex], indices: &[u32]) {
        if let Some(&mut BatchType::SpriteTriangle(ref mut batch)) = self.batches.last_mut() {
            let p1 = &(*atlas) as *const RefCell<TextureAtlas>;
            let p2 = &(*batch.atlas) as *const RefCell<TextureAtlas>;
            if p1 == p2 {
                batch.add_sprite_triangles(vertices, indices);
                return;
            }
        }
        let mut batch = SpriteTriangleBatch::new(self.display, atlas);
        batch.add_sprite_triangles(vertices, indices);
        self.batches.push(BatchType::SpriteTriangle(batch));
    }

    pub fn add_font_triangles(&mut self, atlas: Rc<RefCell<TextureAtlas>>, vertices: &[SpriteVertex], indices: &[u32]) {
        if let Some(&mut BatchType::FontTriangle(ref mut batch)) = self.batches.last_mut() {
            let p1 = &(*atlas) as *const RefCell<TextureAtlas>;
            let p2 = &(*batch.atlas) as *const RefCell<TextureAtlas>;
            if p1 == p2 {
                batch.add_font_triangles(vertices, indices);
                return;
            }
        }
        let mut batch = FontTriangleBatch::new(self.display, atlas);
        batch.add_font_triangles(vertices, indices);
        self.batches.push(BatchType::FontTriangle(batch));
    }

    pub fn create_buffers(&mut self) {
        for batch in self.batches.iter_mut() {
            match batch {
                &mut BatchType::Color2d(ref mut c2db) => c2db.create_buffers(),
                &mut BatchType::ColorTriangle(ref mut ctb) => ctb.create_buffers(),
                &mut BatchType::SpriteTriangle(ref mut stb) => stb.create_buffers(),
                &mut BatchType::FontTriangle(ref mut ftb) => ftb.create_buffers(),
            }
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        for batch in self.batches.iter() {
            match batch {
                &BatchType::Color2d(ref c2db) => c2db.draw(frame),
                &BatchType::ColorTriangle(ref ctb) => ctb.draw(frame),
                &BatchType::SpriteTriangle(ref stb) => stb.draw(frame),
                &BatchType::FontTriangle(ref ftb) => ftb.draw(frame),
            }
        }
    }
}

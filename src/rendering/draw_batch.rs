use glium;
use super::color_triangle_batch::ColorTriangleBatch;
use super::sprite_triangle_batch::SpriteTriangleBatch;
use super::vertex::ColorVertex;
use super::vertex::SpriteVertex;
use ::resources::TextureAtlas;
use std::rc::Rc;
use std::cell::RefCell;

enum BatchType<'a> {
    ColorTriangle(ColorTriangleBatch<'a>),
    SpriteTriangle(SpriteTriangleBatch<'a>),
}

pub struct DrawBatch<'a> {
    display: &'a glium::backend::glutin_backend::GlutinFacade,
    batches: Vec<BatchType<'a>>,
}

impl<'a> DrawBatch<'a> {
    pub fn new(display: &'a glium::backend::glutin_backend::GlutinFacade) -> DrawBatch<'a> {
        DrawBatch { display: display, batches: Vec::new() }
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

    pub fn create_buffers(&mut self) {
        for batch in self.batches.iter_mut() {
            match batch {
                &mut BatchType::ColorTriangle(ref mut ctb) => ctb.create_buffers(),
                &mut BatchType::SpriteTriangle(ref mut stb) => stb.create_buffers(),
            }
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame) {
        for batch in self.batches.iter_mut() {
            match batch {
                &mut BatchType::ColorTriangle(ref mut ctb) => ctb.draw(frame),
                &mut BatchType::SpriteTriangle(ref mut stb) => stb.draw(frame),
            }
        }
    }
}

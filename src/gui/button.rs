use super::{Widget, Rectangle, BorderImage};
use ::resources::ResourceManager;
use ::rendering::DrawBatch;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Button {
    button_image: BorderImage,
    rect: Rectangle,
}

impl Button {
    pub fn new(resource_manager: &mut ResourceManager) -> Button {
        let texture = resource_manager.create_texture("example_images/button_shadow.png").unwrap();
        Button {
            button_image: BorderImage::new(texture, 14.0, 14.0, 13.0, 15.0),
            rect: Rectangle::new(),
        }
    }
}

impl Widget for Button {
    fn set_position(&mut self, x: i32, y: i32) {
        self.rect.position = (x, y);
        self.button_image.set_position(x as f32, y as f32);
    }

    fn set_dimensions(&mut self, width: i32, height: i32) {
        self.rect.dimensions = (width, height);
        self.button_image.set_size(width as f32, height as f32);
    }

    fn add_to_batch(&self, batch: &mut DrawBatch) {
        self.button_image.add_to_batch(batch);
    }

    fn get_highest_priority_child(&self, x: i32, y: i32) -> (i32, Option<Rc<RefCell<Widget>>>) {
        if self.rect.contains(x, y) {
            return (3, None);
        }
        (0, None)
    }
}

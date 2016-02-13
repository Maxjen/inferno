use ::rendering::DrawBatch;
use ::resources::ResourceManager;
use super::BorderImage;
use super::{Widget, Rectangle, EventListener};
use glium;
use std::option::Option;
use std::rc::Rc;
use std::cell::RefCell;

const PADDING: i32 = 5;

pub struct Window<'a> {
    rect: Rectangle,
    draw_batch: DrawBatch<'a>,
    window_background: BorderImage,
    child: Option<Rc<RefCell<Widget>>>
}

impl<'a> Window<'a> {
    pub fn new(
        resource_manager: &mut ResourceManager,
        display: &'a glium::backend::glutin_backend::GlutinFacade,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) -> Window<'a> {
        let texture = resource_manager.create_texture("example_images/window_sq.png").unwrap();
        let mut window_background = BorderImage::new(texture, 21.0, 21.0, 20.0, 22.0);
        window_background.set_position((x - 20) as f32, (y + 19) as f32);
        window_background.set_size((width + 40) as f32, (height + 40) as f32);
        Window {
            rect: Rectangle::new_with_values(x, y, width, height),
            draw_batch: DrawBatch::new(display),
            window_background: window_background,
            child: None,
        }
    }

    pub fn set_child(&mut self, child: Rc<RefCell<Widget>>) {
        self.child = Some(child);
    }

    pub fn create_buffers(&mut self) {
        self.draw_batch.clear();
        self.window_background.add_to_batch(&mut self.draw_batch);
        if let Some(ref child) = self.child {
            child.borrow_mut().set_dimensions(self.rect.dimensions.0 - 2 * PADDING, self.rect.dimensions.1 - 2 * PADDING);
            child.borrow_mut().set_position(self.rect.position.0 + PADDING, self.rect.position.1 - PADDING);
            child.borrow_mut().add_to_batch(&mut self.draw_batch);
        }
        self.draw_batch.create_buffers();
    }

    pub fn test(&self, x: i32, y: i32) {
        if let Some(ref child) = self.child {
            let (i, _) = child.borrow().get_highest_priority_child(x, y);
            println!("{}", i);
        }
    }

    pub fn create_event_listener(&self, x: i32, y: i32) -> Option<Box<EventListener>> {
        if let Some(ref child) = self.child {
            let (i, widget) = child.borrow().get_highest_priority_child(x, y);
            if let Some(widget) = widget {
                println!("{}", i);
                return widget.borrow().create_event_listener(x, y);
            }
        }
        None
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        self.draw_batch.draw(frame);
    }
}

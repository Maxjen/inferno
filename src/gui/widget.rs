//use glium;
use std::rc::Rc;
use std::cell::RefCell;
use ::rendering::DrawBatch;
use glium::glutin::Event;

pub trait Widget {
    //fn draw(&self, frame: &mut glium::Frame);
    fn set_position(&mut self, x: i32, y: i32);
    fn set_dimensions(&mut self, width: i32, height: i32);
    fn add_to_batch(&self, batch: &mut DrawBatch);
    fn get_highest_priority_child(&self, x: i32, y: i32) -> (i32, Option<Rc<RefCell<Widget>>>);
    fn create_event_listener(&self, x: i32, y: i32) -> Option<Box<EventListener>> { None }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub position: (i32, i32),
    pub dimensions: (i32, i32),
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            position: (0, 0),
            dimensions: (0, 0),
        }
    }

    pub fn new_with_values(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            position: (x, y),
            dimensions: (width, height),
        }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.position.0 && x < self.position.0 + self.dimensions.0 &&
        y >= -self.position.1 && y < -self.position.1 + self.dimensions.1
    }
}

pub trait EventListener {
    fn handle_event(&mut self, event: Event) -> bool;
    fn add_to_batch(&self, batch: &mut DrawBatch) {}
}

#[macro_use]
extern crate glium;
extern crate image;
extern crate inferno;

#[path = "data/teapot.rs"]
mod teapot;

use inferno::resources::{ResourceManager, Font};
use inferno::rendering::{DrawBatch, ColorVertex2d, ColorVertex};
use inferno::gui::{EventListener, Image, Text, Window, Button, Docks};

use std::rc::Rc;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                        .with_dimensions(800, 600)
                        .with_depth_buffer(24)
                        .build_glium().unwrap();

    let mut resource_manager = ResourceManager::new();
    let inferno_logo_texture = resource_manager.create_texture("example_images/inferno-logo2.png").unwrap();

    let gui_font = resource_manager.create_font("DejaVuSans.ttf", 14).unwrap();
    gui_font.load_glyphs();

    //let font = resource_manager.create_font("Gudea-Regular.ttf", 28).unwrap();
    let font = resource_manager.create_font("DejaVuSans.ttf", 28).unwrap();
    font.load_glyphs();
    let mut text = Text::new(font.clone(), "Inferno Test");
    text.set_position(650.0, -150.0);
    text.set_color(255, 255, 0, 255);


    let mut teapot_vertices = Vec::<ColorVertex>::new();
    for (p, n) in teapot::VERTICES.iter().zip(teapot::NORMALS.iter()) {
        teapot_vertices.push(ColorVertex { position: [p.position.0, p.position.1, p.position.2],
                                           normal: [n.normal.0, n.normal.1, n.normal.2],
                                           color: [0u8, 255u8, 0u8, 255u8] });
    }

    let mut teapot_indices = Vec::<u32>::new();
    for i in teapot::INDICES.iter() {
        teapot_indices.push(*i as u32);
    }

    let inferno_logo = Image::new(inferno_logo_texture.clone(), 650.0, 0.0, 128.0, 128.0);

    let mut batch = DrawBatch::new(&display);
    batch.add_color_triangles(&teapot_vertices, &teapot_indices);
    inferno_logo.add_to_batch(&mut batch);
    batch.create_buffers();

    let mut overlay_batch = DrawBatch::new(&display);

    //let mut window = Window::new(&mut resource_manager, &display, -470, 330, 600, 450);
    let mut window = Window::new(&mut resource_manager, &display, 0, 0, 600, 450);
    /*let button = Rc::new(Button::new(&mut resource_manager));
    window.set_child(button);*/
    let docks = Docks::new(&mut resource_manager);
    docks.borrow_mut().add_test_docks();
    window.set_child(docks.clone());
    window.create_buffers();

    let atlas = inferno_logo_texture.atlas.clone();
    atlas.borrow_mut().upload(&display);

    gui_font.atlas.borrow_mut().upload(&display);
    font.atlas.borrow_mut().upload(&display);

    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;

    let mut event_listener: Option<Box<EventListener>> = None;

    loop {
        use glium::glutin::Event;

        let mut target = display.draw();

        target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

        batch.draw(&mut target);
        window.create_buffers();
        window.draw(&mut target);
        text.add_to_batch(&mut overlay_batch);
        overlay_batch.create_buffers();
        overlay_batch.draw(&mut target);
        overlay_batch.clear();

        target.finish().unwrap();

        let mut remove_listener = false;

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::MouseMoved((x, y)) => {
                    mouse_x = x;
                    mouse_y = y;
                    if let Some(ref mut event_listener) = event_listener {
                        remove_listener = remove_listener || event_listener.handle_event(ev);
                    }
                }
                Event::MouseInput(glium::glutin::ElementState::Pressed, glium::glutin::MouseButton::Left) => {
                    if let Some(ref mut event_listener) = event_listener {
                        remove_listener = remove_listener || event_listener.handle_event(ev);
                    } else {
                        event_listener = window.create_event_listener(mouse_x, mouse_y);
                    }
                }
                Event::MouseInput(glium::glutin::ElementState::Released, glium::glutin::MouseButton::Left) => {
                    if let Some(ref mut event_listener) = event_listener {
                        remove_listener = remove_listener || event_listener.handle_event(ev);
                    }
                }
                _ => ()
            }
        }
        if remove_listener {
            event_listener = None;
        }
        if let Some(ref mut event_listener) = event_listener {
            event_listener.add_to_batch(&mut overlay_batch);
        }
        //println!("{} {}", mouse_x, mouse_y);
    }
}

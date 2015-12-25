#[macro_use]
extern crate glium;
extern crate image;
extern crate inferno;

mod teapot;

use inferno::resources::ResourceManager;
use inferno::rendering::{DrawBatch, ColorVertex, ColorTriangleBatch, SpriteVertex, SpriteTriangleBatch};
use inferno::gui::{Image, BorderImage};

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                        .with_depth_buffer(24)
                        .build_glium().unwrap();

    let mut resource_manager = ResourceManager::new();

    let t1 = resource_manager.create_texture("example_images/rust-logo.png").unwrap();
    let t2 = resource_manager.create_texture("example_images/window_sq.png").unwrap();

    let atlas = t1.atlas.clone();
    atlas.borrow_mut().upload(&display);

    /*let quad = [
        SpriteVertex {
            position: [0.0, 0.0],
            tex_coords: [0.0, 0.0],
            color: [255, 255, 255, 255]
        },
        SpriteVertex {
            position: [0.0, -2000.0],
            tex_coords: [0.0, 1.0],
            color: [255, 255, 255, 255]
        },
        SpriteVertex {
            position: [2000.0, -2000.0],
            tex_coords: [1.0, 1.0],
            color: [255, 255, 255, 255]
        },
        SpriteVertex {
            position: [2000.0, 0.0],
            tex_coords: [1.0, 0.0],
            color: [255, 255, 255, 255]
        }
    ];

    let quad_indices: [u32; 6] = [0, 1, 2, 0, 2, 3];*/

    /*let mut sprite_batch = SpriteTriangleBatch::new(&display, atlas);
    sprite_batch.add_sprite_triangles(&quad, &quad_indices);
    sprite_batch.create_buffers();*/


    /*let triangle = [
        ColorVertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            color: [255, 255, 255, 255]
        },
        ColorVertex {
            position: [100.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            color: [255, 255, 255, 255]
        },
        ColorVertex {
            position: [100.0, 100.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            color: [255, 255, 255, 255]
        }
    ];

    let indices: [u32; 3] = [0, 1, 2];

    let mut batch_triangle = ColorTriangleBatch::new(&display, false);
    batch_triangle.add_color_triangles(&triangle, &indices);
    batch_triangle.create_buffers();*/

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

    /*let mut batch_teapot = ColorTriangleBatch::new(&display, true);
    batch_teapot.add_color_triangles(&teapot_vertices, &teapot_indices);
    batch_teapot.create_buffers();*/

    let rust = Image::new(t1.clone(), 0.0, 0.0, 256.0, 256.0);
    let border_image = BorderImage::new(t2.clone(), -100.0, 100.0, 500.0, 300.0, 21.0, 21.0, 20.0, 22.0);


    let mut batch = DrawBatch::new(&display);
    batch.add_color_triangles(&teapot_vertices, &teapot_indices);
    border_image.add_to_batch(&mut batch);
    rust.add_to_batch(&mut batch);
    batch.create_buffers();

    loop {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        batch.draw(&mut target);

        /*{
            batch_triangle.draw(&mut target);
        }
        {
            batch_teapot.draw(&mut target);
        }
        {
            sprite_batch.draw(&mut target);
        }*/

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

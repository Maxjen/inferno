use glium;
use super::vertex::ColorVertex2d;

#[derive(PartialEq, Copy, Clone)]
pub enum PolygonMode {
    Point,
    Line,
    Triangle,
}

pub struct Color2dBatch<'a> {
    display: &'a glium::backend::glutin_backend::GlutinFacade,
    polygon_mode: PolygonMode,
    program: glium::Program,
    vertices: Vec<ColorVertex2d>,
    indices: Vec<u32>,
    vertex_buffer: Option<glium::VertexBuffer<ColorVertex2d>>,
    index_buffer: Option<glium::IndexBuffer<u32>>,
}

impl<'a> Color2dBatch<'a> {
    pub fn new(display: &'a glium::backend::glutin_backend::GlutinFacade, polygon_mode: PolygonMode) -> Self {
        let vertex_shader_src = r#"
            #version 150

            in vec2 position;
            in vec4 color;

            out vec4 v_color;

            uniform mat4 projection;
            uniform mat4 matrix;

            void main() {
                gl_Position = projection * matrix * vec4(position, 0.0, 1.0);
                v_color = color * vec4(0.00392156862, 0.00392156862, 0.00392156862, 0.00392156862);;
            }
        "#;

        let fragment_shader_src = r#"
            #version 150

            in vec4 v_color;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = v_color;
            }
        "#;

        Color2dBatch {
            display: display,
            polygon_mode: polygon_mode,
            //program: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            program: glium::Program::new(display, glium::program::ProgramCreationInput:: SourceCode {
                vertex_shader: vertex_shader_src,
                fragment_shader: fragment_shader_src,
                geometry_shader: None,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: false,
            }).unwrap(),
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn get_polygon_mode(&self) -> PolygonMode {
        self.polygon_mode
    }

    pub fn add_color_vertices(&mut self, vertices: &[ColorVertex2d], indices: &[u32]) {
        let index_offset: u32 = self.vertices.len() as u32;
        for v in vertices {
            self.vertices.push(*v);
        }
        for i in indices {
            self.indices.push(*i + index_offset);
        }
    }

    pub fn create_buffers(&mut self) {
        self.vertex_buffer = Some(glium::VertexBuffer::new(self.display, &self.vertices).unwrap());
        self.index_buffer = match self.polygon_mode {
            PolygonMode::Point => Some(glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::Points, &self.indices).unwrap()),
            PolygonMode::Line => Some(glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::LinesList, &self.indices).unwrap()),
            PolygonMode::Triangle => Some(glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap()),
        };
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        use glium::Surface;

        let vertex_buffer = match self.vertex_buffer {
            Some(ref vertex_buffer) => vertex_buffer,
            None => return,
        };
        let index_buffer = match self.index_buffer {
            Some(ref index_buffer) => index_buffer,
            None => return,
        };

        let projection = {
            let (width, height) = frame.get_dimensions();
            let r: f32 = width as f32 / 2.0;
            let t: f32 = height as f32 / 2.0;
            let n: f32 = 128.0;
            let f: f32 = -128.0;
            [
                [1.0f32 / r, 0.0, 0.0, 0.0],
                [0.0, 1.0 / t, 0.0, 0.0],
                [0.0, 0.0, - 2.0 / (f - n), - (f + n) / (f - n)],
                [-1.0, 1.0, 0.0, 1.0],
                /*[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],*/
            ]
        };

        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                //test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            point_size: Some(3.0),
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        frame.draw(vertex_buffer, index_buffer, &self.program,
                   &uniform! {
                       projection: projection,
                       matrix: matrix
                   },
                   &params).unwrap();
    }
}

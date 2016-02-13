use glium;
use super::vertex::ColorVertex;

pub struct ColorTriangleBatch<'a> {
    display: &'a glium::backend::glutin_backend::GlutinFacade,
    perspective: bool,
    program: glium::Program,
    vertices: Vec<ColorVertex>,
    indices: Vec<u32>,
    vertex_buffer: Option<glium::VertexBuffer<ColorVertex>>,
    index_buffer: Option<glium::IndexBuffer<u32>>,
}

impl<'a> ColorTriangleBatch<'a> {
    pub fn new(display: &'a glium::backend::glutin_backend::GlutinFacade, perspective: bool) -> Self {
        let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;
            out vec3 v_position;

            uniform mat4 projection;
            uniform mat4 matrix;

            void main() {
                v_normal = transpose(inverse(mat3(matrix))) * normal;
                gl_Position = projection * matrix * vec4(position, 1.0);
                v_position = gl_Position.xyz / gl_Position.w;
            }
        "#;

        let fragment_shader_src = r#"
            #version 150

            in vec3 v_normal;
            in vec3 v_position;

            out vec4 color;

            uniform vec3 u_light;
            //uniform sampler2D tex;

            const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
            const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
            const vec3 specular_color = vec3(1.0, 1.0, 1.0);

            void main() {
                float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

                vec3 camera_dir = normalize(-v_position);
                vec3 half_direction = normalize(normalize(u_light) + camera_dir);
                float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

                color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);


                /*float brightness = dot(normalize(v_normal), normalize(u_light));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);*/
                //color = vec4(1.0, 0, 0, 1.0);
            }
        "#;

        ColorTriangleBatch {
            display: display,
            perspective: perspective,
            program: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: None,
            index_buffer: None
        }
    }

    pub fn add_color_triangles(&mut self, vertices: &[ColorVertex], indices: &[u32]) {
        let index_offset: u32 = self.vertices.len() as u32;
        for v in vertices {
            self.vertices.push(*v);
            //println!("{} {} {}", v.position[0], v.position[1], v.position[2]);
        }
        for i in indices {
            self.indices.push(*i + index_offset);
        }
    }

    pub fn create_buffers(&mut self) {
        self.vertex_buffer = Some(glium::VertexBuffer::new(self.display, &self.vertices).unwrap());
        self.index_buffer = Some(glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap());
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

        let projection = if self.perspective {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        } else {
            let (width, height) = frame.get_dimensions();
            let r: f32 = width as f32 / 2.0;
            let t: f32 = height as f32 / 2.0;
            let n: f32 = 128.0;
            let f: f32 = -128.0;
            [
                [1.0f32 / r, 0.0, 0.0, 0.0],
                [0.0, 1.0 / t, 0.0, 0.0],
                [0.0, 0.0, - 2.0 / (f - n), - (f + n) / (f - n)],
                [0.0, 0.0, 0.0, 1.0],
                /*[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],*/
            ]
        };

        let matrix =  if self.perspective {
            [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 2.0, 1.0f32]
            ]
        } else {
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        /*let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];*/

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        frame.draw(vertex_buffer, index_buffer, &self.program,
                   &uniform! { projection: projection, matrix: matrix, u_light: light },
                   &params).unwrap();
    }
}

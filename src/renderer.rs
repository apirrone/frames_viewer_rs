use gl::types::*;
use nalgebra as na;
use std::ffi::CString;
use std::mem;
use std::ptr;

use crate::camera::Camera;

pub struct Renderer {
    program: GLuint,
    frame_vao: GLuint,
    frame_vbo: GLuint,
    grid_vao: GLuint,
    grid_vbo: GLuint,
    camera: Camera,
    uniform_locations: UniformLocations,
}

struct UniformLocations {
    model: GLint,
    view: GLint,
    projection: GLint,
}

const VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 position;
    layout (location = 1) in vec4 color;
    
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    
    out vec4 fragColor;
    
    void main() {
        gl_Position = projection * view * model * vec4(position, 1.0);
        fragColor = color;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec4 fragColor;
    out vec4 FragColor;
    
    void main() {
        FragColor = fragColor;
    }
"#;

const GRID_SIZE: f32 = 1.0; // 1 meter
const GRID_STEP: f32 = 0.1; // 10 centimeters
const GRID_LINES: i32 = (GRID_SIZE / GRID_STEP) as i32;
const VERTICES_PER_GRID: i32 = (GRID_LINES + 1) * 4; // 2 lines (horizontal/vertical) * 2 vertices per line
const TOTAL_GRID_VERTICES: i32 = VERTICES_PER_GRID * 3; // 3 grids (XY, XZ, YZ)

impl Renderer {
    pub fn new() -> Self {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::LINE_SMOOTH);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::LineWidth(1.0);
            gl::DepthFunc(gl::LEQUAL);
            
            // Create and compile shaders
            let vertex_shader = compile_shader(VERTEX_SHADER, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
            
            // Create program
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            
            // Get uniform locations
            let model = CString::new("model").unwrap();
            let view = CString::new("view").unwrap();
            let projection = CString::new("projection").unwrap();
            
            let uniform_locations = UniformLocations {
                model: gl::GetUniformLocation(program, model.as_ptr()),
                view: gl::GetUniformLocation(program, view.as_ptr()),
                projection: gl::GetUniformLocation(program, projection.as_ptr()),
            };
            
            // Create VAO and VBO for coordinate frames
            let mut frame_vao = 0;
            let mut frame_vbo = 0;
            gl::GenVertexArrays(1, &mut frame_vao);
            gl::GenBuffers(1, &mut frame_vbo);
            
            // Vertices for coordinate axes (position and color)
            #[rustfmt::skip]
            let frame_vertices: [f32; 42] = [
                // Position           // Color (RGBA)
                0.0, 0.0, 0.0,       1.0, 0.0, 0.0, 1.0,  // X axis start
                0.1, 0.0, 0.0,       1.0, 0.0, 0.0, 1.0,  // X axis end
                0.0, 0.0, 0.0,       0.0, 1.0, 0.0, 1.0,  // Y axis start
                0.0, 0.1, 0.0,       0.0, 1.0, 0.0, 1.0,  // Y axis end
                0.0, 0.0, 0.0,       0.0, 0.0, 1.0, 1.0,  // Z axis start
                0.0, 0.0, 0.1,       0.0, 0.0, 1.0, 1.0,  // Z axis end
            ];
            
            gl::BindVertexArray(frame_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, frame_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (frame_vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                frame_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            
            setup_vertex_attributes();

            // Create VAO and VBO for grid
            let mut grid_vao = 0;
            let mut grid_vbo = 0;
            gl::GenVertexArrays(1, &mut grid_vao);
            gl::GenBuffers(1, &mut grid_vbo);

            // Generate grid vertices for all three planes
            let mut grid_vertices = Vec::new();

            // Colors for each plane's grid
            let xy_color = [0.8, 0.8, 0.8, 0.3]; // Light gray with transparency
            let xz_color = [0.8, 0.8, 0.8, 0.3];
            let yz_color = [0.8, 0.8, 0.8, 0.3];

            // XY plane grid (parallel to ground)
            for i in 0..=GRID_LINES {
                let pos = i as f32 * GRID_STEP;
                // Lines parallel to X axis
                grid_vertices.extend_from_slice(&[
                    0.0, pos, 0.0, xy_color[0], xy_color[1], xy_color[2], xy_color[3],
                    GRID_SIZE, pos, 0.0, xy_color[0], xy_color[1], xy_color[2], xy_color[3],
                ]);
                // Lines parallel to Y axis
                grid_vertices.extend_from_slice(&[
                    pos, 0.0, 0.0, xy_color[0], xy_color[1], xy_color[2], xy_color[3],
                    pos, GRID_SIZE, 0.0, xy_color[0], xy_color[1], xy_color[2], xy_color[3],
                ]);
            }

            // XZ plane grid (vertical, facing forward)
            for i in 0..=GRID_LINES {
                let pos = i as f32 * GRID_STEP;
                // Lines parallel to X axis
                grid_vertices.extend_from_slice(&[
                    0.0, 0.0, pos, xz_color[0], xz_color[1], xz_color[2], xz_color[3],
                    GRID_SIZE, 0.0, pos, xz_color[0], xz_color[1], xz_color[2], xz_color[3],
                ]);
                // Lines parallel to Z axis
                grid_vertices.extend_from_slice(&[
                    pos, 0.0, 0.0, xz_color[0], xz_color[1], xz_color[2], xz_color[3],
                    pos, 0.0, GRID_SIZE, xz_color[0], xz_color[1], xz_color[2], xz_color[3],
                ]);
            }

            // YZ plane grid (vertical, facing sideways)
            for i in 0..=GRID_LINES {
                let pos = i as f32 * GRID_STEP;
                // Lines parallel to Y axis
                grid_vertices.extend_from_slice(&[
                    0.0, 0.0, pos, yz_color[0], yz_color[1], yz_color[2], yz_color[3],
                    0.0, GRID_SIZE, pos, yz_color[0], yz_color[1], yz_color[2], yz_color[3],
                ]);
                // Lines parallel to Z axis
                grid_vertices.extend_from_slice(&[
                    0.0, pos, 0.0, yz_color[0], yz_color[1], yz_color[2], yz_color[3],
                    0.0, pos, GRID_SIZE, yz_color[0], yz_color[1], yz_color[2], yz_color[3],
                ]);
            }

            gl::BindVertexArray(grid_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, grid_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (grid_vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
                grid_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            setup_vertex_attributes();
            
            // Clean up shaders
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            
            Renderer {
                program,
                frame_vao,
                frame_vbo,
                grid_vao,
                grid_vbo,
                camera: Camera::new(800.0 / 600.0),
                uniform_locations,
            }
        }
    }
    
    pub fn render(&self, transform: &na::Matrix4<f32>) {
        unsafe {
            gl::UseProgram(self.program);

            let view = self.camera.view_matrix();
            let projection = self.camera.projection_matrix();
            
            // Draw grid first
            gl::LineWidth(1.0); // Thin lines for grid
            gl::UniformMatrix4fv(self.uniform_locations.model, 1, gl::FALSE, na::Matrix4::identity().as_ptr());
            gl::UniformMatrix4fv(self.uniform_locations.view, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(self.uniform_locations.projection, 1, gl::FALSE, projection.as_ptr());
            
            gl::BindVertexArray(self.grid_vao);
            gl::DrawArrays(gl::LINES, 0, TOTAL_GRID_VERTICES);
            
            // Draw coordinate frame with thicker lines and ensure it's on top
            gl::LineWidth(3.0);
            gl::UniformMatrix4fv(self.uniform_locations.model, 1, gl::FALSE, transform.as_ptr());
            
            gl::BindVertexArray(self.frame_vao);
            gl::DrawArrays(gl::LINES, 0, 6);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.95, 0.95, 0.95, 1.0); // Light gray background
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.set_aspect(width as f32 / height as f32);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

unsafe fn setup_vertex_attributes() {
    // Position attribute
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        7 * mem::size_of::<f32>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);
    
    // Color attribute (now with alpha)
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        7 * mem::size_of::<f32>() as GLsizei,
        (3 * mem::size_of::<f32>()) as *const _,
    );
    gl::EnableVertexAttribArray(1);
}

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        
        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            panic!("Shader compilation failed: {}", String::from_utf8_lossy(&buffer));
        }
        
        shader
    }
} 
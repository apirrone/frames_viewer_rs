mod renderer;
mod camera;
mod python;

use glutin::{
    Api, ContextBuilder, GlRequest,
};
use nalgebra as na;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use thiserror::Error;
use winit::event::{Event, WindowEvent, MouseButton, ElementState, DeviceEvent, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use winit::platform::unix::EventLoopBuilderExtUnix;
use winit::window::WindowBuilder;

use crate::renderer::Renderer;

#[derive(Error, Debug)]
pub enum ViewerError {
    #[error("Window creation failed")]
    WindowCreationError(#[from] winit::error::OsError),
    #[error("OpenGL context creation failed")]
    ContextCreationError(String),
}

pub type Result<T> = std::result::Result<T, ViewerError>;
pub type Transform = na::Matrix4<f32>;

#[derive(Clone)]
struct Frame {
    transform: Transform,
}

pub struct Viewer {
    frames: Arc<RwLock<HashMap<String, Frame>>>,
    running: Arc<RwLock<bool>>,
}

impl Viewer {
    pub fn new() -> Self {
        Viewer {
            frames: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub fn start(&self) -> Result<()> {
        let frames = self.frames.clone();
        let running = self.running.clone();
        *running.write() = true;

        thread::spawn(move || {
            let event_loop = EventLoopBuilder::new()
                .with_any_thread(true)
                .build();
                
            let window_builder = WindowBuilder::new()
                .with_title("Frames Viewer")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

            let context = ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                .with_vsync(false)
                .build_windowed(window_builder, &event_loop)
                .unwrap();

            let context = unsafe { context.make_current().unwrap() };

            gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

            let mut renderer = Renderer::new();
            
            let mut left_mouse_pressed = false;
            let mut middle_mouse_pressed = false;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(physical_size) => {
                            context.resize(physical_size);
                            renderer.resize(physical_size.width, physical_size.height);
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            match button {
                                MouseButton::Left => {
                                    left_mouse_pressed = state == ElementState::Pressed;
                                }
                                MouseButton::Middle => {
                                    middle_mouse_pressed = state == ElementState::Pressed;
                                }
                                _ => (),
                            }
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let scroll_amount = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y * 2.0,
                                MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
                            };
                            renderer.camera_mut().zoom(scroll_amount);
                        }
                        _ => (),
                    },
                    Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                        if left_mouse_pressed {
                            renderer.camera_mut().orbit(delta.0 as f32 * 0.01, delta.1 as f32 * 0.01);
                        } else if middle_mouse_pressed {
                            renderer.camera_mut().pan(-delta.0 as f32 * 0.08, delta.1 as f32 * 0.08);
                        }
                    }
                    Event::MainEventsCleared => {
                        // Clear the screen once before rendering all frames
                        renderer.clear();
                        
                        // First render all other frames
                        for frame in frames.read().values() {
                            renderer.render(&frame.transform);
                        }
                        
                        // Then render the origin frame last so it's always on top
                        renderer.render(&Transform::identity());
                        
                        context.swap_buffers().unwrap();
                    }
                    _ => (),
                }
            });
        });

        Ok(())
    }

    pub fn push_frame(&self, transform: Transform, name: &str) {
        let frame = Frame { transform };
        self.frames.write().insert(name.to_string(), frame);
    }

    pub fn clear_frames(&self) {
        self.frames.write().clear();
    }

    pub fn stop(&self) {
        *self.running.write() = false;
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_viewer() {
        let viewer = Viewer::new();
        assert_eq!(*viewer.running.read(), false);
    }

    #[test]
    fn test_push_frame() {
        let viewer = Viewer::new();
        let transform = Transform::identity();
        viewer.push_frame(transform, "test_frame");
        assert_eq!(viewer.frames.read().len(), 1);
    }
}

// Re-export for Python
pub use crate::python::*;

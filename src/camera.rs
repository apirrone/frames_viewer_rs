use nalgebra as na;

pub struct Camera {
    position: na::Point3<f32>,
    target: na::Point3<f32>,
    up: na::Vector3<f32>,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Camera {
            position: na::Point3::new(2.0, 2.0, 2.0),
            target: na::Point3::new(0.0, 0.0, 0.0),
            up: na::Vector3::new(0.0, 1.0, 0.0),
            fov: std::f32::consts::PI / 4.0,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        let right = (self.position - self.target).cross(&self.up).normalize();
        
        // Vertical rotation around the right vector
        let vertical_rotation = na::Rotation3::from_axis_angle(&na::Unit::new_normalize(right), delta_y);
        
        // Horizontal rotation around the world up vector (negative delta_x to fix direction)
        let horizontal_rotation = na::Rotation3::from_axis_angle(&na::Unit::new_normalize(self.up), -delta_x);
        
        // Apply rotations
        let current_pos = self.position - self.target;
        let rotated_pos = horizontal_rotation * vertical_rotation * current_pos;
        self.position = self.target + rotated_pos;
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let view_dir = (self.target - self.position).normalize();
        let right = view_dir.cross(&self.up).normalize();
        let up = right.cross(&view_dir).normalize();
        
        let movement = right * delta_x + up * delta_y;
        let scale = (self.position - self.target).magnitude() * 0.02; // Increased panning sensitivity
        
        self.position += movement * scale;
        self.target += movement * scale;
    }

    pub fn zoom(&mut self, delta: f32) {
        let view_dir = (self.target - self.position).normalize();
        let movement = view_dir * delta * 0.2;
        self.position += movement;
    }
} 
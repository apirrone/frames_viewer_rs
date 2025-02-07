use frames_viewer::{Viewer, Transform};
use std::{thread, time};
use nalgebra as na;

fn main() {
    let viewer = Viewer::new();
    viewer.start().unwrap();

    // Give the window a moment to initialize
    thread::sleep(time::Duration::from_millis(100));

    let mut frame_count = 0.0;
    
    loop {
        // Update frame positions
        frame_count += 0.016; // Increment by roughly 1/60th of a second
        
        // Frame 1: Moving along X axis with rotation around Y
        let rot1 = na::Matrix4::from_axis_angle(&na::Vector3::y_axis(), frame_count);
        let trans1 = na::Matrix4::new_translation(&na::Vector3::new(
            0.3 * (0.4 * std::f32::consts::PI * frame_count).sin(), // 30cm amplitude
            0.1,  // 10cm Y offset
            0.0
        ));
        let frame1 = trans1 * rot1;
        
        // Frame 2: Moving along Z axis with rotation around X
        let rot2 = na::Matrix4::from_axis_angle(&na::Vector3::x_axis(), frame_count * 0.5);
        let trans2 = na::Matrix4::new_translation(&na::Vector3::new(
            0.0,
            0.0,
            0.2 * (0.4 * std::f32::consts::PI * frame_count).cos() // 20cm amplitude
        ));
        let frame2 = trans2 * rot2;
        
        // Frame 3: Moving along Y axis with rotation around Z
        let rot3 = na::Matrix4::from_axis_angle(&na::Vector3::z_axis(), frame_count * 0.7);
        let trans3 = na::Matrix4::new_translation(&na::Vector3::new(
            0.0,
            0.15 * (0.4 * std::f32::consts::PI * frame_count + std::f32::consts::PI / 3.0).sin(), // 15cm amplitude
            0.0
        ));
        let frame3 = trans3 * rot3;
            
        // Clear previous frames and push new ones
        viewer.clear_frames();
        viewer.push_frame(frame1, "frame1");
        viewer.push_frame(frame2, "frame2");
        viewer.push_frame(frame3, "frame3");
        
        thread::sleep(time::Duration::from_millis(16)); // ~60 FPS

        // Print to confirm the animation is running
        println!("Frames updated: x = {}, y = {}, z = {}", 
                frame1[(0, 3)], frame3[(1, 3)], frame2[(2, 3)]);
    }
} 
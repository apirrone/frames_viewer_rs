from frames_viewer import Viewer
import numpy as np
import time

def create_wave_transform(x, y, t, wave_params):
    """Create a transform matrix with wave-like motion."""
    # Base position in the grid
    base_x = x * 0.2  # 20cm spacing
    base_y = y * 0.2
    base_z = 0.0

    # Add wave motion
    z_offset = wave_params['amplitude'] * np.sin(
        wave_params['frequency'] * t + 
        np.sqrt(x*x + y*y) * wave_params['spatial_frequency']
    )
    
    # Add some rotation
    rot_x = np.sin(t * wave_params['rot_speed'] + x * 0.1) * wave_params['rot_amplitude']
    rot_y = np.cos(t * wave_params['rot_speed'] + y * 0.1) * wave_params['rot_amplitude']
    rot_z = np.sin(t * wave_params['rot_speed'] + (x+y) * 0.1) * wave_params['rot_amplitude']

    # Create transformation matrix
    transform = np.eye(4, dtype=np.float32)
    
    # Apply rotations
    cx, sx = np.cos(rot_x), np.sin(rot_x)
    cy, sy = np.cos(rot_y), np.sin(rot_y)
    cz, sz = np.cos(rot_z), np.sin(rot_z)
    
    # Rotation matrix
    transform[0:3, 0:3] = np.array([
        [cy*cz, -cy*sz, sy],
        [sx*sy*cz + cx*sz, -sx*sy*sz + cx*cz, -sx*cy],
        [-cx*sy*cz + sx*sz, cx*sy*sz + sx*cz, cx*cy]
    ], dtype=np.float32)
    
    # Translation
    transform[0:3, 3] = [base_x, base_y, base_z + z_offset]
    
    return transform

def main():
    # Initialize viewer
    viewer = Viewer()
    viewer.start()
    
    # Give the window time to initialize
    time.sleep(0.1)
    
    # Grid dimensions (32x32 = 1024 frames)
    grid_size = 32
    
    # Animation parameters
    wave_params = {
        'amplitude': 0.1,        # 10cm wave height
        'frequency': 2.0,        # Wave speed
        'spatial_frequency': 0.3, # Wave spread
        'rot_speed': 1.0,        # Rotation speed
        'rot_amplitude': 0.2     # Rotation amount
    }
    
    print(f"Animating {grid_size*grid_size} frames...")
    
    try:
        start_time = time.time()
        frame_count = 0
        fps_update_interval = 100  # Update FPS every 100 frames
        
        while True:
            t = time.time() - start_time
            
            # Update all frames
            for x in range(grid_size):
                for y in range(grid_size):
                    transform = create_wave_transform(x - grid_size/2, y - grid_size/2, t, wave_params)
                    viewer.push_frame(transform, f"frame_{x}_{y}")
            
            frame_count += 1
            if frame_count % fps_update_interval == 0:
                elapsed = time.time() - start_time
                fps = frame_count / elapsed
                print(f"FPS: {fps:.2f} ({grid_size*grid_size} frames)")
            
            # Small sleep to prevent maxing out CPU
            time.sleep(0.001)
            
    except KeyboardInterrupt:
        print("\nStopping animation...")
        viewer.stop()
        print(f"Final statistics:")
        print(f"Total frames rendered: {frame_count}")
        print(f"Average FPS: {frame_count/(time.time()-start_time):.2f}")

if __name__ == "__main__":
    main() 
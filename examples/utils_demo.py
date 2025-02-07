from frames_viewer import Viewer
from frames_viewer.utils import make_pose, rotate_in_self, translate_absolute, rotate_about
import numpy as np
import time

def main():
    # Create and start viewer
    viewer = Viewer()
    viewer.start()
    
    # Give the window time to initialize
    time.sleep(0.1)
    
    try:
        # Create a frame at an offset
        base_frame = make_pose([0.3, 0.2, 0.1], [0, 0, 0])
        viewer.push_frame(base_frame, "base")
        
        t = 0
        while True:
            # Create a rotating frame
            rotating_frame = make_pose([0.5, 0, 0], [0, 0, t * 90])  # Rotating around Z
            viewer.push_frame(rotating_frame, "rotating")
            
            # Create a frame that rotates around the base frame
            orbiting = rotate_about(
                frame=make_pose([0.2, 0, 0], [0, 0, 0]),  # Small offset in X
                rotation=[0, t * 90, 0],  # Rotating around Y
                center=[0.3, 0.2, 0.1],  # Same as base_frame position
            )
            viewer.push_frame(orbiting, "orbiting")
            
            # Create a frame that moves back and forth
            oscillating = translate_absolute(
                frame=make_pose([0, 0, 0], [45, 0, 45]),  # 45° rotation around X and Z
                translation=[0.3 + 0.2 * np.sin(t * 2), 0.4, 0.2]  # Oscillating in X
            )
            viewer.push_frame(oscillating, "oscillating")
            
            # Create a frame that rotates in its own coordinate system
            self_rotating = rotate_in_self(
                frame=make_pose([0, 0.5, 0], [0, 0, 0]),  # Offset in Y
                rotation=[t * 90, t * 45, 0],  # Rotating around X and Y
            )
            viewer.push_frame(self_rotating, "self_rotating")
            
            time.sleep(0.016)  # ~60 FPS
            t += 0.016
            
    except KeyboardInterrupt:
        print("\nStopping demo...")
        viewer.stop()

if __name__ == "__main__":
    main() 
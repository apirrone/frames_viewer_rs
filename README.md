# frames_viewer_rs

A fast OpenGL-based 6D frames viewer with Python bindings.

## Installation

Requirements:
- Python 3.7 or later
- Rust toolchain
- OpenGL development libraries

```bash
# Install OpenGL development libraries (Ubuntu/Debian)
sudo apt-get install libgl1-mesa-dev

# Install the package
pip install -e .
```

## Usage

```python
from frames_viewer import Viewer
import numpy as np
import time

# Create and start the viewer
fv = Viewer()
fv.start()  # pops the viewer window in a separate thread

# Create a transformation matrix (4x4)
frame1 = np.eye(4, dtype=np.float32)
frame1[:3, 3] = [0.0, 0.1, 0.1]  # meters

# Animate the frame
while True:
    # Update frame position
    frame1[:3, 3][0] = 0.1 * np.sin(2 * np.pi * 1.0 * time.time())
    
    # Push the frame to the viewer
    fv.push_frame(frame1, "frame1")
    
    time.sleep(0.01)
```

## Features

- Real-time visualization of multiple coordinate frames
- Metric units (meters)
- Smooth animations
- Interactive camera controls:
  - Left mouse button: Orbit
  - Middle mouse button: Pan
  - Mouse wheel: Zoom
- Grid visualization in XY, XZ, and YZ planes with 10cm spacing

## License

MIT 
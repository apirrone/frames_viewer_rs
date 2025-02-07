# frames_viewer_rs

A fast OpenGL-based 6D frames viewer with Python bindings.

## Requirements

### System Dependencies
- Python >= 3.7
- Rust toolchain (install via [rustup](https://rustup.rs/))
- OpenGL development libraries
```bash
# Ubuntu/Debian
sudo apt-get install libgl1-mesa-dev

# Fedora
sudo dnf install mesa-libGL-devel

# Arch Linux
sudo pacman -S mesa
```

### Python Dependencies
- numpy
- setuptools >= 64
- setuptools-rust >= 1.5.2
- wheel

## Installation

```bash
# Create and activate a virtual environment (optional but recommended)
python -m venv venv
source venv/bin/activate  # or `vf new frames_viewer_rs` if using virtualfish

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
frame1[:3, 3] = [0.0, 0.1, 0.1]  # position in meters

try:
    while True:
        # Animate the frame
        frame1[:3, 3][0] = 0.1 * np.sin(2 * np.pi * 1.0 * time.time())
        
        # Push the frame to the viewer
        fv.push_frame(frame1, "frame1")
        
        time.sleep(0.01)
except KeyboardInterrupt:
    fv.stop()
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
- Color-coded axes (Red: X, Green: Y, Blue: Z)

## Development

To contribute to the project:

```bash
# Clone the repository
git clone https://github.com/apirrone/frames_viewer_rs.git
cd frames_viewer_rs

# Create a virtual environment
python -m venv venv
source venv/bin/activate  # or use virtualfish

# Install in development mode
pip install -e .
```

## License

MIT 
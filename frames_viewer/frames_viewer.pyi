from typing import Optional
import numpy as np
import numpy.typing as npt

class Viewer:
    """A real-time 6D frames viewer with OpenGL rendering.

    This viewer allows visualization of multiple coordinate frames in 3D space.
    Each frame is represented by its axes (Red: X, Green: Y, Blue: Z) and can be
    animated in real-time.

    Features:
        - Real-time visualization of multiple coordinate frames
        - Metric units (meters)
        - Interactive camera controls (orbit, pan, zoom)
        - Grid visualization in XY, XZ, and YZ planes
    """

    def __init__(self) -> None:
        """Initialize a new frames viewer."""
        ...

    def start(self) -> None:
        """Start the viewer in a separate thread.
        
        This opens a new window showing the 3D visualization.
        The window remains responsive while frames are being updated.
        
        Raises:
            RuntimeError: If the viewer fails to start
        """
        ...

    def push_frame(self, transform: npt.NDArray[np.float32], name: str) -> None:
        """Push a new frame or update an existing frame in the viewer.

        Args:
            transform: A 4x4 homogeneous transformation matrix (float32)
            name: Unique identifier for the frame
        
        Raises:
            ValueError: If transform is not a 4x4 matrix
        """
        ...

    def clear_frames(self) -> None:
        """Remove all frames from the viewer."""
        ...

    def stop(self) -> None:
        """Stop the viewer and close the window."""
        ... 
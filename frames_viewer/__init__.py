"""
frames_viewer - A fast OpenGL-based 6D frames viewer with Python bindings.

This package provides a real-time viewer for visualizing and animating 
multiple coordinate frames in 3D space.

Features:
    - Real-time visualization of multiple coordinate frames
    - Metric units (meters)
    - Interactive camera controls (orbit, pan, zoom)
    - Grid visualization in XY, XZ, and YZ planes
    - Color-coded axes (Red: X, Green: Y, Blue: Z)
    - Utilities for frame transformations and manipulations
"""

from .frames_viewer import Viewer

from . import utils

__all__ = ['Viewer', 'utils']
__version__ = '0.1.0' 
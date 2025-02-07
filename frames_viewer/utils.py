"""Utility functions for frame transformations and manipulations.

This module provides a set of functions to create and manipulate transformation matrices,
including operations like rotation, translation, and axis swapping.
"""

import numpy as np
from scipy.spatial.transform import Rotation as R
from typing import Union, List, Tuple

def make_pose(
    translation: Union[List[float], np.ndarray],
    xyz: Union[List[float], np.ndarray],
    degrees: bool = True
) -> np.ndarray:
    """Create a 6D pose matrix from translation and Euler angles.

    Args:
        translation: [x, y, z] translation vector
        xyz: [roll, pitch, yaw] Euler angles in XYZ convention
        degrees: If True, angles are in degrees; if False, in radians

    Returns:
        np.ndarray: 4x4 homogeneous transformation matrix

    Example:
        >>> pose = make_pose([1, 0, 0], [90, 0, 0])  # 90° rotation around X axis, 1m translation in X
    """
    translation = np.asarray(translation, dtype=np.float32)
    xyz = np.asarray(xyz, dtype=np.float32)
    
    if translation.shape != (3,):
        raise ValueError("Translation must be a 3D vector")
    if xyz.shape != (3,):
        raise ValueError("Rotation angles must be a 3D vector")

    pose = np.eye(4, dtype=np.float32)
    pose[:3, :3] = R.from_euler("xyz", xyz, degrees=degrees).as_matrix()
    pose[:3, 3] = translation
    return pose

def rotate_in_self(
    frame: np.ndarray,
    rotation: Union[List[float], np.ndarray],
    degrees: bool = True
) -> np.ndarray:
    """Rotate a frame around its own axes.

    Args:
        frame: 4x4 homogeneous transformation matrix
        rotation: [roll, pitch, yaw] Euler angles for rotation
        degrees: If True, angles are in degrees; if False, in radians

    Returns:
        np.ndarray: Transformed 4x4 homogeneous transformation matrix

    Example:
        >>> rotated = rotate_in_self(frame, [0, 90, 0])  # 90° rotation around frame's Y axis
    """
    frame = np.asarray(frame, dtype=np.float32)
    if frame.shape != (4, 4):
        raise ValueError("Frame must be a 4x4 transformation matrix")

    to_origin = np.eye(4, dtype=np.float32)
    to_origin[:3, :3] = frame[:3, :3]
    to_origin[:3, 3] = frame[:3, 3]
    to_origin_inv = np.linalg.inv(to_origin)

    result = to_origin_inv @ frame
    result = make_pose([0, 0, 0], rotation, degrees=degrees) @ result
    result = to_origin @ result

    return result

def rotate_about(
    frame: np.ndarray,
    rotation: Union[List[float], np.ndarray],
    center: Union[List[float], np.ndarray],
    degrees: bool = True
) -> np.ndarray:
    """Rotate a frame around a specified point.

    Args:
        frame: 4x4 homogeneous transformation matrix
        rotation: [roll, pitch, yaw] Euler angles for rotation
        center: [x, y, z] center of rotation
        degrees: If True, angles are in degrees; if False, in radians

    Returns:
        np.ndarray: Transformed 4x4 homogeneous transformation matrix

    Example:
        >>> rotated = rotate_about(frame, [0, 90, 0], [1, 0, 0])  # 90° rotation around Y at point [1,0,0]
    """
    frame = np.asarray(frame, dtype=np.float32)
    center = np.asarray(center, dtype=np.float32)
    
    if frame.shape != (4, 4):
        raise ValueError("Frame must be a 4x4 transformation matrix")
    if center.shape != (3,):
        raise ValueError("Center must be a 3D point")

    to_origin = np.eye(4, dtype=np.float32)
    to_origin[:3, :3] = frame[:3, :3]
    to_origin[:3, 3] = center
    to_origin_inv = np.linalg.inv(to_origin)

    result = to_origin_inv @ frame
    result = make_pose([0, 0, 0], rotation, degrees=degrees) @ result
    result = to_origin @ result

    return result

def translate_in_self(
    frame: np.ndarray,
    translation: Union[List[float], np.ndarray]
) -> np.ndarray:
    """Translate a frame along its own axes.

    Args:
        frame: 4x4 homogeneous transformation matrix
        translation: [x, y, z] translation vector in frame's coordinate system

    Returns:
        np.ndarray: Transformed 4x4 homogeneous transformation matrix

    Example:
        >>> translated = translate_in_self(frame, [1, 0, 0])  # 1m translation along frame's X axis
    """
    frame = np.asarray(frame, dtype=np.float32)
    translation = np.asarray(translation, dtype=np.float32)
    
    if frame.shape != (4, 4):
        raise ValueError("Frame must be a 4x4 transformation matrix")
    if translation.shape != (3,):
        raise ValueError("Translation must be a 3D vector")

    to_origin = np.eye(4, dtype=np.float32)
    to_origin[:3, :3] = frame[:3, :3]
    to_origin[:3, 3] = frame[:3, 3]
    to_origin_inv = np.linalg.inv(to_origin)

    result = to_origin_inv @ frame
    result = make_pose(translation, [0, 0, 0]) @ result
    result = to_origin @ result

    return result

def translate_absolute(
    frame: np.ndarray,
    translation: Union[List[float], np.ndarray]
) -> np.ndarray:
    """Translate a frame in world coordinates.

    Args:
        frame: 4x4 homogeneous transformation matrix
        translation: [x, y, z] translation vector in world coordinates

    Returns:
        np.ndarray: Transformed 4x4 homogeneous transformation matrix

    Example:
        >>> translated = translate_absolute(frame, [1, 0, 0])  # 1m translation along world X axis
    """
    frame = np.asarray(frame, dtype=np.float32)
    translation = np.asarray(translation, dtype=np.float32)
    
    if frame.shape != (4, 4):
        raise ValueError("Frame must be a 4x4 transformation matrix")
    if translation.shape != (3,):
        raise ValueError("Translation must be a 3D vector")

    translate = make_pose(translation, [0, 0, 0])
    return translate @ frame

def swap_axes(
    frame: np.ndarray,
    axis1: str,
    axis2: str
) -> np.ndarray:
    """Swap two axes of a frame.

    Args:
        frame: 4x4 homogeneous transformation matrix
        axis1: First axis to swap ('x', 'y', or 'z')
        axis2: Second axis to swap ('x', 'y', or 'z')

    Returns:
        np.ndarray: Transformed 4x4 homogeneous transformation matrix

    Raises:
        ValueError: If invalid axes are specified or if they are the same

    Example:
        >>> swapped = swap_axes(frame, 'x', 'y')  # Swap X and Y axes
    """
    frame = np.asarray(frame, dtype=np.float32)
    if frame.shape != (4, 4):
        raise ValueError("Frame must be a 4x4 transformation matrix")
    
    axis1 = axis1.lower()
    axis2 = axis2.lower()
    
    if axis1 not in ['x', 'y', 'z'] or axis2 not in ['x', 'y', 'z']:
        raise ValueError("Axes must be 'x', 'y', or 'z'")
    if axis1 == axis2:
        raise ValueError("Cannot swap an axis with itself")

    axes_indices = {'x': 0, 'y': 1, 'z': 2}
    idx1, idx2 = axes_indices[axis1], axes_indices[axis2]

    result = frame.copy()
    # Swap rotation columns
    result[:3, [idx1, idx2]] = result[:3, [idx2, idx1]]
    # Swap translation components
    result[0:3, 3][[idx1, idx2]] = result[0:3, 3][[idx2, idx1]]

    return result 
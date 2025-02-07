from typing import Union, List
import numpy as np
import numpy.typing as npt

def make_pose(
    translation: Union[List[float], npt.NDArray[np.float32]],
    xyz: Union[List[float], npt.NDArray[np.float32]],
    degrees: bool = True
) -> npt.NDArray[np.float32]: ...

def rotate_in_self(
    frame: npt.NDArray[np.float32],
    rotation: Union[List[float], npt.NDArray[np.float32]],
    degrees: bool = True
) -> npt.NDArray[np.float32]: ...

def rotate_about(
    frame: npt.NDArray[np.float32],
    rotation: Union[List[float], npt.NDArray[np.float32]],
    center: Union[List[float], npt.NDArray[np.float32]],
    degrees: bool = True
) -> npt.NDArray[np.float32]: ...

def translate_in_self(
    frame: npt.NDArray[np.float32],
    translation: Union[List[float], npt.NDArray[np.float32]]
) -> npt.NDArray[np.float32]: ...

def translate_absolute(
    frame: npt.NDArray[np.float32],
    translation: Union[List[float], npt.NDArray[np.float32]]
) -> npt.NDArray[np.float32]: ...

def swap_axes(
    frame: npt.NDArray[np.float32],
    axis1: str,
    axis2: str
) -> npt.NDArray[np.float32]: ... 
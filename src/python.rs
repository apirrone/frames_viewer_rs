use pyo3::prelude::*;
use numpy::{PyArray2, PyReadonlyArray2};
use nalgebra as na;

use crate::Viewer as RustViewer;
use crate::Transform;

#[pyclass(name = "Viewer")]
/// A real-time 6D frames viewer with OpenGL rendering.
///
/// This viewer allows visualization of multiple coordinate frames in 3D space.
/// Each frame is represented by its axes (Red: X, Green: Y, Blue: Z) and can be
/// animated in real-time.
///
/// Features:
///     - Real-time visualization of multiple coordinate frames
///     - Metric units (meters)
///     - Interactive camera controls (orbit, pan, zoom)
///     - Grid visualization in XY, XZ, and YZ planes
struct PyViewer {
    viewer: RustViewer,
}

#[pymethods]
impl PyViewer {
    #[new]
    /// Initialize a new frames viewer.
    ///
    /// Returns:
    ///     Viewer: A new instance of the frames viewer.
    fn new() -> Self {
        PyViewer {
            viewer: RustViewer::new(),
        }
    }

    /// Start the viewer in a separate thread.
    ///
    /// This opens a new window showing the 3D visualization.
    /// The window remains responsive while frames are being updated.
    ///
    /// Returns:
    ///     None
    ///
    /// Raises:
    ///     RuntimeError: If the viewer fails to start
    fn start(&self) -> PyResult<()> {
        self.viewer.start().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to start viewer: {}", e))
        })
    }

    /// Push a new frame or update an existing frame in the viewer.
    ///
    /// Args:
    ///     transform (numpy.ndarray): A 4x4 homogeneous transformation matrix (float32)
    ///     name (str): Unique identifier for the frame
    ///
    /// Returns:
    ///     None
    ///
    /// Raises:
    ///     ValueError: If transform is not a 4x4 matrix
    fn push_frame(&self, transform: PyReadonlyArray2<f32>, name: &str) -> PyResult<()> {
        let array = transform.as_array();
        if array.shape() != [4, 4] {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Transform must be a 4x4 matrix",
            ));
        }

        let mut matrix = na::Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                matrix[(i, j)] = array[[i, j]];
            }
        }

        self.viewer.push_frame(matrix, name);
        Ok(())
    }

    /// Remove all frames from the viewer.
    ///
    /// This clears all frames currently being displayed in the viewer.
    fn clear_frames(&self) {
        self.viewer.clear_frames();
    }

    /// Stop the viewer and close the window.
    ///
    /// This stops the viewer thread and closes the visualization window.
    fn stop(&self) {
        self.viewer.stop();
    }
}

#[pymodule]
/// A fast OpenGL-based 6D frames viewer with Python bindings.
fn frames_viewer(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyViewer>()?;
    Ok(())
} 
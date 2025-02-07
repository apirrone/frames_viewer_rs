use pyo3::prelude::*;
use numpy::{PyArray2, PyReadonlyArray2};
use nalgebra as na;

use crate::Viewer as RustViewer;
use crate::Transform;

#[pyclass(name = "Viewer")]
struct PyViewer {
    viewer: RustViewer,
}

#[pymethods]
impl PyViewer {
    #[new]
    fn new() -> Self {
        PyViewer {
            viewer: RustViewer::new(),
        }
    }

    fn start(&self) -> PyResult<()> {
        self.viewer.start().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to start viewer: {}", e))
        })
    }

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

    fn clear_frames(&self) {
        self.viewer.clear_frames();
    }

    fn stop(&self) {
        self.viewer.stop();
    }
}

#[pymodule]
fn frames_viewer(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyViewer>()?;
    Ok(())
} 
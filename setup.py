from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="frames_viewer",
    version="0.1.0",
    rust_extensions=[RustExtension("frames_viewer.frames_viewer", binding=Binding.PyO3)],
    packages=["frames_viewer"],
    zip_safe=False,
    install_requires=["numpy"],
) 
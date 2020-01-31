from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="symspell_pybind",
    version="0.3.0",
    description="Fast and Accurate SpellChecker",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    keywords="Symspell Spellchecker rust python_bind pyo3",
    author="VJAYSLN",
    author_email="",
    rust_extensions=[RustExtension("symspell_pybind.symspell_pybind", binding=Binding.PyO3)],
    packages=[
            "symspell_pybind"
    ],
    zip_safe=False,
)

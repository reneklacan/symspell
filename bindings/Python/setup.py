from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="symspell_rs",
    version="0.3.0",
    description="Fast and Accurate SpellChecker",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    keywords="Symspell Spellchecker rust python_bind pyo3",
    author="VJAYSLN",
    author_email="",
    rust_extensions=[RustExtension("symspell_rs.symspell_rs", binding=Binding.PyO3)],
    packages=[
            "symspell_rs"
    ],
    zip_safe=False,
)

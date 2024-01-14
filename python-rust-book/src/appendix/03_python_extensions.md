# Python extensions

Using Rust extension in Python is not the only way to speed up Python code or if you need to integrate with C/C++ code. In this section we will look at some of them. We will also look at some of the tools that can be used.

## Cython

[Cython](https://cython.org/) is a superset of the programming language Python, which allows developers to write Python code (with optional, C-inspired syntax extensions) that yields performance comparable to that of C.

## Numba

[Numba](https://numba.pydata.org/) is an open source JIT compiler that translates a subset of Python and NumPy code into fast machine code.

## PyPy

[PyPy](https://www.pypy.org/) is a drop-in replacement for the stock Python interpreter, CPython. Whereas CPython compiles Python to intermediate bytecode that is then interpreted by a virtual machine, PyPy uses just-in-time (JIT) compilation to translate Python code into machine-native assembly language.

## ctypes

[ctypes](https://docs.python.org/3/library/ctypes.html#module-ctypes) is a foreign function library for Python. It provides C compatible data types, and allows calling functions in DLLs or shared libraries. It can be used to wrap these libraries in pure Python.

## cffi

[C Foreign Function Interface for Python](https://cffi.readthedocs.io/en/stable/). Interact with almost any C code from Python, based on C-like declarations that you can often copy-paste from header files or documentation.

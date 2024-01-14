# Workshop - Create Python API service with FastAPI using Poetry, Rust bindings for Python and Docker

![image](images/python_rust.jpg)

## What is this workshop about?

This tutorial will show you how to create a Python API service with FastAPI using Poetry and Rust bindings for Python and Docker.

Specifically, we will build a simple API service that will give the N-th digits of pi. The API service will be written in Python using FastAPI. The API service will use Rust bindings for Python to calculate the pi digits to make it blazingly fast. The API service will be packaged in a Docker container.

## What is FastAPI?

FastAPI is a modern, fast (high-performance), web framework for building APIs with Python 3.6+ based on standard Python type hints.

## What is Poetry?

Poetry is a tool for dependency management and packaging in Python. It allows you to declare the libraries your project depends on and it will manage (install/update) them for you.

## What is Rust?

Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but can guarantee memory safety by using a borrow checker to validate references. Rust achieves memory safety without garbage collection, and reference counting is optional.

## Repository Structure

```txt

├── python-rust-book    # Workshop instructions and guidance in mdbook format
├── rust-pidigits       # Rust extension
├── src                 # Python code
│   ├── pi_api       
├── .dockerignore       # Docker ignore file
├── docker-compose.yml  # Docker compose file
├── Dockerfile          # Common code shared between the API and the front-end
├── poetry.lock         # Poetry lock file
├── pyproject.toml      # Poetry project file
├── start.bash          # Bash script to start the API service
└── README.md           # Workshop instructions and guidance
```

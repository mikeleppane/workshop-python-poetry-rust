# Dockerfile

Create a new file called `Dockerfile` in the root of the project and add the following code to it:

```dockerfile
# pull official base image
FROM python:3.11-bookworm as base

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1
ENV DEBIAN_FRONTEND=noninteractive

# install system dependencies
RUN set -eux; \
    apt update; \
    apt -y upgrade; \
    apt install --no-install-recommends wget -y; \
    pip install --no-cache-dir --upgrade pip; \
    apt autoremove -y; \
    apt clean; \
    rm -rf /var/lib/apt/lists/*;


FROM base AS python-deps

ENV PATH="${PATH}:/root/.local/bin"
ENV PATH="/root/.cargo/bin:${PATH}"

# Install poetry and rust
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN set -eux; \
    apt-get update; \
    apt-get install --no-install-recommends curl gcc libc6-dev clang lld -y; \
    curl -sSL https://install.python-poetry.org | python3 -; \
    curl https://sh.rustup.rs -sSf | sh -s -- -y; \
    poetry self update


# Install python dependencies in /.venv
COPY pyproject.toml poetry.lock ./
COPY ./pidigits-rust ./pidigits-rust
RUN POETRY_VIRTUALENVS_IN_PROJECT=1 poetry install
RUN poetry run maturin build -m pidigits-rust/Cargo.toml; \
    poetry add pidigits-rust/target/wheels/*.whl; \
    poetry install


FROM base AS runtime

# set work directory
WORKDIR /app

# Copy virtual env from python-deps stage
COPY --from=python-deps /.venv /.venv
ENV PATH="/.venv/bin:$PATH"

# create the app user
RUN adduser --group --system appuser; \
    chown appuser:appuser /app

# copy project
COPY src start.bash ./

# run entrypoint
USER appuser
EXPOSE 8000
ENTRYPOINT ["bash", "start.bash"]
```

We need one last thing before we can build our Docker image. We need to create a `start.bash` file in the root of the project and add the following code to it:

```bash
#!/bin/bash
uvicorn pidigits.main:app --host 0.0.0.0 --port 8000 
```

This file will be used as the entrypoint for our Docker image. The entrypoint is the command that is executed when the container is started. 

```admonish info title="Docker Entrypoint"
You can read more about Docker's entrypoint [here](https://docs.docker.com/engine/reference/builder/#entrypoint).
```

```admonish tip title="Note"
We don't necessarily need a `start.bash` file. We could also use the following command as the entrypoint:
```

```bash
ENTRYPOINT ["uvicorn", "pidigits.main:app", "--host", "0.0.0.0", "--port", "8000"]
```

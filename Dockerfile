# pull official base image
FROM python:3.11-bookworm as base

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1
ENV DEBIAN_FRONTEND=noninteractive

# install system dependencies
RUN set -eux; \
    apt update; \
    apt -y upgrade; \
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

# Install python dependencies in /.venv and build the Rust extension
COPY pyproject.toml poetry.lock ./
COPY ./rust-pidigits ./rust-pidigits
RUN POETRY_VIRTUALENVS_IN_PROJECT=1 poetry install
RUN poetry run maturin build -m rust-pidigits/Cargo.toml; \
    poetry run pip install rust-pidigits/target/wheels/*.whl;
    
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

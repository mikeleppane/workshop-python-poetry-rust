# Prerequisites

- [Python 3.8+](https://www.python.org/)
- [Poetry](https://python-poetry.org/)
- [pyenv](https://github.com/pyenv/pyenv)
- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Visual Studio Code](https://code.visualstudio.com/)

## Install Python 3.10+

In this workshop we will use Python 3.12 but any Python 3.10+ version should work.

A good way to install Python 3.10+ is to use [pyenv](https://github.com/pyenv/pyenv)

```bash
curl https://pyenv.run | bash
```

Note: you may need to add the following to your `.bashrc` or `.zshrc`:

```bash
echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bashrc
echo 'command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bashrc
echo 'eval "$(pyenv init -)"' >> ~/.bashrc
```

To verify that pyenv is installed, run:

```bash
pyenv --version
```

## Install Poetry

[Poetry installation guide](https://python-poetry.org/docs/#installation)

```bash
curl -sSL https://install.python-poetry.org | python3 -
```

To verify that Poetry is installed, run:

```bash
poetry --version
```

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Note: you may need to add the following to your `.bashrc` or `.zshrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

To verify that Rust is installed, run:

```bash
rustc .-vV
```

## Install Docker

[Docker installation guide](https://docs.docker.com/get-docker/)

## Visual Studio Code

You can use whatever IDE you want but we're going to use Visual Studio Code as our code editor.

The following extensions are recommended:

### Python

- [Python](https://marketplace.visualstudio.com/items?itemName=ms-python.python)
- [Pylance](https://marketplace.visualstudio.com/items?itemName=ms-python.vscode-pylance)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [Black Formatter](https://marketplace.visualstudio.com/items?itemName=ms-python.black-formatter)
- [isort](https://marketplace.visualstudio.com/items?itemName=ms-python.isort)
- [mypy](https://marketplace.visualstudio.com/items?itemName=ms-python.mypy-type-checker)
- [ruff](https://marketplace.visualstudio.com/items?itemName=charliermarsh.ruff)

### Rust

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
- [Code LLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

### Docker

- [Docker](https://marketplace.visualstudio.com/items?itemName=ms-azuretools.vscode-docker)

### Other

- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
- [markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)
- [Github Copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)
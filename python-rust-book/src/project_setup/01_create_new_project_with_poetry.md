# Create a new project with Poetry

We will start by creating a new project with Poetry. We will call our project `pi-api`.

```bash
poetry new --src pi-api
```

This will create a new project called `pi-api` with the following structure:

```txt
├── pi-api
   ├── src
   │   └── pi_api
   │       └── __init__.py
   ├── tests
   │   ├── __init__.py
   ├── pyproject.toml
   └── README.md

```

```admonish info title="Poetry's new command"
You can read more about Poetry's new command [here](https://python-poetry.org/docs/cli/#new).
```

Initiate a git repository:

```bash
git init
```

Finally add `.gitignore` file to the root of the project:

```bash
touch .gitignore
```

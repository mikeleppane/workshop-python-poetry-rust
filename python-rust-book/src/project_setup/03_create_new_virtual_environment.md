# Create a new virtual environment

We will create a new virtual environment for our project with Poetry.
Poetry doesn’t create a virtual environment right away when you start a project. You can tell Poetry explicitly which Python version you want to use for it and go from there:

```bash
poetry env use 3.12.0
```

For me this will print out the following:

```bash
Creating virtualenv pi-api-MilAWA87-py3.12 in /home/mikko/.cache/pypoetry/virtualenvs
Using virtualenv: /home/mikko/.cache/pypoetry/virtualenvs/pi-api-MilAWA87-py3.12
```

To verify that the virtual environment is created, run:

```bash
poetry env list
```

It should output something like this:

```bash
pi-api-MilAWA87-py3.12 (Activated)
```

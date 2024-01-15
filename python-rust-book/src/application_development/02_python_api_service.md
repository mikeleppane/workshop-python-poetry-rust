# Implementing Python API service

First create a `main.py` file in the `src/pi_api` directory and add the following code to it:

```python
import math
from typing import Annotated

from fastapi import FastAPI, HTTPException, Query
from pydantic import BaseModel
from rust_pidigits import chudnovsky_pi

app = FastAPI()


MAX_DIGITS = math.ceil((2**32 - 1) / 4)


@app.get("/pidigits/")
async def pidigits(
    digits: Annotated[int, Query(ge=0, lt=MAX_DIGITS)],
    limit: Annotated[int | None, Query(gt=0, lt=MAX_DIGITS)] = None,
) -> str:
    try:
        if limit is not None:
            return chudnovsky_pi(digits)[: limit + 2]
        return chudnovsky_pi(digits)[: digits + 2]
    except (ValueError, OverflowError) as ex:
        raise HTTPException(
            status_code=400,
            detail=str(ex),
        )


class HealthResponse(BaseModel):
    status: str


@app.get("/healthz")
async def healthz() -> HealthResponse:
    return HealthResponse(status="ok")
```

Here we are using FastAPI to create a simple API service. We have two endpoints: `/pidigits/` and `/healthz`. The `/pidigits/` endpoint takes two query parameters: `digits` and `limit`. The `digits` parameter is used to determine the number of pi digits to return. The `limit` parameter is used to limit the number of pi digits to return. The `/healthz` endpoint is used to check the health of the API service. The `/healthz` endpoint returns a simple JSON object with the status `ok`. 

```admonish info title="Query Parameters"
You can read more about FastAPI's query parameters [here](https://fastapi.tiangolo.com/tutorial/query-params/). You can read more about FastAPI's endpoints [here](https://fastapi.tiangolo.com/tutorial/path-params/). 
```

Because this is a very simple API, we have defined endpoints in the same file as the main function. In a real-world application, you would probably define the endpoints in a separate file.

Our rust extension can be imported with `from pidigits_rust import chudnovsky_pi`. `pidigits-rust` being the name of the Rust extension and `chudnovsky_pi` being the name of the function we want to import. You can read more about PyO3's module structure [here](https://pyo3.rs/latest/module.html).

Before we can run our API service, we need to install our Rust extension. We can install the wheel file directly with pip:

```bash
pip install rust-pidigits/target/wheels/pidigits-0.1.0-cp312-cp312-manylinux_2_34_x86_64.whl
```

or we can install the wheel file with Poetry:

```bash
poetry run pip install rust-pidigits/target/wheels/pidigits-0.1.0-cp312-cp312-manylinux_2_34_x86_64.whl
```

```admonish note title="Version Number"
You may need to change the version number in the wheel file name. In this case the version number is `0.1.0`.
```

Now we are ready to run our API service! 🎉

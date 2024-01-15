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

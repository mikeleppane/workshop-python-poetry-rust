#!/bin/bash

uvicorn pi_api.main:app --host 0.0.0.0 --port 8000
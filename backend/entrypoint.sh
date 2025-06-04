#!/usr/bin/env bash

pip install uv
uv venv /venv
source /venv/bin/activate
uv pip install -r pyproject.toml

fastapi dev main.py --host 0.0.0.0

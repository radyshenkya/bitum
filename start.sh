#!/bin/bash

# Exit early on errors
set -eu

# TODO: replace it with python code
( mkdir -p files )

# Setting up python
export PYTHONUNBUFFERED=true

VIRTUALENV=./venv

if [ ! -d $VIRTUALENV ]; then
  python3 -m venv $VIRTUALENV
fi

if [ ! -f $VIRTUALENV/bin/pip ]; then
  curl --silent --show-error --retry 5 https://bootstrap.pypa.io/get-pip.py | $VIRTUALENV/bin/python
fi

$VIRTUALENV/bin/pip install -r requirements.txt

# Setting up rust
export RUSTUP_HOME=/tmp/rustup
export CARGO_HOME=/tmp/cargo
export CARGO_TARGET_DIR=/tmp/target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Setting up trunk
rustup target add wasm32-unknown-unknown
cargo install trunk

# Compiling frontend
(cd frontend/bitum-frontend/ ; trunk build)

# Running app
gunicorn --bind 0.0.0.0:8000 wsgi:app

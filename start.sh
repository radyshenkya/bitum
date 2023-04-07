#!/bin/bash

FRONTEND_URL=https://cdn.discordapp.com/attachments/745573062279168050/1093706145794900088/dist.zip

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

# downloading frontend
(cd frontend/bitum-frontend ; rm -rf dist ; wget $FRONTEND_URL ; unzip dist.zip)

# Running app
$VIRTUALENV/bin/python3 main.py
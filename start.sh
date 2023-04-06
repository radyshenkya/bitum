#!/bin/sh

# Dev server
# python3 main.py

# Production server
gunicorn --bind 0.0.0.0:8000 wsgi:app

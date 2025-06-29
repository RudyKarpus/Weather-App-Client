#!/bin/bash

trunk build --release

simple-http-server dist --port 8080 --host 0.0.0.0
